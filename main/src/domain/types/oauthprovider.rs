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
    pub fn url(&self, redirect_url: String, state: Option<String>) -> Url {
        let mut url = self.auth_url.clone();
        let response_type = "response_type=code";
        let client_id = String::from("client_id=") + &self.client_id;
        let redirect_url = String::from("redirect_uri=") + &redirect_url;
        let mut scopes = String::from("scope=");
        for (index, scope) in self.scopes.iter().enumerate() {
            if index != 0 {
                scopes.push(' ');
            }
            scopes.push_str(scope);
        }
        let mut query = String::from(response_type);
        query.push('&');
        query.push_str(&client_id);
        query.push('&');
        query.push_str(&redirect_url);
        query.push('&');
        query.push_str(&scopes);
        if let Some(state) = &state {
            let state = String::from("state=") + state;
            query.push('&');
            query.push_str(&state);
        }
        url.set_query(Some(&query));
        url
    }
}





#[cfg(test)]
mod test {
    use super::OAuthProvider;

    #[test]
    fn test_url_method() {
        let client_id = String::from("client_id");
        let secret_id = String::from("secret_id");
        let auth_url = "https://example.com/oauth/authorize".parse().unwrap();
        let token_url = "https://example.com/oauth/token".parse().unwrap();
        let scopes = vec!["read".into(), "write".into()];
        let provider = OAuthProvider{client_id, secret_id, auth_url, token_url, scopes};
        let redirect_url = "https://example.com/redirect".into();
        let state = None;
        let url = provider.url(redirect_url, state);
        let expected_url = "https://example.com/oauth/authorize?response_type=code&client_id=client_id&redirect_uri=https://example.com/redirect&scope=read%20write".parse().unwrap();
        assert_eq!(url, expected_url)
    }
}