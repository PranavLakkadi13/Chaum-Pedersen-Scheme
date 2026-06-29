pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use std::io::stdin;

use chaum_pedersen_rust::ZKP;
use num_bigint::BigUint;
use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationChallengeRequest, RegisterRequest,
    auth_client::AuthClient,
};

#[tokio::main]
async fn main() {
    let mut client = AuthClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Couldnt connect to the client");

    println!("Hi from the client.....");

    let mut buffer = String::new();
    println!("Please provide the username = ");
    stdin()
        .read_line(&mut buffer)
        .expect("could not read username");

    let username = buffer
        .trim()
        .parse::<String>()
        .expect("Error converting the message...");

    buffer.clear();

    println!("Please provide the password = ");
    stdin()
        .read_line(&mut buffer)
        .expect("could not read username");

    let password = BigUint::from_bytes_be(buffer.trim().as_bytes());
    buffer.clear();

    let (alpha, beta, p, q) = ZKP::get_constants();

    let zkp = ZKP {
        alpha: alpha.clone(),
        beta: beta.clone(),
        p: p.clone(),
        q: q.clone(),
    };

    ////////////////////////////////////
    // Request to the server to register

    let y1 = ZKP::exponentiate(&alpha, &password, &p);
    let y2 = ZKP::exponentiate(&beta, &password, &p);
    drop(password);

    let request = RegisterRequest {
        user: username.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };

    let _response = client
        .register(request)
        .await
        .expect("error registering...");
    println!("the response is {:?}", _response);

    ////////////////////////////////////
    // Request the server for challege post registration

    let k = ZKP::generate_random_below(&q);
    let r1 = ZKP::exponentiate(&alpha, &k, &p);
    let r2 = ZKP::exponentiate(&beta, &k, &p);

    let request = AuthenticationChallengeRequest {
        user: username,
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };

    let challenge_response = client
        .create_authentication_challenge(request)
        .await
        .expect("Failed to retreive the challenge check again with registration")
        .into_inner();

    println!("the repsonse is {:?}", challenge_response);

    let auth_id = challenge_response.auth_id;
    let challenge = challenge_response.c;

    println!("Please provide the password to login = ");
    stdin()
        .read_line(&mut buffer)
        .expect("could not read username");

    let password = BigUint::from_bytes_be(buffer.trim().as_bytes());

    let s = zkp.solve(&k, &BigUint::from_bytes_be(&challenge), &password);
    drop(password);

    let request = AuthenticationAnswerRequest {
        auth_id: auth_id,
        s: s.to_bytes_be(),
    };

    let session_id = client
        .verify_authentication(request)
        .await
        .expect("Auth failed...")
        .into_inner()
        .session_id;

    println!("The sessionid  post auth = {session_id}, logged successfullyyyyyy");
}
