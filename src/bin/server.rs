use r2d2::Pool;
use redis::Client;
use tonic::{transport::Server, Code, Request, Response, Status};
use ubiquitous_bassoon::post_repo::PostRepository;

use post::poster_server::{Poster, PosterServer};
use post::{GetReply, GetRequest, DeleteRequest};

pub mod post {
    tonic::include_proto!("post");
}

#[derive(Debug)]
pub struct MyPoster {
    repo: PostRepository,
}

impl MyPoster {
    fn new(repo: PostRepository) -> MyPoster {
        MyPoster { repo }
    }
}

#[tonic::async_trait]
impl Poster for MyPoster {
    async fn get_post(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        let id = request.into_inner().id;
        let post = self
            .repo
            .retrieve(id)
            .ok_or_else(|| Status::new(Code::NotFound, "ID not found"))?;
        let post = post::Post {
            id: post.get_id(),
            content: post.get_content(),
            author: post.get_author(),
        };
        let reply = post::GetReply { post: Some(post) };
        Ok(Response::new(reply))
    }

    async fn delete_post(&self, request: tonic::Request<DeleteRequest>) -> Result<tonic::Response<()>, Status> {
        let id = request.into_inner().id;
        self.repo.delete(id);
        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_host = get_redis_host();
    let client = Client::open(format!("redis://{}", redis_host)).unwrap();
    let pool = Pool::builder().build(client).unwrap();
    let repo = PostRepository::new(pool);

    let addr = "[::1]:50051".parse()?;
    let poster = MyPoster::new(repo.clone());

    Server::builder()
        .add_service(PosterServer::new(poster))
        .serve(addr)
        .await?;

    Ok(())
}

fn get_redis_host() -> String {
    std::env::var("REDIS_HOST").expect("environment variable REDIS_HOST must be defined")
}
