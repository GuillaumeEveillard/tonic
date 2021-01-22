use hello_world_json::json_greeter_client::JsonGreeterClient;
use hello_world_json::HelloRequest;

pub mod hello_world_json {
    tonic::include_proto!("helloworld_json");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = JsonGreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.method1(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
