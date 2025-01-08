use oauth2::{CsrfToken, basic::BasicClient, Scope};
use super::super::super::types::Id;
use url::Url;

pub trait OAuth2 {
    fn url(&self) -> (Url, CsrfToken);
}


impl OAuth2 for (BasicClient, Vec<Scope>, Vec<(String, String)>) {
    fn url(&self) -> (Url, CsrfToken) {
        let client = &self.0;
        let mut url = client.authorize_url(CsrfToken::new_random).add_scopes(self.1.clone());
        for (name, value) in &self.2  {
            url = url.add_extra_param(name, value);
        }
        url.url()
    }
}