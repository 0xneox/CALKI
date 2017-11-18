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
#![allow(deprecated, unused_must_use, unused_mut, unused_assignments)]
#![feature(iter_rfind)]
#[macro_use]
extern crate log;
extern crate clap;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;
extern crate rustc_serialize;
extern crate libproto;
extern crate protobuf;
extern crate pubsub;
extern crate dotenv;
extern crate logger;
extern crate bytes;
extern crate notify;
extern crate util;
extern crate rand;


pub mod config;
pub mod netserver;
pub mod connection;
pub mod calkiprotocol;
pub mod synchronizer;
//pub mod sync_vec;
pub mod network;

use clap::{App, SubCommand};
use config::NetConfig;
use connection::{Connection, manage_connect};
use dotenv::dotenv;
use netserver::NetServer;
use network::NetWork;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use pubsub::start_pubsub;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use synchronizer::Synchronizer;
use util::panichandler::set_panic_handler;


fn main() {
    dotenv().ok();
    // Always print backtrace on panic.
    env::set_var("RUST_BACKTRACE", "full");

    //exit process when panic
    set_panic_handler();

    // Init logger
    logger::init();
    info!("CALKI:network");
    // init app
    // todo load config
    let matches = App::new("network")
        .version("0.1")
        .author("Cryptape")
        .about("CALKI Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .subcommand(SubCommand::with_name("test").about("does testing things"))
        .get_matches();

    let mut config_path = "config";
    if let Some(c) = matches.value_of("config") {
        info!("Value for config: {}", c);
        config_path = c;
    }

    // check for the existence of subcommands
    let is_test = matches.is_present("test");
    let config = if is_test { NetConfig::test_config() } else { NetConfig::new(config_path) };

    // init pubsub

    // split new_tx with other msg
    let (ctx_sub_tx, crx_sub_tx) = channel();
    let (ctx_pub_tx, crx_pub_tx) = channel();
    start_pubsub("network_tx", vec!["auth.tx"], ctx_sub_tx, crx_pub_tx);

    let (ctx_sub_consensus, crx_sub_consensus) = channel();
    let (ctx_pub_consensus, crx_pub_consensus) = channel();
    start_pubsub("network_consensus", vec!["consensus.msg"], ctx_sub_consensus, crx_pub_consensus);

    let (ctx_sub, crx_sub) = channel();
    let (ctx_pub, crx_pub) = channel();
    start_pubsub("network", vec!["chain.status", "chain.blk", "jsonrpc.net"], ctx_sub, crx_pub);

    let (net_work_tx, net_work_rx) = channel();
    // start server
    // This brings up our server.
    // all server recv msg directly publish to mq
    let address_str = format!("0.0.0.0:{}", config.port.unwrap());
    let address = address_str.parse::<SocketAddr>().unwrap();
    let net_server = NetServer::new(net_work_tx.clone());

    //network server listener
    thread::spawn(move || net_server.server(address));

    //connections manage to loop
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
    let _ = watcher.watch(".", RecursiveMode::NonRecursive).unwrap();

    let (sync_tx, sync_rx) = channel();
    let con = Arc::new(Connection::new(&config));
    let net_work = NetWork::new(con.clone(), ctx_pub.clone(), sync_tx, ctx_pub_tx, ctx_pub_consensus);
    manage_connect(con.clone(), config_path, rx);

    //loop deal data
    thread::spawn(move || loop {
                      if let Ok((source, data)) = net_work_rx.recv() {
                          net_work.receiver(source, data);
                      }
                  });

    //sync loop
    let mut synchronizer = Synchronizer::new(ctx_pub, con.clone());
    thread::spawn(move || loop {
                      if let Ok((source, msg)) = sync_rx.recv() {
                          synchronizer.receive(source, msg);
                      }
                  });

    //sub new tx
    let con_tx = con.clone();
    thread::spawn(move || {
        loop {
            // msg from sub  new tx
            let (key, body) = crx_sub_tx.recv().unwrap();
            trace!("from {:?}, topic = {:?}", Source::LOCAL, key);
            let (topic, mut data) = NetWork::parse_msg(&body);
            if topic == "net.tx".to_string() {
                con_tx.broadcast(data);
            }
        }
    });

    //sub consensus msg
    thread::spawn(move || {
        loop {
            // msg from sub  new tx
            let (key, body) = crx_sub_consensus.recv().unwrap();
            trace!("from {:?}, topic = {:?}", Source::LOCAL, key);
            let (topic, mut data) = NetWork::parse_msg(&body);
            if topic == "net.msg".to_string() {
                con.broadcast(data);
            }
        }
    });

    loop {
        // msg from mq need proc before broadcast
        let (key, body) = crx_sub.recv().unwrap();
        trace!("handle delivery id {:?} payload {:?}", key, body);
        net_work_tx.send((Source::LOCAL, body)).unwrap();
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Source {
    LOCAL,
    REMOTE,
}
