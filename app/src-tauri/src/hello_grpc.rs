tonic::include_proto!("helloworld");

use tonic::transport::Channel;
use greeter_client::GreeterClient;

pub async fn say_hello(
    channel: Channel, name: &str
) -> String {
    let mut client = GreeterClient::new(channel);
    let request = tonic::Request::new(HelloRequest {
        name: name.to_string(),
    });
    let response = client.say_hello(request).await.unwrap();
    tracing::debug!("RESPONSE={:#?}", response);

    response.into_inner().message
}
