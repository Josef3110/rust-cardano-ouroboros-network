/**

SPDX-License-Identifier: GPL-3.0-only OR LGPL-3.0-only

*/


/*use std::{
    time::{Duration, Instant},
    io,
    ops::Sub,
};*/

use log::{debug, error, warn};
use serde_cbor::{de, Deserializer, Value};

use crate::{
    Agency,
    Protocol,
    BlockStore,
    BlockHeader,
};

use blake2b_simd::Params;


#[derive(Debug)]
pub enum State {
    Idle,
    Busy,
    Streaming,
    Done,
}

pub struct BlockFetchProtocol {
    pub(crate) state: State,
    pub(crate) result: Option<Result<String, String>>,
    pub store: Option<Box<dyn BlockStore>>,
    pub pending_blocks: Vec<BlockHeader>,
	pub received_request: bool,
	pub sent_request: bool,
}

impl Default for BlockFetchProtocol {
    fn default() -> Self {
        BlockFetchProtocol { 
			state: State::Idle, 
			result: None, 
			store: None,
			pending_blocks: Vec::new(),
			received_request: false, 
			sent_request: false }
    }
}

impl BlockFetchProtocol {
/*    fn save_block(&mut self, parse_msg_block: &BlockHeader) -> io::Result<()> {		// TODO: this is just a copy from chainsync!
        match self.store.as_mut() {
            Some(store) => {
                self.pending_blocks.push((*parse_msg_block).clone());
            }
            None => {}
        }
        Ok(())
    }*/

}

impl Protocol for BlockFetchProtocol {
    fn protocol_id(&self) -> u16 {
        return 0x0003u16;
    }

    fn result(&self) -> Result<String, String> {
        self.result.clone().unwrap()
    }

    fn role(&self) -> Agency {
        Agency::Client
    }

    fn agency(&self) -> Agency {
        let agency = match self.state {
            State::Idle => { Agency::Client }
            State::Busy => { Agency::Server }
            State::Streaming => { Agency::Server }
            State::Done => { Agency::None }
            };
        return agency;
        }

    fn state(&self) -> String {
        format!("{:?}", self.state)
    }

    fn send_data(&mut self) -> Option<Vec<u8>> {
        return match self.state {
            State::Idle => {
                debug!("BlockFetchProtocol::State::Idle");
                None
            },
            State::Busy => {
                debug!("BlockFetchProtocol::State::Busy");
				if self.received_request {  // TODO: send MsgNoBlocks message to indicate that we have nothing to share 
				}
				else if self.sent_request {  // TODO: send MsgStartBatch to start getting blocks	
				}
				None
            },
            State::Streaming => {
                debug!("BlockFetchProtocol::State::Streaming");
//                Some(payload)
				None				// TODO define payload
            },
            State::Done => {
                warn!("BlockFetchProtocol::State::Done");
                self.result = Option::Some(Ok(String::from("Done")));
                None
            }
        };
    }

    fn receive_data(&mut self, data: Vec<u8>) {
        let cbor_iter = Deserializer::from_slice(&data[..]).into_iter::<Value>();

        for cbor_result in cbor_iter {
            match cbor_result {
                Ok(cbor_value) => {
                    match cbor_value {
                        Value::Array(cbor_array) => {
                            match cbor_array[0] {
                                Value::Integer(message_id) => {
                        match message_id {
                            //msgRequestRange = [0 ,point ,point]
                            //msgClientDone   = [1]
                            //msgStartBatch   = [2]
                            //msgNoBlocks     = [3]
                            //msgBlock        = [4, #6.24(bytes.cborblock)]
                            //msgBatchDone    = [5]
                            0 => {
                                debug!("BlockFetchProtocol received MsgRequestRange");
                                self.state = State::Busy;
								self.received_request = true
                            }
                            1 => {
                                debug!("BlockFetchProtocol received MsgClientDone");
                                self.state = State::Done
                            }
                            2 => {
                                debug!("BlockFetchProtocol received MsgStartBatch");
                                self.state = State::Streaming
                            }
                            3 => {
                                debug!("BlockFetchProtocol received MsgNoBlocks");
                                self.state = State::Idle
                            }
                            4 => {
                                debug!("BlockFetchProtocol received MsgBlock");
                                self.state = State::Done
                            }
                            5 => {
                                debug!("BlockFetchProtocol received MsgBatchDone");
                                self.state = State::Idle
                            }
                            _ => {
                                error!("Got unexpected message_id: {}", message_id);
                            }
                       }
                                }
                                _ => {
                                    error!("Unexpected cbor!")
                                }
                            }
                        }
                        _ => {
                            error!("Unexpected cbor!")
                        }
                    }
                }
                Err(err) => { error!("cbor decode error!: {}, hex: {}", err, hex::encode(&data)) }
            }
        }
    }
}

