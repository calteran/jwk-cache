use crate::{IssuerUrl, Jwk};
use jsonwebtoken::jwk::JwkSet;
use scc::HashMap;

pub trait JwkStore {
    fn get(&self, kid: &str, issuer: &IssuerUrl) -> Option<Jwk>;

    fn replace(&mut self, issuer: IssuerUrl, set: JwkSet);
}

#[derive(Clone, Debug, Default)]
pub struct JwkMemoryStore {
    store: HashMap<IssuerUrl, JwkSet>,
}

impl JwkStore for JwkMemoryStore {
    fn get(&self, kid: &str, issuer: &IssuerUrl) -> Option<Jwk> {
        self.store
            .read(issuer, |_, jwk_set| jwk_set.find(kid).cloned())
            .flatten()
    }

    fn replace(&mut self, issuer: IssuerUrl, set: JwkSet) {
        self.store.upsert(issuer, set);
    }
}
