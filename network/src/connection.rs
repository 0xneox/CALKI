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

use byteorder::{BigEndian, ByteOrder};
use config;
use config::NetConfig;
use libproto::communication;
use notify::DebouncedEvent;
use protobuf::Message;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use util::RwLock;

const TIMEOUT: u64 = 15;

pub struct Connection {
    pub id_card: u32,
    pub peers_pair: Arc<RwLock<Vec<(u32, String, Option<TcpStream>)>>>,
}

impl Connection {
    pub fn new(config: &config::NetConfig) -> Self {
        let id_card = config.id_card.unwrap();
        let mut peers_pair = Vec::default();
        match config.peers.as_ref() {
            Some(peers) => {
                for peer in peers.iter() {
                    let id_card: u32 = peer.id_card.unwrap();
                    let addr = format!("{}:{}", peer.ip.clone().unwrap(), peer.port.unwrap());
                    let addr = addr.parse::<String>().unwrap();
                    peers_pair.push((id_card, addr, None));
                }
            }
            None => (),

        }

        Connection {
            id_card,
            peers_pair: Arc::new(RwLock::new(peers_pair)),
        }
    }

    pub fn is_send(id_card: u32, origin: u32, operate: communication::OperateType) -> bool {
        operate == communication::OperateType::BROADCAST || (operate == communication::OperateType::SINGLE && id_card == origin) || (operate == communication::OperateType::SUBTRACT && origin != id_card)
    }

    pub fn update(&self, config: &config::NetConfig) {
        //添加更新的配置到self
        match config.peers.as_ref() {
            Some(peers) => {
                let mut peers_addr = Vec::new();
                for peer in self.peers_pair.read().iter() {
                    peers_addr.push(peer.1.clone());
                }
                info!("peers before update {:?}", peers_addr);
                let mut config_addr = Vec::new();
                for peer in peers.iter() {
                    let id_card: u32 = peer.id_card.unwrap();
                    let addr = format!("{}:{}", peer.ip.clone().unwrap(), peer.port.unwrap());
                    config_addr.push(addr.clone());
                    if peers_addr.contains(&addr) {
                        continue;
                    }
                    peers_addr.push(addr.clone());
                    self.peers_pair.write().push((id_card, addr, None));
                }
                loop {
                    let index_opt = peers_addr.iter().position(|addr| !config_addr.contains(&addr));
                    if let Some(index) = index_opt {
                        peers_addr.remove(index);
                        self.peers_pair.write().remove(index);
                    } else {
                        break;
                    }
                }
                info!("peers after update {:?}", peers_addr);
            }
            None => {
                info!("clear all peers after update!");
                self.peers_pair.write().clear();
            }
        }
    }

    pub fn broadcast(&self, mut msg: communication::Message) {
        let origin = msg.get_origin();
        let operate = msg.get_operate();
        msg.set_origin(self.id_card);

        trace!("broadcast msg {:?} ", msg);
        let msg = msg.write_to_bytes().unwrap();
        let request_id = 0xDEADBEEF00000000 + msg.len();
        let mut encoded_request_id = [0; 8];
        BigEndian::write_u64(&mut encoded_request_id, request_id as u64);
        let mut buf = Vec::new();
        buf.extend(&encoded_request_id);
        buf.extend(msg);

        let mut peers = vec![];
        for peer in self.peers_pair.write().iter_mut() {
            if Connection::is_send(peer.0, origin, operate) {
                if let Some(ref mut stream) = peer.2 {
                    peers.push(peer.0);
                    let _ = stream.write(&buf);
                }
            }
        }

        trace!("{:?} broadcast msg to nodes {:?} {:?}", self.id_card, operate, peers);
    }
}

fn connect(con: Arc<Connection>) {
    thread::spawn(move || loop {
                      for peer in con.peers_pair.write().iter_mut() {
                          let mut need_reconnect = true;
                          let mut header = [0; 8];
                          BigEndian::write_u64(&mut header, 0xDEADBEEF00000000 as u64);
                          if let Some(ref mut stream) = peer.2 {
                              let res = stream.write(&header);
                              if res.is_ok() {
                                  need_reconnect = false;
                              }
                          }
                          if need_reconnect {
                              warn!("connect {:?}!", peer.1);
                              peer.2 = TcpStream::connect(peer.1.clone()).ok();
                          }
                      }

                      thread::sleep(Duration::from_millis(TIMEOUT * 1000));
                      trace!("after sleep retry connect!");
                  });
}

pub fn manage_connect(con: Arc<Connection>, config_path: &str, rx: Receiver<DebouncedEvent>) {
    connect(con.clone());
    let config = String::from(config_path);

    let con = con.clone();
    thread::spawn(move || loop {
                      match rx.recv() {
                          Ok(event) => {
                              match event {
                                  DebouncedEvent::Create(path_buf) |
                                  DebouncedEvent::Write(path_buf) => {
                                      if path_buf.is_file() {
                                          let file_name = path_buf.file_name().unwrap().to_str().unwrap();
                                          if file_name == config.as_str() {
                                              info!("file {} change", file_name);
                                              let config = NetConfig::new(&config.as_str());
                                              con.update(&config);
                                          }
                                      }
                                  }
                                  _ => trace!("file notify event: {:?}", event),
                              }
                          }
                          Err(e) => warn!("watch error: {:?}", e),
                      }
                  });
}


#[cfg(test)]
mod test {
    use super::Connection;
    use libproto::communication;
    #[test]
    fn is_send_mag() {
        assert!(Connection::is_send(0, 0, communication::OperateType::BROADCAST));
        assert!(Connection::is_send(0, 1, communication::OperateType::BROADCAST));

        assert!(Connection::is_send(0, 0, communication::OperateType::SINGLE));
        assert!(!Connection::is_send(0, 1, communication::OperateType::SINGLE));

        assert!(!Connection::is_send(0, 0, communication::OperateType::SUBTRACT));
        assert!(Connection::is_send(0, 1, communication::OperateType::SUBTRACT));
    }
}
