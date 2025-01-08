use oauth2::{
    basic::BasicClient, reqwest::async_http_client as http_client, AccessToken, AuthUrl,
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, RefreshToken, Scope, TokenResponse,
    TokenUrl,
};
use serde::{Deserialize, Serialize};
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
use super::{Error, Value};
use url::Url;

// Define a type alias for Result to simplify error handling
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Represents an OAuth provider with necessary configurations and clients.
#[derive(Debug, Clone)]
pub struct OAuthProvider {
    /// OAuth2 client for initializing OAuth2 operations.
    pub client: BasicClient,
    /// Scopes for which the token is being requested.
    pub scopes: Vec<Scope>,
    /// Query parameters for requests, organized by request type.
    pub params: Vec<Vec<[String; 2]>>,
    /// OpenID Connect URL for accessing user information.
    pub oidc_url: Url,
    /// Mapping from provider's user field names to our field names.
    pub renames: Vec<[String; 2]>,
    /// HTTP client for sending requests.
    http_client: Client,
}

impl OAuthProvider {
    /// Generates the authorization URL with CSRF token.
    fn authorize_url(&self) -> (Url, CsrfToken) {
        let mut url = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone());

        if let Some(params) = self.params.first() {
            for [name, value] in params {
                url = url.add_extra_param(name, value);
            }
        }
        url.url()
    }

    /// Exchanges authorization code for access and refresh tokens.
    async fn code_exchange(&self, code: String) -> Result<(AccessToken, Option<RefreshToken>)> {
        let code = AuthorizationCode::new(code);
        let mut req = self.client.exchange_code(code);

        if let Some(params) = self.params.get(1) {
            for [name, value] in params {
                req = req.add_extra_param(name, value);
            }
        }
        let res = req.request_async(http_client).await?;

        Ok((res.access_token().clone(), res.refresh_token().cloned()))
    }

    /// Retrieves user information using access and refresh tokens.
    async fn user_info(
        &self,
        (access_token, refresh_token): &(AccessToken, Option<RefreshToken>),
    ) -> Result<HashMap<String, Value>> {
        let client = &self.http_client;
        let url = self.oidc_url.as_str();
        let token = access_token.secret();
        let mut builder = client.get(url).bearer_auth(token);

        if let Some(query) = self.params.get(3) {
            builder = builder.query(query);
        }

        if refresh_token.is_none() {
            let mut map = builder.send().await?.json().await?;
            self.rename(&mut map);
            return Ok(map);
        }

        let refresh_token = refresh_token.as_ref().unwrap();
        let res = builder.send().await?;
        let mut map = match res.status() {
            StatusCode::UNAUTHORIZED => {
                let mut builder = self.client.exchange_refresh_token(refresh_token);
                if let Some(params) = self.params.get(2) {
                    for [name, value] in params {
                        builder = builder.add_extra_param(name, value);
                    }
                }
                let token = builder
                    .request_async(http_client)
                    .await?
                    .access_token()
                    .clone();
                let token = token.secret();
                client
                    .get(url)
                    .bearer_auth(token)
                    .send()
                    .await?
                    .json()
                    .await?
            }
            _ => res.json().await?,
        };
        self.rename(&mut map);
        Ok(map)
    }

    /// Renames fields in the user information map according to the renames configuration.
    fn rename(&self, map: &mut HashMap<String, Value>) {
        for [key, rename] in &self.renames {
            if let Some(value) = map.remove(key) {
                map.insert(rename.to_string(), value);
            }
        }
    }
}

impl TryFrom<HashMap<String, Value>> for OAuthProvider {
    type Error = Box<dyn std::error::Error>;

    /// Attempts to create an OAuthProvider from a HashMap of configuration values.
    fn try_from(mut map: HashMap<String, Value>) -> std::result::Result<Self, Self::Error> {
        let client_id = map.remove("").ok_or("err")?;
        let client_id = ClientId::new(
            map.remove("client_id")
                .ok_or("field client_id not found")?
                .try_into()
                .map_err(|err| "expected a string for field client_id")?,
        );
        let client_secret = Some(ClientSecret::new(
            map.remove("client_secret")
                .ok_or("field client_secret not found")?
                .try_into()
                .map_err(|err| "expected a string for field client_secret")?,
        ));
        let auth_url = AuthUrl::from_url(
            map.remove("auth_url")
                .ok_or("field auth_url not found")?
                .try_into()
                .map_err(|err| "expected a string for field auth_url")?,
        );
        let token_url = Some(TokenUrl::from_url(
            map.remove("token_url")
                .ok_or("field token_url not found")?
                .try_into()
                .map_err(|err| "expected a string for field token_url")?,
        ));
        let client = BasicClient::new(client_id, client_secret, auth_url, token_url);
        let scopes = map
            .remove("scopes")
            .ok_or("field scopes not found")?
            .try_into()
            .map_err(|err| "expected an array of strings for field scopes")?;
        let params = Vec::<Vec<[String; 2]>>::try_from(
            map.remove("params")
                .ok_or("field params not found")?,
        )
        .map_err(|err| "expected an array of (String, String) pairs for field params")?
        .into_iter()
        .collect();
        let oidc_url = map
            .remove("oidc_url")
            .ok_or("field oidc_url not found")?
            .try_into()
            .map_err(|err| "expected a string for field oidc_url")?;
        let renames = Vec::<[String; 2]>::try_from(
            map.remove("renames")
                .ok_or("field renames not found")?,
        )
        .map_err(|err| "expected an array of (String, String) pairs for field renames")?
        .into_iter()
        .collect();
        let http_client = Client::new();
        Ok(Self {
            client,
            scopes,
            params,
            oidc_url,
            renames,
            http_client,
        })
    }
}
