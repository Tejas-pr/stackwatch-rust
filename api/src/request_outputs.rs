use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInOuput {
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpUserOutput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteOutput {
    pub url: String
}