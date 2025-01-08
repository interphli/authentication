use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, Scope, TokenUrl};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::{Error, Value};
use url::Url;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Debug, Clone)]
pub struct OAuthProvider {
    pub client: BasicClient,
    pub scopes: Vec<Scope>,
    pub params: Vec<(String, String)>,
    pub open_id_url: Url,
    pub renames: Vec<(String, String)>
}


impl OAuthProvider {
    fn authorize_url(&self) -> (Url, CsrfToken) {
        let mut url = self.client.authorize_url(CsrfToken::new_random).add_scopes(self.scopes.clone());
        for (name, value) in &self.params  {
            url = url.add_extra_param(name, value);
        }
        url.url()
    }

    fn rename(&self, map: &mut HashMap<String, Value>) {
        for (key, rename) in &self.renames {
            if let Some(value) = map.remove(key) {
                map.insert(rename.to_string(), value);
            }
        }
    }
}


impl TryFrom<HashMap<String, Value>> for OAuthProvider {
    type Error = Box<dyn std::error::Error>;
    fn try_from(mut map: HashMap<String, Value>) -> std::result::Result<Self, Self::Error> {
        let client_id = map.remove("").ok_or("err")?;
        let client_id = ClientId::new(map.remove("client_id").ok_or("field client_id not found")?.try_into().map_err(|err|"expected a string for field client_id")?);
        let client_secret = Some(ClientSecret::new(map.remove("client_secret").ok_or("field client_secret not found")?.try_into().map_err(|err|"expected a string for field client_secret")?));
        let auth_url = AuthUrl::from_url(map.remove("auth_url").ok_or("field auth_url not found")?.try_into().map_err(|err|"expected a string for field auth_url")?);
        let token_url = Some(TokenUrl::from_url(map.remove("token_url").ok_or("field token_url not found")?.try_into().map_err(|err|"expected a string for field token_url")?));
        let client = BasicClient::new(client_id, client_secret, auth_url, token_url);
        let scopes = map.remove("scopes").ok_or("field scopes not found")?.try_into().map_err(|err|"expected an array of strings for field scopes")?;
        let params = Vec::<[String; 2]>::try_from(map.remove("params").ok_or("field params not found")?).map_err(|err|"expected an array of (String, String) pairs for field params")?.into_iter().map(|[key, value]|(key, value)).collect();
        let open_id_url = map.remove("open_id_url").ok_or("field open_id_url not found")?.try_into().map_err(|err|"expected a string for field open_id_url")?;
        let renames = Vec::<[String; 2]>::try_from(map.remove("renames").ok_or("field renames not found")?).map_err(|err|"expected an array of (String, String) pairs for field renames")?.into_iter().map(|[key, value]|(key, value)).collect();
        Ok(Self{client, scopes, params, open_id_url, renames})
    }
}