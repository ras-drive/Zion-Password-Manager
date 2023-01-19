use actix_governor::{
    governor::{clock::QuantaInstant, middleware::NoOpMiddleware},
    GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor,
};
use cookie::Key;

#[derive(Clone)]
pub struct Config {
    pub addr: String,
    pub secret_key: Key,
    pub rate_limiter_config: GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>,
}

impl Config {
    pub fn new() -> Self {
        let addr = option_env!("ADDR").unwrap_or("0.0.0.0").to_string();

        let secret_key = Key::generate();
        let rate_limiter_config = GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(25)
            .finish()
            .unwrap();

        Self {
            addr,
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
