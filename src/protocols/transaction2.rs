/**
Forked-off from https://github.com/AndrewWestberg/cncli/ on 2020-11-30
© 2020 Andrew Westberg licensed under Apache-2.0

Re-licensed under GPLv3 or LGPLv3
© 2020 PERLUR Group

SPDX-License-Identifier: GPL-3.0-only OR LGPL-3.0-only

*/

use byteorder::WriteBytesExt;
use log::{debug, error, warn};
use serde_cbor::{de, Value};

use crate::{Agency, Protocol};

#[derive(Debug)]
pub enum State {
    Hello,
    Idle,
    TxIdsBlocking,
    TxIdsNonBlocking,
    Txs,
    Done,
}

pub struct TxSubmission2Protocol {
    pub(crate) state: State,
    pub(crate) result: Option<Result<String, String>>,
}

impl Default for TxSubmission2Protocol {
    fn default() -> Self {
        TxSubmission2Protocol { state: State::Hello, result: None }
    }
}

impl TxSubmission2Protocol {
    fn msg_reply_tx_ids(&self) -> Vec<u8> {
        // We need to do manual cbor encoding to do the empty indefinite array for txs.
        // We always just tell the server we have no transactions to send it.
        let mut message: Vec<u8> = Vec::new();
        message.write_u8(0x82).unwrap(); // array of length 2
        message.write_u8(0x01).unwrap(); // message id for ReplyTxIds is 1
        message.write_u8(0x9f).unwrap(); // indefinite array start
        message.write_u8(0xff).unwrap(); // indefinite array end
        return message;
    }
}

impl Protocol for TxSubmission2Protocol {
    fn protocol_id(&self) -> u16 {
        return 0x0004u16;
    }

    fn result(&self) -> Result<String, String> {
        self.result.clone().unwrap()
    }

    fn role(&self) -> Agency {
        Agency::Client
    }

    fn agency(&self) -> Agency {
        return match self.state {
            State::Idle => { Agency::Server }
            State::TxIdsBlocking => { Agency::Client }
            State::TxIdsNonBlocking => { Agency::Client }
            State::Done => {
                if self.result.is_none() { Agency::Client } else { Agency::None }
            }
            State::Hello => { Agency::None }
            State::Txs => { Agency::None }			// TODO: change to the correct Agency
        };
    }

    fn state(&self) -> String {
        format!("{:?}", self.state)
    }

    fn send_data(&mut self) -> Option<Vec<u8>> {
        return match self.state {
            State::Idle => {
                debug!("TxSubmission2Protocol::State::Idle");
                None
            }
            State::TxIdsBlocking => {
                debug!("TxSubmission2Protocol::State::TxIdsBlocking");
                // Server will wait on us forever. Just move to Done state.
                self.state = State::Done;
                None
            }
            State::TxIdsNonBlocking => {
                debug!("TxSubmission2Protocol::State::TxIdsNonBlocking");
                // Tell the server that we have no transactions to send them
                let payload = self.msg_reply_tx_ids();
                self.state = State::Idle;
                Some(payload)
            }
            State::Txs => { 
				debug!("TxSubmission2Protocol::State::Txs");
				None 
			}
            State::Done => {
                warn!("TxSubmission2Protocol::State::Done");
                self.result = Option::Some(Ok(String::from("Done")));
                None
            }
            State::Hello => { 
				debug!("TxSubmission2Protocol::State::Hello");
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
                            //msgRequestTxIds = [0, tsBlocking, txCount, txCount]
                            //msgReplyTxIds   = [1, [ *txIdAndSize] ]
                            //msgRequestTxs   = [2, tsIdList ]
                            //msgReplyTxs     = [3, tsIdList ]
                            //tsMsgDone       = [4]
                            //msgReplyKTnxBye = [5]
                            //msgHello        = [6]
                            0 => {
                                debug!("TxSubmission2Protocol received MsgRequestTxIds");
                                let is_blocking = cbor_array[1] == Value::Bool(true);
                                self.state = if is_blocking {
                                    State::TxIdsBlocking
                                } else {
                                    State::TxIdsNonBlocking
                                }
                            },
                            6 => {
                                debug!("TxSubmission2Protocol received MsgHello");
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
}
