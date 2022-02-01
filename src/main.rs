pub mod client;

use std::collections::HashMap;

use client::Client;
use lightning_block_sync::{poll::ChainPoller, rpc::RpcClient};
use lightning_block_sync::http::HttpEndpoint;
use lightning_block_sync::SpvClient;
use lightning_block_sync::init::validate_best_block_header;

struct Listener;

impl lightning::chain::Listen for Listener {
    fn block_connected(&self, block: &bitcoin::Block, height: u32) {
        println!("BLOCK CONNECTED: {}", block.block_hash())       
    }

    fn block_disconnected(&self, header: &bitcoin::BlockHeader, height: u32) {
        println!("BLOCK DISCONNECTED: {}", header.block_hash())
    }
}

// Implement tokio async
fn main() {
    let endpoint: HttpEndpoint = HttpEndpoint::for_host("192.168.1.169".to_string()).with_port(18333);

    // Correct error handling instead of `unwrap()`
    let mut client = Client {
        rpc_client: RpcClient::new("ben:bitcoin", endpoint).unwrap()
    };

    let header = validate_best_block_header(&mut  client).await.unwrap();
    let cache = HashMap::new();
    let poller = ChainPoller::new(&mut client, bitcoin::Network::Testnet);

    let listener = Listener;

    let spv_client = SpvClient::new( header, poller, &mut cache, &listener);
}
