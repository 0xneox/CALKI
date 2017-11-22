// CALKI
// Copyright 2016-2017 Zibbit Labs.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![allow(unused_must_use, dead_code)]

use core::filters::eth_filter::EthFilter;
use core::libchain::block::Block;
use core::libchain::call_request::CallRequest;
use core::libchain::chain::Chain;
use error::ErrorCode;
use jsonrpc_types::rpctypes::{self as rpctypes, Filter as RpcFilter, Log as RpcLog, Receipt as RpcReceipt, CountOrCode, BlockNumber, BlockParamsByNumber, BlockParamsByHash, RpcBlock};
use libproto;
use libproto::{communication, factory, topics, submodules, BlockTxHashes, SyncRequest, SyncResponse, parse_msg, response, request, MsgClass};
use libproto::blockchain::{Block as ProtobufBlock, BlockWithProof};
use libproto::blockchain::{Status, RichStatus, ProofType};
use libproto::consensus::SignedProposal;
use libproto::request::Request_oneof_req as Request;
use proof::TendermintProof;
use protobuf::{Message, RepeatedField};
use protobuf::core::parse_from_bytes;
use serde_json;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use types::filter::Filter;
use types::ids::BlockId;
use util::Address;
use util::H256;

pub enum BlockFrom {
    CONSENSUS(BlockWithProof),
    SYNC(SyncResponse),
}

#[derive(Clone)]
pub struct Forward {
    write_sender: Sender<BlockFrom>,
    //读写分离.
    chain: Arc<Chain>,
    ctx_pub: Sender<(String, Vec<u8>)>,
}


impl Forward {
    pub fn new(chain: Arc<Chain>, ctx_pub: Sender<(String, Vec<u8>)>, write_sender: Sender<BlockFrom>) -> Forward {
        Forward {
            chain: chain,
            ctx_pub: ctx_pub,
            write_sender: write_sender,
        }
    }

