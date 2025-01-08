use oauth2::{basic::BasicClient, Scope, ClientId, ClientSecret, AuthUrl, TokenUrl};
use serde::{Serialize, Deserialize};
use super::Error;
use url::Url;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: Url,
    pub token_url: Url,
    pub scopes: Vec<String>,
    pub params: Vec<(String, String)>
}


impl From<OAuthProvider> for (BasicClient, Vec<Scope>, Vec<(String, String)>) {
    fn from(provider: OAuthProvider) -> Self {
        let client_id = ClientId::new(provider.client_id);
        let client_secret = Some(ClientSecret::new(provider.client_secret));
        let auth_url = AuthUrl::from_url(provider.auth_url);
        let token_url = Some(TokenUrl::from_url(provider.token_url));
        let client = BasicClient::new(client_id, client_secret, auth_url, token_url);
        let scopes = provider.scopes.into_iter().map(|scope|Scope::new(scope)).collect();
        (client, scopes, provider.params)
    }
}