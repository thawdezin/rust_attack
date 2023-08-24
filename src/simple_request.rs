use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    send_http().await?;
    Ok(())
}

async fn send_http() -> Result<(), reqwest::Error> {
    // Create a URL
    let url = "https://thawdezin.web.app";

    // Create a vector of headers
    let headers = vec![
        ("Header1", "Value1"),
        ("Header2", "Value2"),
        // Add more headers here...
    ];

    // Create a header map
    let mut header_map = HeaderMap::new();
    for (name, value) in headers {
        header_map.insert(
            name,
            HeaderValue::from_str(value).expect("Invalid header value"),
        );
    }

    // Create a client
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    // Send 10 requests with different headers
    for _ in 0..1000000000 {
        let response = client
            .get(url)
            .headers(header_map.clone())
            .send()
            .await?;

        println!("Response: {:?}", response);
        println!();
    }

    Ok(())
}
