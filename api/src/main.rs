use std::sync::{ Arc, Mutex };

use poem::{ EndpointExt, Route, Server, get, listener::TcpListener, post };
use store::store::Store;

use crate::routes::{ user::{ sign_in, sign_up }, website::{ create_website, get_website } };

pub mod request_inputs;
pub mod request_outputs;
pub mod routes;

// Data(s): Data<&Arc<Mutex<Store>>> - single database connection been shared accross all endpoints since we use the mutex and arc.

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
