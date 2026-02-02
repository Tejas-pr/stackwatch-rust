use std::sync::{ Arc, Mutex };

use poem::{
    EndpointExt,
    Route,
    Server,
    get,
    handler,
    listener::TcpListener,
    post,
    web::{ Data, Json, Path },
};
use store::store::Store;
use crate::{
    request_inputs::{ CreateWebsiteInput, SignInUserInput, SignUpUserInput },
    request_outputs::{ CreateWebsiteOutput, GetWebsiteOutput, SignOutInput, SignUpUserOutput },
};

pub mod request_inputs;
pub mod request_outputs;

// Data(s): Data<&Arc<Mutex<Store>>> - single database connection been shared accross all endpoints since we use the mutex and arc.

#[handler]
fn sign_in(
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
fn sign_up(
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

#[handler]
fn get_website(
    Path(input_id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let websites_ = locked_s.get_website(input_id).unwrap();

    Json(GetWebsiteOutput {
        url: websites_.url,
    })
}

#[handler]
fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>
) -> Json<CreateWebsiteOutput> {
    let mut locked_s = s.lock().unwrap();
    let website_ = locked_s
        .create_website(String::from("sdkfsld-sdfksldf-sdflkm"), data.url)
        .unwrap();

    let response: CreateWebsiteOutput = CreateWebsiteOutput {
        id: website_.id,
    };

    Json(response)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));

    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/signup", post(sign_up))
        .at("/signin", post(sign_in))
        .data(s); // <- here we shared the database connection across all endpoints/ threads.
    Server::new(TcpListener::bind("0.0.0.0:3000")).name("hello-world").run(app).await
}
