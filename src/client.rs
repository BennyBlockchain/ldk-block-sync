use bitcoin::hashes::hex::ToHex;
use lightning_block_sync::rpc::RpcClient;
use lightning_block_sync::{BlockSource, BlockHeaderData};
use lightning_block_sync::AsyncBlockSourceResult;

use bitcoin::{BlockHash, Block};
use serde_json;

pub struct Client {
    pub rpc_client: RpcClient,
}

// Implementation for providing block header & block information. This uses LDK's `call_method` implementation
// but you could use rust-bitcoin's RPC crate for example. Or any source of block information.
impl BlockSource for Client {
    fn get_header<'a> (
        &'a mut self,
        header_hash: &'a BlockHash,
        _height_hint: Option<u32>
    ) -> AsyncBlockSourceResult<'a, BlockHeaderData> {
        Box::pin(async move {
            Ok(self.rpc_client.call_method("getblockheader", &[serde_json::json!(header_hash.to_hex())]).await?)
        })
    }

    fn get_block<'a> (
        &'a mut self,
        header_hash: &'a BlockHash
    ) -> AsyncBlockSourceResult<'a, Block> {
        Box::pin(async move {
            Ok(self.rpc_client.call_method("getblock", &[serde_json::json!(header_hash.to_hex()), serde_json::json!(0)]).await?)
        })
    }

    fn get_best_block<'a>(
        &'a mut self,
    ) -> AsyncBlockSourceResult<'_, (BlockHash, Option<u32>)> {
        Box::pin(async move {
            Ok(self.rpc_client.call_method("getblockchaininfo", &[]).await?)
        })
    }
}