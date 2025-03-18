#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not fetch issuer {0}: {1}")]
    FetchIssuer(String, reqwest::Error),
    #[error("Could not fetch JWK Set for issuer {0}: {1}")]
    FetchJwkSet(String, reqwest::Error),
    #[error("Key {0} not found for issuer {1}")]
    JwkNotFound(String, String),
    #[error("Issuer {0} not found")]
    IssuerNotFound(String),
    #[error("Could not parse JWK Set for issuer {0}: {1}")]
    ParseJwkSet(String, reqwest::Error),
    #[error("Could not parse issuer metadata {0}: {1}")]
    ParseIssuerMetadata(String, reqwest::Error),
    #[error("unknown error: {0}")]
    Unknown(String), //TODO: DO NOT PUBLISH WITH THIS!
}
