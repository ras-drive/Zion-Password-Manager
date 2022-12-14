use actix_governor::{GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use cookie::Key;

#[derive(Clone)]
pub struct Config {
    pub secret_key: Key,
    pub rate_limiter_config: GovernorConfig<PeerIpKeyExtractor>,
}

impl Config {
    pub fn new() -> Self {
        let secret_key = Key::generate();
        let rate_limiter_config = GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .use_headers()
            .finish()
            .unwrap();

        Self {
            secret_key,
            rate_limiter_config,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
