tonic::include_proto!("helloworld");            // 透過巨集引入 build產生的rs檔

use greeter_client::GreeterClient;              // 使用proto產生的Client物件

pub async fn say_hello() {
    let mut client = GreeterClient::connect("http://[::1]:3032")
        .await
        .unwrap();                              // 連線到server端

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),                   // 準備請求內容
    });

    let response = client.say_hello(request)
        .await
        .unwrap();                              // 呼叫並取得回應

    tracing::debug!("RESPONSE={:#?}", response); // 輸出紀錄回應結果
}
