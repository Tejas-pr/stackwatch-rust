use poem::{get, post,handler, listener::TcpListener, web::Path, Route, Server};

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}

#[handler]
fn create_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .at("/status/:website_id", get(get_website))
    .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
      .run(app)
      .await
}