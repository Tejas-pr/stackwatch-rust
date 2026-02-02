use poem::{ Route, Server, get, handler, listener::TcpListener, post, web::{ Json, Path } };
use store::store::Store;
use crate::{
    request_inputs::{ CreateWebsiteInput, SignInUserInput, SignUpUserInput },
    request_outputs::{ CreateWebsiteOutput, SignOutInput, SignUpUserOutput },
};

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn sign_in(Json(data): Json<SignInUserInput>) -> Json<SignOutInput> {
    let mut s = Store::default().unwrap();

    let exists = s.sign_in(data.username, data.password).expect("DB error during sign in");

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
fn sign_up(Json(data): Json<SignUpUserInput>) -> Json<SignUpUserOutput> {
    let mut s = Store::default().unwrap();

    let id = s.sign_up(data.username, data.password).unwrap();

    let response = SignUpUserOutput {
        id,
    };

    Json(response)
}

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    let s = Store::default();
    format!("hello {name}")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut s = Store::default().unwrap();
    let website_ = s.create_website(String::from("sdkfsld-sdfksldf-sdflkm"), data.url).unwrap();

    let response: CreateWebsiteOutput = CreateWebsiteOutput {
        id: website_.id,
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/signup", post(sign_up))
        .at("/signin", post(sign_in));
    Server::new(TcpListener::bind("0.0.0.0:3000")).name("hello-world").run(app).await
}
