use crate::error::Error;
use crate::issuers::IssuerMetadata;
use jsonwebtoken::jwk::JwkSet;

#[derive(Debug)]
pub struct FetchManager {
    client: reqwest::Client,
}

impl FetchManager {
    pub fn new(client: &reqwest::Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub async fn fetch(&self, iss: &IssuerMetadata) -> Result<JwkSet, Error> {
        self.client
            .get(&iss.jwks_uri)
            .send()
            .await
            .map_err(|e| Error::FetchJwkSet(iss.issuer.clone(), e))?
            .json::<JwkSet>()
            .await
            .map_err(|e| Error::FetchJwkSet(iss.issuer.clone(), e))
    }
}
