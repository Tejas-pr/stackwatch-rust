use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignOutInput {
    pub jwt: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpUserOutput {
    pub id: String,
}
