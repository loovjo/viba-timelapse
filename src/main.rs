use std::convert::TryInto;

mod error;
mod requestor;
mod vibaimage;
#[tokio::main]
async fn main() {
    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::client::Client::builder().build::<_, hyper::Body>(https);

    let mut r = requestor::Requestor {
        img_url: "https://vibacam.citrons.xyz/cam.jpg".try_into().unwrap(),
        emit_path: "/tmp/images".into(),

        client,
        time_between_images: std::time::Duration::from_secs(30),
        min_time_between_reqs: std::time::Duration::from_millis(100),
    };
    r.run().await.expect("epic failure");
}
