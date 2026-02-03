use std::sync::{ Arc, Mutex };

use poem::{ Error, handler, http::{ StatusCode }, web::{ Data, Json } };
use store::store::Store;
use crate::{
    request_inputs::{ SignInUserInput, SignUpUserInput },
    request_outputs::{ SignInOuput, SignUpUserOutput },
};

#[handler]
pub fn sign_in(
    Json(data): Json<SignInUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Result<Json<SignInOuput>, Error> {
    let mut locked_s = s.lock().unwrap();

    match locked_s.sign_in(data.username, data.password) {
        Ok(user_id) => {
            Ok(Json(SignInOuput {
                jwt: user_id,
            }))
        }
        Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
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