    pub fn broad_current_status(&self) {
        let current_hash = self.chain.get_current_hash();
        let current_height = self.chain.get_current_height();
        let nodes: Vec<Address> = {
            self.chain.nodes.read().clone()
        };
        //drop(self);
        info!("broad_current_status {:?}, {:?}", current_hash, current_height);
        let mut rich_status = RichStatus::new();
        rich_status.set_hash(current_hash.0.to_vec());
        rich_status.set_height(current_height);
        let node_list = nodes.into_iter().map(|address| address.to_vec()).collect();
        rich_status.set_nodes(RepeatedField::from_vec(node_list));

        let msg = factory::create_msg(submodules::CHAIN, topics::RICH_STATUS, communication::MsgType::RICH_STATUS, rich_status.write_to_bytes().unwrap());
        trace!("chain after current height {:?}", current_height);
        self.ctx_pub.send(("chain.richstatus".to_string(), msg.write_to_bytes().unwrap())).unwrap();

        let status: Status = rich_status.into();
        let sync_msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, status.write_to_bytes().unwrap());
        trace!("add_block chain.status {:?}, {:?}", status.get_height(), status.get_hash());
        self.ctx_pub.send(("chain.status".to_string(), sync_msg.write_to_bytes().unwrap())).unwrap();
    }

    //注意: 划分函数处理流程
    pub fn dispatch_msg(&self, key: String, msg: Vec<u8>) {
        let (cmd_id, origin, content_ext) = parse_msg(msg.as_slice());
        trace!("dispatch_msg call {:?}", key);
        match content_ext {
            MsgClass::REQUEST(req) => {
                self.reply_request(req);
            }

            MsgClass::BLOCKWITHPROOF(proof_blk) => {
                //from consensus
                self.write_sender.send(BlockFrom::CONSENSUS(proof_blk));
            }

            MsgClass::SYNCREQUEST(sync_req) => {
                if libproto::cmd_id(submodules::CHAIN, topics::SYNC_BLK) == cmd_id {
                    self.reply_syn_req(sync_req, origin);
                } else {
                    warn!("sync: other content.");
                }
            }

            MsgClass::SYNCRESPONSE(sync_res) => {
                //from sync
                self.write_sender.send(BlockFrom::SYNC(sync_res));
            }

            MsgClass::BLOCKTXHASHESREQ(block_tx_hashes_req) => {
                self.deal_block_tx_req(block_tx_hashes_req);
            }

            MsgClass::MSG(content) => {
                if libproto::cmd_id(submodules::CONSENSUS, topics::NEW_PROPOSAL) == cmd_id {
                    self.deal_new_proposal(content);
                } else {
                    trace!("Receive other message content.");
                }
            }

            _ => {
                error!("error MsgClass!!!!");
            }
        }
    }

    pub fn dispatch_blocks(&self, blocks: BlockFrom) {
        match blocks {
            BlockFrom::CONSENSUS(block) => {
                self.deal_consensus_block(block)
            }

            BlockFrom::SYNC(blocks) => {
                self.deal_sync_res(blocks)
            }
        }
    }

    fn reply_request(&self, mut req: request::Request) {
        let mut response = response::Response::new();
        response.set_request_id(req.take_request_id());
        let topic = "chain.rpc".to_string();
        match req.req.unwrap() {
            // TODO: should check the result, parse it first!
            Request::block_number(_) => {
                // let sys_time = SystemTime::now();
                let height = self.chain.get_current_height();
                response.set_block_number(height);
            }

            Request::block_by_hash(rpc) => {
                //let rpc: BlockParamsByHash = serde_json::from_str(&rpc);
                match serde_json::from_str::<BlockParamsByHash>(&rpc) {
                    Ok(param) => {
                        let hash = param.hash;
                        let include_txs = param.include_txs;
                        match self.chain.block_by_hash(H256::from(hash.as_slice())) {
                            Some(block) => {
                                let rpc_block = RpcBlock::new(hash, include_txs, block.protobuf().write_to_bytes().unwrap());
                                serde_json::to_string(&rpc_block).map(|data| response.set_block(data)).map_err(|err| {
                                                                                                                   response.set_code(ErrorCode::query_error());
                                                                                                                   response.set_error_msg(format!("{:?}", err));
                                                                                                               });
                            }
                            None => {
                                response.set_none(true)
                            }
                        }
                    }
                    Err(err) => {
                        response.set_block(format!("{:?}", err));
                        response.set_code(ErrorCode::query_error());
                    }
                };
            }

            Request::block_by_height(block_height) => {
                let block_height: BlockParamsByNumber = serde_json::from_str(&block_height).expect("Invalid param");
                let include_txs = block_height.include_txs;
                match self.chain.block(block_height.block_id.into()) {
                    Some(block) => {
                        let rpc_block = RpcBlock::new(block.hash().to_vec(), include_txs, block.protobuf().write_to_bytes().unwrap());
                        serde_json::to_string(&rpc_block).map(|data| response.set_block(data)).map_err(|err| {
                                                                                                           response.set_code(ErrorCode::query_error());
                                                                                                           response.set_error_msg(format!("{:?}", err));
                                                                                                       });
                    }
                    None => {
                        response.set_none(true);
                    }
                }
            }

            Request::transaction(hash) => {
                match self.chain.full_transaction(H256::from_slice(&hash)) {
                    Some(ts) => {
                        response.set_ts(ts);
                    }
                    None => {
                        response.set_none(true);
                    }
                }
            }

            Request::transaction_receipt(hash) => {
                let tx_hash = H256::from_slice(&hash);
                let receipt = self.chain.localized_receipt(tx_hash);
                if let Some(receipt) = receipt {
                    let rpc_receipt: RpcReceipt = receipt.into();
                    let serialized = serde_json::to_string(&rpc_receipt).unwrap();
                    response.set_receipt(serialized);
                } else {
                    response.set_none(true);
                }
            }

            Request::call(call) => {
                trace!("Chainvm Call {:?}", call);
                serde_json::from_str::<BlockNumber>(&call.height)
                    .map(|block_id| {
                        let call_request = CallRequest::from(call);
                        self.chain
                            .eth_call(call_request, block_id.into())
                            .map(|ok| { response.set_call_result(ok); })
                            .map_err(|err| {
                                         response.set_code(ErrorCode::query_error());
                                         response.set_error_msg(err);
                                     })
                    })
                    .map_err(|err| {
                                 response.set_code(ErrorCode::query_error());
                                 response.set_error_msg(format!("{:?}", err));
                             });
            }

            Request::filter(encoded) => {
                trace!("filter: {:?}", encoded);
                serde_json::from_str::<RpcFilter>(&encoded)
                    .map_err(|err| {
                                 response.set_code(ErrorCode::query_error());
                                 response.set_error_msg(format!("{:?}", err));
                             })
                    .map(|rpc_filter| {
                             let filter: Filter = rpc_filter.into();
                             let logs = self.chain.get_logs(filter);
                             let rpc_logs: Vec<RpcLog> = logs.into_iter().map(|x| x.into()).collect();
                             response.set_logs(serde_json::to_string(&rpc_logs).unwrap());
                         });
            }

            Request::transaction_count(tx_count) => {
                trace!("transaction count request from jsonrpc {:?}", tx_count);
                serde_json::from_str::<CountOrCode>(&tx_count)
                    .map_err(|err| {
                                 response.set_code(ErrorCode::query_error());
                                 response.set_error_msg(format!("{:?}", err));
                             })
                    .map(|tx_count| {
                        let address = Address::from_slice(tx_count.address.as_ref());
                        match self.chain.nonce(&address, tx_count.block_id.into()) {
                            Some(nonce) => {
                                response.set_transaction_count(u64::from(nonce));
                            }
                            None => {
                                response.set_transaction_count(0);
                            }
                        };
                    });
            }

            Request::code(code_content) => {
                trace!("code request from josnrpc  {:?}", code_content);
                serde_json::from_str::<CountOrCode>(&code_content)
                    .map_err(|err| {
                                 response.set_code(ErrorCode::query_error());
                                 response.set_error_msg(format!("{:?}", err));
                             })
                    .map(|code_content| {
                        let address = Address::from_slice(code_content.address.as_ref());
                        match self.chain.code_at(&address, code_content.block_id.into()) {
                            Some(code) => {
                                match code {
                                    Some(code) => {
                                        response.set_contract_code(code);
                                    }
                                    None => {
                                        response.set_contract_code(vec![]);
                                    }
                                }
                            }
                            None => {
                                response.set_contract_code(vec![]);
                            }
                        };
                    });
            }

            Request::new_filter(new_filter) => {
                trace!("new_filter {:?}", new_filter);
                let new_filter: RpcFilter = serde_json::from_str(&new_filter).expect("Invalid param");
                trace!("new_filter {:?}", new_filter);
                response.set_filter_id(self.chain.new_filter(new_filter) as u64);
            }

            Request::new_block_filter(_) => {
                let block_filter = self.chain.new_block_filter();
                response.set_filter_id(block_filter as u64);
            }

            Request::uninstall_filter(filter_id) => {
                trace!("uninstall_filter's id is {:?}", filter_id);
                let index = rpctypes::Index(filter_id as usize);
                let b = self.chain.uninstall_filter(index);
                response.set_uninstall_filter(b);
            }

            Request::filter_changes(filter_id) => {
                trace!("filter_changes's id is {:?}", filter_id);
                let index = rpctypes::Index(filter_id as usize);
                let log = self.chain.filter_changes(index).unwrap();
                trace!("Log is: {:?}", log);
                response.set_filter_changes(serde_json::to_string(&log).unwrap());
            }

            Request::filter_logs(filter_id) => {
                trace!("filter_log's id is {:?}", filter_id);
                let index = rpctypes::Index(filter_id as usize);
                let log = self.chain.filter_logs(index).unwrap_or(vec![]);
                trace!("Log is: {:?}", log);
                response.set_filter_logs(serde_json::to_string(&log).unwrap());
            }
            _ => {
                error!("mtach error Request_oneof_req msg!!!!");
            }
        };
        let msg: communication::Message = response.into();
        self.ctx_pub.send((topic, msg.write_to_bytes().unwrap())).unwrap();
    }

    fn deal_consensus_block(&self, proof_blk: BlockWithProof) {
        let current_height = self.chain.get_current_height();
        let mut proof_blk = proof_blk;
        let block = proof_blk.take_blk();
        let proof = proof_blk.take_proof();
        let blk_height = block.get_header().get_height();
        trace!("received proof block: block_number:{:?} current_height: {:?}", blk_height, current_height);
        if blk_height == (current_height + 1) {
            {
                self.chain.block_map.write().insert(blk_height, (Some(proof), None));
            }
            if self.chain.set_block(Block::from(block), &self.ctx_pub).is_some() {
                self.chain.block_map.write().clear();
            }
            trace!("block insert {:?}", blk_height);
        }
    }

    fn reply_syn_req(&self, sync_req: SyncRequest, origin: u32) {
        let mut sync_req = sync_req;
        let heights = sync_req.take_heights();
        debug!("sync: receive sync from node {:?}, height lists = {:?}", origin, heights);

        let mut res_vec = SyncResponse::new();
        for height in heights {
            if let Some(block) = self.chain.block(BlockId::Number(height)) {
                res_vec.mut_blocks().push(block.protobuf());
                //push double
                if height == self.chain.get_current_height() {
                    let mut proof_block = ProtobufBlock::new();
                    //get current block proof
                    if let Some(proof) = self.chain.current_block_poof() {
                        proof_block.mut_header().set_proof(proof);
                        proof_block.mut_header().set_height(::std::u64::MAX);
                        res_vec.mut_blocks().push(proof_block);
                        trace!("sync: max height {:?}, chain.blk: OperateType {:?}", height, communication::OperateType::SINGLE);
                    }
                }
            }
        }

        debug!("sync: reply node = {}, response blocks len = {}", origin, res_vec.get_blocks().len());
        if res_vec.mut_blocks().len() > 0 {
            let msg = factory::create_msg_ex(submodules::CHAIN, topics::NEW_BLK, communication::MsgType::SYNC_RES, communication::OperateType::SINGLE, origin, res_vec.write_to_bytes().unwrap());
            trace!("sync: origin {:?}, chain.blk: OperateType {:?}", origin, communication::OperateType::SINGLE);
            self.ctx_pub.send(("chain.blk".to_string(), msg.write_to_bytes().unwrap())).unwrap();
        }
    }

    fn deal_sync_res(&self, sync_res: SyncResponse) {
        let mut sync_res = sync_res;
        let iter_res = sync_res.take_blocks().into_iter();
        for block in iter_res {
            debug!("sync: deal_sync_res: current height = {}", self.chain.get_current_height());
            let blk_height = block.get_header().get_height();
            // Check transaction root
            if !block.check_hash() && blk_height != ::std::u64::MAX {
                warn!("sync: transactions root isn't correct, height is {}", blk_height);
                return;
            }

            let block = Block::from(block);
            if let (value, true) = self.add_sync_block(block) {
                if let Some(block) = value {
                    //add block
                    if self.chain.set_block(block, &self.ctx_pub).is_some() {
                        debug!("sync: deal_sync_res: set_block, current height = {}", self.chain.get_current_height());
                        continue;

                    } else {
                        break;
                    }
                }

            } else {
                break;
            }
        }

        {
            let mut guard = self.chain.block_map.write();
            let new_map = guard.split_off(&self.chain.get_current_height());
            *guard = new_map;
        }
    }

    fn add_sync_block(&self, block: Block) -> (Option<Block>, bool) {
        let mut blk = None;
        let mut is_countinue = false;
        match block.proof_type() {
            Some(ProofType::Tendermint) => {
                let proof = TendermintProof::from(block.proof().clone());
                let mut proof_height = 0;
                if proof.height == ::std::usize::MAX {
                    //block height 1's proof is height MAX
                    proof_height = 0;

                } else {
                    proof_height = proof.height as u64;
                }

                debug!("sync: add_sync_block: proof_height = {}, block height = {}", proof_height, block.header().number());
                {
                    let mut blocks = self.chain.block_map.write();
                    if proof_height == self.chain.get_current_height() && (block.number() as usize) != ::std::usize::MAX {
                        debug!("sync: add_sync_block: proof == current, proof_height = {}, block height = {}", proof_height, block.number());
                        if !blocks.contains_key(&block.number()) {
                            blocks.insert(block.number(), (None, Some(block)));
                        }
                        is_countinue = true;

                    } else if proof_height == (self.chain.get_current_height() + 1) {
                        if let Some(value) = blocks.get_mut(&proof_height) {
                            //save blk proof
                            value.0 = Some(block.proof().clone());
                            mem::swap(&mut (value.1), &mut blk);
                            debug!("sync: add_sync_block:proof_height = (current_height + 1), is_countinue =  {}", is_countinue);
                        }

                        let authorities = self.chain.nodes.read().clone();
                        if proof.check(proof_height as usize, &authorities) && blk.is_some() {
                            blocks.insert(block.number(), (None, Some(block)));
                            is_countinue = true;
                            debug!("sync: add_sync_block: proof.check success, proof_height = {}, is_countinue = {}", proof_height, is_countinue);
                        } else {
                            error!("sync: proof check error!");
                        }
                    }
                }
                debug!("sync: add_sync_block: is_countinue =  {}", is_countinue);
                (blk, is_countinue)
            }

            _ => {
                (blk, is_countinue)
            }
        }
    }

    fn deal_block_tx_req(&self, block_tx_hashes_req: libproto::BlockTxHashesReq) {
        let block_height = block_tx_hashes_req.get_height();
        if let Some(tx_hashes) = self.chain.transaction_hashes(BlockId::Number(block_height)) {
            //prepare and send the block tx hashes to auth
            let mut block_tx_hashes = BlockTxHashes::new();
            block_tx_hashes.set_height(block_height);
            let mut tx_hashes_in_u8 = Vec::new();
            for tx_hash_in_h256 in tx_hashes.iter() {
                tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());
            }
            block_tx_hashes.set_tx_hashes(RepeatedField::from_slice(&tx_hashes_in_u8[..]));
            block_tx_hashes.set_block_gas_limit(self.chain.block_gas_limit.load(Ordering::SeqCst) as u64);
            block_tx_hashes.set_account_gas_limit(self.chain.account_gas_limit.read().clone().into());
            let msg = factory::create_msg(submodules::CHAIN, topics::BLOCK_TXHASHES, communication::MsgType::BLOCK_TXHASHES, block_tx_hashes.write_to_bytes().unwrap());
            self.ctx_pub.send(("chain.txhashes".to_string(), msg.write_to_bytes().unwrap())).unwrap();
            trace!("response block's tx hashes for height:{}", block_height);
        } else {
            warn!("get block's tx hashes for height:{} error", block_height);
        }

    }

    fn deal_new_proposal(&self, content: Vec<u8>) {
        info!("Receive new proposal.");
        let signed_proposal = parse_from_bytes::<SignedProposal>(&content).unwrap();
        let _ = signed_proposal.get_proposal().get_block();
    }
}
