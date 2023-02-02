use checkssl::CheckSSL;
use reqwest;
use std::time::Instant;

#[tokio::main]
async fn main() {
    /*
     * HTTP
     */
    let url = std::env::args().nth(1).expect("no pattern given");
    println!("Url: {:?}", url);
    let start = Instant::now();
    let result = reqwest::get(url).await;
    let elapsed = start.elapsed();
    println!("Response time: {:.2?}", elapsed);
    let domain = result.as_ref().unwrap().url().host().unwrap().to_string();
    println!("Domain name: {:?}", domain);
    let status = result.as_ref().unwrap().status().to_string();
    println!("Status: {:?}", status);

    /*
     * Check SSL
     */
    match CheckSSL::from_domain(&domain) {
        Ok(certificate) => {
            println!("Is valid: {:?}", certificate.server.is_valid);
            println!("Expiration date: {:?}", certificate.server.not_after);
            println!(
                "Time to expiration: {:?}",
                certificate.server.time_to_expiration
            );
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
