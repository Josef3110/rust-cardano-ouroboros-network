/**

SPDX-License-Identifier: GPL-3.0-only OR LGPL-3.0-only

*/

use byteorder::WriteBytesExt;
use log::{debug, error, warn};
use serde_cbor::{de, Value};

use crate::{Agency, Protocol};

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
}

impl Default for BlockFetchProtocol {
    fn default() -> Self {
        BlockFetchProtocol { state: State::Idle, result: None }
    }
}

impl BlockFetchProtocol {
    // TODO: implement
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
        return match self.state {
            State::Idle => { Agency::Client }
            State::Busy => { Agency::Server }
            State::Streaming => { Agency::Server }
            State::Done => { Agency::None }
            }
        }
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
                None
            },
            State::Streaming => {
                debug!("BlockFetchProtocol::State::Streaming");
                Some(payload)
            },
            State::Done => {
                warn!("BlockFetchProtocol::State::Done");
                self.result = Option::Some(Ok(String::from("Done")));
                None
            }
        };
    }

    fn receive_data(&mut self, data: Vec<u8>) {
        let cbor_value: Value = de::from_slice(&data[..]).unwrap();
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
                                self.state = State::Busy
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
                                error!("unexpected message_id: {}", message_id);
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

