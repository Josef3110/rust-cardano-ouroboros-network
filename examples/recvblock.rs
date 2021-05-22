/**


SPDX-License-Identifier: GPL-3.0-only OR LGPL-3.0-only

*/

use cardano_ouroboros_network::{
    mux,
    protocols::blockfetch::{BlockFetchProtocol, Mode},
};
use futures::executor::block_on;

mod common;
mod sqlite;

fn main() {
    let cfg = common::init();

    block_on(async {
        let channel = mux::tcp::connect(&cfg.host, cfg.port).await.unwrap();
        channel.handshake(cfg.magic).await.unwrap();
        channel.execute({BlockFetchProtocol {
            mode: Mode::Receive,
            network_magic: cfg.magic,
            store: Some(Box::new(sqlite::SQLiteBlockStore::new(&cfg.db).unwrap())),
            ..Default::default()
        }}).await.unwrap();
    });
}