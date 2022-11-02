// use dss_openapi::apis::configuration::Configuration;

extern crate dss_interface;
extern crate dss_openapi;
extern crate reqwest;

#[tokio::main]
async fn main() {
    println!("Hello, world! {}", dss_interface::add(7, 9));

    let c = dss_openapi::apis::configuration::Configuration {
        client: crate::reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
        ..dss_openapi::apis::configuration::Configuration::default()
    };
    let result = dss_openapi::apis::system_api::get_dsid(&c).await.unwrap();
    println!("Response: {:?}", result.ok.ok_or(false));
}
