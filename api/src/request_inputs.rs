use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInUserInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpUserInput {
    pub username: String,
    pub password: String,
}
