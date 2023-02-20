fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/http://www.google.co.uk",
        delay_ms,
    );
    reqwest::Url::parse(&url).unwrap()
}

#[tokio::main]
async fn main() {
    info!("Starting program!");
    let _resp1 = reqwest::get(slowwly(1000)).await?;
    info!("Got response 1");
    let _resp2 = reqwest::get(slowwly(1000)).await?;
    info!("Got response 2");
    Ok(())
}
