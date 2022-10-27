use mongodb::Client;
use mongodb::options::{ClientOptions, ResolverConfig};

pub async fn get_client() -> Client {
    // let client_uri =
        // env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client_uri = dotenv!("MONGODB_URI");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    Client::with_options(options).unwrap()
}

pub mod passwords;
pub mod users;
