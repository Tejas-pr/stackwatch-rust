use poem::{ Route, Server, get, handler, listener::TcpListener, post, web::{ Json, Path } };
use store::store::Store;
use crate::{ request_inputs::CreateWebsiteInput, request_outputs::CreateWebsiteOutput };

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    let s = Store::default();
    s.create_user();

    format!("hello {name}")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let s = Store::default();
    let id = s.create_website();

    let response: CreateWebsiteOutput = CreateWebsiteOutput {
        id,
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000")).name("hello-world").run(app).await
}
