/**
Forked-off from https://github.com/AndrewWestberg/cncli/ on 2020-11-30
© 2020 Andrew Westberg licensed under Apache-2.0

Re-licensed under GPLv3 or LGPLv3
© 2020 PERLUR Group

SPDX-License-Identifier: GPL-3.0-only OR LGPL-3.0-only

*/

pub mod mux;
pub mod protocols;
pub mod block;

use std::io;
use block::BlockHeader;



pub trait Protocol {
    // Each protocol has a unique hardcoded id
    fn protocol_id(&self) -> u16;

    // Each protocol can provide a result
    fn result(&self) -> Result<String, String>;

    // We have a client or server role in the protocol
    fn role(&self) -> Agency;

    // Tells us what agency state the protocol is in
    fn agency(&self) -> Agency;

    // Printable version of the state for the Protocol
    fn state(&self) -> String;

    // Fetch the next piece of data this protocol wants to send, or None if the client doesn't
    // have agency.
    fn send_data(&mut self) -> Option<Vec<u8>>;

    // Process data received from the remote server destined for this protocol
    fn receive_data(&mut self, data: Vec<u8>);
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Agency {
    // Client continues
    Client,
    // Server continues
    Server,
    // End of exchange
    None,
}

pub trait BlockStore {
    fn save_block(&mut self, pending_blocks: &mut Vec<BlockHeader>, network_magic: u32) -> io::Result<()>;
    fn load_blocks(&mut self) -> Option<Vec<(i64, Vec<u8>)>>;
}


