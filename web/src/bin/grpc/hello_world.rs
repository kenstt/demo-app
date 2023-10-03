tonic::include_proto!("helloworld");    // 透過巨集引入 build產生的rs檔

use tonic::{Request, Response, Status};
use greeter_server::Greeter;            // build檔幫我們生成的trait

#[derive(Default)]
pub struct MyGreeter {}                 // 我們要實作服務的實體

// 實作proto裡的服務(service)
#[tonic::async_trait]
impl Greeter for MyGreeter {
    // 實作proto裡的程序(rpc)，遵照trait裡依proto定義的簽章
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        tracing::debug!("Got a request from {:?}", request.remote_addr());
        let reply = HelloReply {                // proto裡定義的 message
            message: format!("Hello {}!", request.into_inner().name),
        };                                      // HelloRequest和HelloReply等struct
        Ok(Response::new(reply))       // 是第一行巨集產生的，所以可以逕行使用
    }
}