use crate::error::Error;
use crate::fetch::FetchManager;
use crate::issuers::{IssuerMetadata, IssuerUrl, Issuers};
use crate::storage::{JwkMemoryStore, JwkStore};
use jsonwebtoken::jwk::Jwk;
use std::sync::Arc;

/// TODO: JwkCache struct documentation
#[derive(Clone, Debug)]
pub struct JwkCache<S>
where
    S: JwkStore,
{
    issuers: Arc<Issuers>,
    fetch_manager: Arc<FetchManager>,
    store: S,
}

impl<S> JwkCache<S>
where
    S: JwkStore,
{
    /// Build a new JwkCache
    pub fn new() -> JwkCacheBuilder {
        JwkCacheBuilder::default()
    }

    /// Retrieve the Jwk for the given `kid`
    pub async fn get(&mut self, kid: &str, issuer: &IssuerUrl) -> Option<Jwk> {
        self.try_get(kid, issuer).await.ok()
    }

    /// Try to retrieve the Jwk for the given `kid` and `issuer`
    pub async fn try_get(&mut self, kid: &str, issuer: &IssuerUrl) -> Result<Jwk, Error> {
        if let Some(jwk) = self.store.get(kid, issuer) {
            return Ok(jwk.clone());
        }

        self.refresh(issuer).await?;

        if let Some(jwk) = self.store.get(kid, issuer) {
            Ok(jwk.clone())
        } else {
            Err(Error::JwkNotFound(kid.to_string(), issuer.to_string()))
        }
    }

    async fn refresh(&mut self, issuer: &IssuerUrl) -> Result<(), Error> {
        let meta = self
            .issuers
            .get(issuer)
            .ok_or(Error::IssuerNotFound(issuer.to_string()))?;
        let jwk_set = self.fetch_manager.fetch(meta).await?;
        self.store.replace(issuer.clone(), jwk_set);
        Ok(())
    }
}

/// TODO: JwkCacheBuilder struct documentation
#[derive(Debug)]
pub struct JwkCacheBuilder<S = JwkMemoryStore> {
    issuers: Vec<String>,
    client: reqwest::Client,
    store: S,
}

impl Default for JwkCacheBuilder<JwkMemoryStore> {
    fn default() -> Self {
        Self {
            issuers: Vec::new(),
            client: reqwest::ClientBuilder::new()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("Failed to build `reqwest` client"),
            store: JwkMemoryStore::default(),
        }
    }
}

impl<S> JwkCacheBuilder<S>
where
    S: JwkStore,
{
    /// Replace the current store with a new one.
    pub fn with_store<NewS>(self, store: NewS) -> JwkCacheBuilder<NewS>
    where
        NewS: JwkStore,
    {
        JwkCacheBuilder {
            issuers: self.issuers,
            client: self.client,
            store,
        }
    }

    /// Add an issuer to the cache
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuers.push(issuer.into());
        self
    }

    /// Add multiple issuers to the cache
    pub fn with_issuers(mut self, issuers: Vec<impl Into<String>>) -> Self {
        self.issuers.extend(issuers.into_iter().map(|s| s.into()));
        self
    }

    /// Build the cache.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn build(self) -> Result<JwkCache<S>, Error> {
        let cache = JwkCache {
            issuers: Arc::new(self.fetch_issuers().await?),
            fetch_manager: Arc::new(FetchManager::new(&self.client)),
            store: self.store,
        };

        Ok(cache)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    async fn fetch_issuers(&self) -> Result<Issuers, Error> {
        let mut issuers = Issuers::new();
        for url in &self.issuers {
            let config_url = format!("{}/.well-known/openid-configuration", url);
            let res = self.fetch_issuer(&config_url).await?;
            issuers.insert(url.clone(), res);
        }
        Ok(issuers)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    async fn fetch_issuer(&self, url: &IssuerUrl) -> Result<IssuerMetadata, Error> {
        self.client
            .clone()
            .get(url)
            .send()
            .await
            .map_err(|e| Error::FetchIssuer(url.to_string(), e))?
            .json::<IssuerMetadata>()
            .await
            .map_err(|e| Error::ParseIssuerMetadata(url.to_string(), e))
    }
}
