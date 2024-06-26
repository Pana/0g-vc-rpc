mod rpc;
mod types;
use jsonrpc_http_server::ServerBuilder;
use rpc::api::ZgVc;
use rpc::impls::RpcImpl;

fn main() {
    let mut io = jsonrpc_core::IoHandler::new();
    let rpc_impl = RpcImpl::new();
    io.extend_with(rpc_impl.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
