use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub client_id: String,
    pub secret_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub scopes: Vec<String>
}



impl OAuthProvider {
    
}