use std::sync::LazyLock;

use reqwest::Client;

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("PawKit")
        .build()
        .expect("Failed to build reqwest client")
});

pub async fn fetch_string(url: &str) -> Option<String> {
    return HTTP_CLIENT.get(url).send().await.ok()?.text().await.ok();
}

pub async fn fetch_binary(url: &str) -> Option<Vec<u8>> {
    return HTTP_CLIENT
        .get(url)
        .send()
        .await
        .ok()?
        .bytes()
        .await
        .ok()
        .map(|b| b.to_vec());
}
