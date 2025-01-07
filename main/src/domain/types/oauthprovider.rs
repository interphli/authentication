use serde::{Serialize, Deserialize};
use super::Error;
use url::Url;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub client_id: String,
    pub secret_id: String,
    pub auth_url: Url,
    pub token_url: Url,
    pub scopes: Vec<String>
}



impl OAuthProvider {
    fn url(&self, redirect_url: String, state: Option<String>) -> Result<Url> {
        let mut url = self.auth_url.clone();
        let client_id = String::from("client_id=") + &self.client_id;
        let response_type = "response_type=code";
        let redirect_url = String::from("redirect_uri=") + &redirect_url;
        let mut scopes = String::from("scope=");
        for (index, scope) in self.scopes.iter().enumerate() {
            if index != 0 {
                scopes.push(' ');
            }
            scopes.push_str(scope);
        }
        let state = match state {
            Some(state) => Some(String::from("state=") + &state),
            None => None,
        };
        url.set_query(Some(&client_id));
        url.set_query(Some(response_type));
        url.set_query(Some(&redirect_url));
        url.set_query(Some(&scopes));
        if let Some(state) = state {
            url.set_query(Some(&state));
        }
        Ok(url)
    }
}