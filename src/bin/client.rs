use post::poster_client::PosterClient;
use post::GetRequest;

pub mod post {
    tonic::include_proto!("post");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PosterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GetRequest {
        id: 0,
    });

    let response = client.get_post(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
