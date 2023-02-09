use checkssl::CheckSSL;
use log::{error, info};
use reqwest;
use std::time::Instant;

#[tokio::main]
async fn main() {
    /*
     * SETUP
     */
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    /*
     * HTTP
     */
    let url = std::env::args().nth(1).expect("no pattern given");
    info!("Url: {:?}", url);
    let start = Instant::now();
    let result = reqwest::get(url).await;
    let elapsed = start.elapsed();
    info!("Response time: {:.2?}", elapsed);
    let domain = result.as_ref().unwrap().url().host().unwrap().to_string();
    info!("Domain name: {:?}", domain);
    let status = result.as_ref().unwrap().status().to_string();
    info!("Status: {:?}", status);

    /*
     * Check SSL
     */
    match CheckSSL::from_domain(&domain) {
        Ok(certificate) => {
            info!("Is valid: {:?}", certificate.server.is_valid);
            info!("Expiration date: {:?}", certificate.server.not_after);
            info!(
                "Time to expiration: {:?}",
                certificate.server.time_to_expiration
            );
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}
