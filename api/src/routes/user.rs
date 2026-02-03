use std::sync::{ Arc, Mutex };

use poem::{ handler, web::{ Data, Json } };
use store::store::Store;
use crate::{
    request_inputs::{ SignInUserInput, SignUpUserInput },
    request_outputs::{ SignOutInput, SignUpUserOutput },
};

#[handler]
pub fn sign_in(
    Json(data): Json<SignInUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Json<SignOutInput> {
    let mut locked_s = s.lock().unwrap();
    let exists = locked_s.sign_in(data.username, data.password).expect("DB error during sign in");

    if exists {
        // TODO: generate jwt, status codes.
        Json(SignOutInput {
            jwt: Some("tejastejasteajs".to_string()),
            error: None,
        })
    } else {
        Json(SignOutInput {
            jwt: None,
            error: Some("Not authorized!!".to_string()),
        })
    }
}

#[handler]
pub fn sign_up(
    Json(data): Json<SignUpUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Json<SignUpUserOutput> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).unwrap();

    let response = SignUpUserOutput {
        id,
    };

    Json(response)
}
