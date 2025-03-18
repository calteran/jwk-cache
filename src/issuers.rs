use serde::Deserialize;
use std::collections::HashMap;

pub type IssuerUrl = String;

pub(crate) type Issuers = HashMap<IssuerUrl, IssuerMetadata>;

#[derive(Debug, Deserialize)]
pub(crate) struct IssuerMetadata {
    pub issuer: String,
    pub jwks_uri: String,
}

impl IssuerMetadata {}
