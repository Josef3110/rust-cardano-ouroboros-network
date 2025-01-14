/**
Forked-off from https://github.com/AndrewWestberg/cncli/ on 2020-11-30
© 2020 Andrew Westberg licensed under Apache-2.0

Re-licensed under GPLv3 or LGPLv3
© 2020 PERLUR Group

SPDX-License-Identifier: GPL-3.0-only OR LGPL-3.0-only

*/

use log::{debug, error, info, trace, warn};
use serde_cbor::{de, Deserializer, ser, Value};

pub trait Block {
//	fn parse_block (data: Vec<u8>) -> Option<Block>;	// parse a complete block

	fn parse_block_header (data: Vec<u8>) -> Option<BlockHeader>;	// parse only the header of a block

}

/*
#[derive(Debug, Clone)]
pub struct Block {
	pub block_header: BlockHeader,
	// TODO: define rest of block content
}
*/

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub block_number: i64,
    pub slot_number: i64,
    pub hash: Vec<u8>,
    pub prev_hash: Vec<u8>,
    pub node_vkey: Vec<u8>,
    pub node_vrf_vkey: Vec<u8>,
    pub eta_vrf_0: Vec<u8>,
    pub eta_vrf_1: Vec<u8>,
    pub leader_vrf_0: Vec<u8>,
    pub leader_vrf_1: Vec<u8>,
    pub block_size: i64,
    pub block_body_hash: Vec<u8>,
    pub pool_opcert: Vec<u8>,
    pub unknown_0: i64,
    pub unknown_1: i64,
    pub unknown_2: Vec<u8>,
    pub protocol_major_version: i64,
    pub protocol_minor_version: i64,
}
