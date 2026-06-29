use std::{collections::HashMap, sync::Mutex};

use chaum_pedersen_rust::ZKP;
use num_bigint::BigUint;

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
pub struct AuthImpl {
    // since we will be using it in async code it the userInfo could be over-ridden by other fns
    // so to avoid it in rust and have the ability to lock its thread we use mutex..
    // pub user_info: HashMap<String, UserInfo>
    // to use it safely in between threads we will use the mutex keyword
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_ids: Mutex<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct UserInfo {
    // registraion
    pub user_name: String,
    pub y1: BigUint,
    pub y2: BigUint,
    // authorization
    pub r1: BigUint,
    pub r2: BigUint,
    // verification
    pub s: BigUint,
    pub c: BigUint,
    // session id
    pub session_id: String,
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    // even tho in the proto we defined everything in Captals rust will convert and use in snake case
    async fn register(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Processing a register request");
        println!("The request data is {:?}", req);

        let request = req.into_inner();
        let user_name = request.user;

        let mut user_info = UserInfo::default();
        user_info.user_name = user_name.clone();
        user_info.y1 = BigUint::from_bytes_be(&request.y1);
        user_info.y2 = BigUint::from_bytes_be(&request.y2);

        let user_info_state = &mut self.user_info.lock().unwrap();

        user_info_state.insert(user_name, user_info);

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        req: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Processing a Create Auth Challenge request");
        println!("The req made was {:?}", req);

        let request = req.into_inner();
        let user_name = request.user;

        let mut user_info = UserInfo::default();
        user_info.user_name = user_name.clone();

        let user_info_state = &mut self.user_info.lock().unwrap();

        if let Some(user) = user_info_state.get_mut(&user_name) {
            user.r1 = BigUint::from_bytes_be(&request.r1);
            user.r2 = BigUint::from_bytes_be(&request.r2);

            let (_, _, _, q) = ZKP::get_constants();

            let c = ZKP::generate_random_below(&q);
            let auth_id = ZKP::get_random_strings(25);

            user.c = c.clone();
            user.session_id = auth_id.clone();

            let auth_id_to_user = &mut self.auth_ids.lock().unwrap();
            auth_id_to_user.insert(auth_id.clone(), user_name.to_string());

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: c.to_bytes_be(),
            }))
        } else {
            Err(Status::new(
                Code::NotFound,
                "The user doesnt exist in the data base..".to_string(),
            ))
        }
    }

    async fn verify_authentication(
        &self,
        req: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        println!("Processing a Auth Verification request");
        println!("The req made was {:?}", req);

        let request = req.into_inner();
        let auth_id = request.auth_id;

        let auth_ids = self.auth_ids.lock().unwrap();

        if let Some(user_name) = auth_ids.get(&auth_id) {
            let user_info_state = &mut self.user_info.lock().unwrap();
            let user_info = user_info_state
                .get_mut(user_name)
                .expect("username not found as there is no registered auth id");
            user_info.s = BigUint::from_bytes_be(&request.s);

            let (alpha, beta, p, q) = ZKP::get_constants();

            let zkp = ZKP {
                alpha: alpha,
                beta: beta,
                p: p,
                q: q,
            };

            if zkp.verify(
                &user_info.r1,
                &user_info.r2,
                &user_info.y1,
                &user_info.y2,
                &BigUint::from_bytes_be(&request.s),
                &user_info.c,
            ) {
                Ok(Response::new(AuthenticationAnswerResponse {
                    session_id: ZKP::get_random_strings(25),
                }))
            } else {
                Err(Status::new(
                    Code::PermissionDenied,
                    "The user denied permission since failed auth".to_string(),
                ))
            }
        } else {
            Err(Status::new(
                Code::NotFound,
                "The auth id doesnt exist".to_string(),
            ))
        }
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
