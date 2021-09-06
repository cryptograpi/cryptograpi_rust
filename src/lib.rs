mod build;

use openssl::ssl::{SslConnector, SslMethod};

fn main() {
    let mut ctx = SslConnector::builder(SslMethod::tls()).unwrap();

    #[cfg(openssl111)]
    ctx.set_ciphersuites("TLS_AES_256_GCM_SHA384:TLS_AES_128_GCM_SHA256").unwrap();

}