trait UnwrapValue {
    fn integer(&self) -> i128;
    fn bytes(&self) -> Vec<u8>;
}

impl UnwrapValue for Value {
    fn integer(&self) -> i128 {
        match self {
            Value::Integer(integer_value) => { *integer_value }
            _ => { panic!("not an integer!") }
        }
    }

    fn bytes(&self) -> Vec<u8> {
        match self {
            Value::Bytes(bytes_vec) => { bytes_vec.clone() }
            _ => { panic!("not a byte array!") }
        }
    }
}

    
	pub fn parse_msg_block(cbor_array: Vec<Value>) -> Option<BlockHeader> {
    	let mut msg_block = BlockHeader {
        	block_number: 0,
	        slot_number: 0,
	        hash: vec![],
    	    prev_hash: vec![],
        	node_vkey: vec![],
	        node_vrf_vkey: vec![],
    	    eta_vrf_0: vec![],
        	eta_vrf_1: vec![],
	        leader_vrf_0: vec![],
    	    leader_vrf_1: vec![],
        	block_size: 0,
	        block_body_hash: vec![],
    	    pool_opcert: vec![],
        	unknown_0: 0,
	        unknown_1: 0,
    	    unknown_2: vec![],
        	protocol_major_version: 0,
	        protocol_minor_version: 0,
	    };

    match &cbor_array[1] {
        Value::Array(header_array) => {
            match &header_array[1] {
                Value::Bytes(wrapped_block_header_bytes) => {
                    // calculate the block hash
                    let hash = Params::new().hash_length(32).to_state().update(&*wrapped_block_header_bytes).finalize();
//                    parse_msg_block.hash = hash.as_bytes().to_owned();

                    let block_header: Value = de::from_slice(&wrapped_block_header_bytes[..]).unwrap();
                    match block_header {
                        Value::Array(block_header_array) => {
                            match &block_header_array[0] {
                                Value::Array(block_header_array_inner) => {
                                    msg_block.block_number = block_header_array_inner[0].integer() as i64;
                                    msg_block.slot_number = block_header_array_inner[1].integer() as i64;
                                    msg_block.prev_hash.append(&mut block_header_array_inner[2].bytes());
                                    msg_block.node_vkey.append(&mut block_header_array_inner[3].bytes());
                                    msg_block.node_vrf_vkey.append(&mut block_header_array_inner[4].bytes());
                                    match &block_header_array_inner[5] {
                                        Value::Array(nonce_array) => {
                                            msg_block.eta_vrf_0.append(&mut nonce_array[0].bytes());
                                            msg_block.eta_vrf_1.append(&mut nonce_array[1].bytes());
                                        }
                                        _ => {
                                            warn!("invalid cbor! code: 340");
                                            return None;
                                        }
                                    }
                                    match &block_header_array_inner[6] {
                                        Value::Array(leader_array) => {
                                            msg_block.leader_vrf_0.append(&mut leader_array[0].bytes());
                                            msg_block.leader_vrf_1.append(&mut leader_array[1].bytes());
                                        }
                                        _ => {
                                            warn!("invalid cbor! code: 341");
                                            return None;
                                        }
                                    }
                                    msg_block.block_size = block_header_array_inner[7].integer() as i64;
                                    msg_block.block_body_hash.append(&mut block_header_array_inner[8].bytes());
                                    msg_block.pool_opcert.append(&mut block_header_array_inner[9].bytes());
                                    msg_block.unknown_0 = block_header_array_inner[10].integer() as i64;
                                    msg_block.unknown_1 = block_header_array_inner[11].integer() as i64;
                                    msg_block.unknown_2.append(&mut block_header_array_inner[12].bytes());
                                    msg_block.protocol_major_version = block_header_array_inner[13].integer() as i64;
                                    msg_block.protocol_minor_version = block_header_array_inner[14].integer() as i64;
                                }
                                _ => {
                                    warn!("invalid cbor! code: 342");
                                    return None;
                                }
                            }
                        }
                        _ => {
                            warn!("invalid cbor! code: 343");
                            return None;
                        }
                    }
                }
                _ => {
                    warn!("invalid cbor! code: 344");
                    return None;
                }
            }
        }
        _ => {
            warn!("invalid cbor! code: 345");
            return None;
        }
    }

    Some(msg_block)
}

