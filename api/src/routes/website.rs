use std::sync::{ Arc, Mutex };

use poem::{ handler, web::{ Data, Json, Path } };
use store::store::Store;
use crate::{
    request_inputs::{ CreateWebsiteInput },
    request_outputs::{ CreateWebsiteOutput, GetWebsiteOutput },
};

#[handler]
pub fn get_website(
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
pub fn create_website(
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
