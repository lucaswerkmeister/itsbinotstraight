use reqwest::blocking::Client;
use reqwest::header;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let biword = itsbinotstraight::biword(&mut rng);
    let phrase = itsbinotstraight::phrase(&biword, &mut rng);

    toot(&phrase)
}

#[derive(Deserialize)]
struct Toot {
    url: String,
    // we don’t care about anything else
}

fn toot(text: &str) -> Result<(), Box<dyn Error>> {
    let mut authorization = header::HeaderValue::from_str(&format!(
        "Bearer {}",
        &dotenv::var("MASTODON_ACCESS_TOKEN")?
    ))?;
    authorization.set_sensitive(true);
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, authorization);

    let client = Client::builder().default_headers(headers).build()?;

    let mut payload = HashMap::new();
    payload.insert("status", text);
    payload.insert("language", "en");

    let endpoint = format!("{}/api/v1/statuses", &dotenv::var("MASTODON_HOST")?);
    let response = client.post(endpoint).json(&payload).send()?;
    let result: Toot = response.error_for_status()?.json()?;
    println!("{}", result.url);
    Ok(())
}
