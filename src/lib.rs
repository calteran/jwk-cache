//! TODO: Crate-level documentation
//#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]

mod cache;
pub mod error;
mod fetch;
mod issuers;
mod storage;

pub use cache::JwkCache;
pub use issuers::*;
pub use jsonwebtoken::jwk::Jwk;
pub use storage::{JwkMemoryStore, JwkStore};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::JwkMemoryStore;

    #[tokio::test]
    async fn it_works() {
        JwkCache::<JwkMemoryStore>::new()
            .with_issuer("http://localhost:8080")
            .build()
            .await
            .expect("cache");
    }
}
