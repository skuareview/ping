use checkssl::CheckSSL;

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    match CheckSSL::from_domain(&pattern) {
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
