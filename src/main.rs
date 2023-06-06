use ipgeolocate::{Service, Locator};
use std::{env, fs, str};
use futures::future;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    let f = args.get(1)
        .expect("Please specify a file.");
    let ips = fs::read(f)
        .expect("File does not exist.");
    let ips = str::from_utf8(&ips)
        .expect("File has invalid characters.");
    let mut handles = vec![];
    for ip in ips.split("\n") {
        handles.push(tokio::spawn(scan_location(String::from(ip))));
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    future::join_all(handles).await;
}

async fn scan_location(ip: String) {
    let service = Service::IpApi;

    match Locator::get(ip.as_str(), service).await {
        Ok(ip) => println!("{}:{}/{}", ip.ip, ip.city, ip.country),
        Err(_) => (),
    };
}
