use tonic::{transport::Server, Request, Response, Status};

use post::poster_server::{Poster, PosterServer};
use post::{GetRequest, GetReply};

pub mod post {
    tonic::include_proto!("post");
}

#[derive(Debug, Default)]
pub struct MyPoster {}

#[tonic::async_trait]
impl Poster for MyPoster {
    async fn get_post(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetReply>, Status> {
        println!("Got a request: {:?}", request);

        let post = post::Post {
            id: 0,
            content: "hoge".to_string(),
            author: "fuga".to_string(),
        };

        let reply = post::GetReply {post:Some(post)};

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let poster = MyPoster::default();

    Server::builder()
        .add_service(PosterServer::new(poster))
        .serve(addr)
        .await?;

    Ok(())
}

