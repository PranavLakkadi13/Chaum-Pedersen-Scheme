use tokio::net::unix::SocketAddr;
use zkp_auth::auth_server;

use crate::zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
    auth_server::{Auth, AuthServer},
};
use tonic::{Code, Request, Response, Status, transport::Server};

// since the built crate is not directly accessible using mod we need to explicitely load it
pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

#[derive(Debug, Default)]
struct AuthImpl {}

#[tonic::async_trait]
impl Auth for AuthImpl {
    // even tho in the proto we defined everything in Captals rust will convert and use in snake case
    async fn register(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        todo!()
    }

    async fn create_authentication_challenge(
        &self,
        req: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        todo!()
    }

    async fn verify_authentication(
        &self,
        req: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:50051".to_string();
    println!("Hi from the server...., running at {:?}", &addr);

    Server::builder()
        .add_service(AuthServer::new(AuthImpl::default()))
        .serve(
            addr.parse()
                .expect("Could not convert the address to Socket Addr"),
        )
        .await
        .unwrap();
}
