use color_eyre::Report;
use reqwest::Client;
use serde_json::Value as JsonValue;

#[tokio::main]
async fn main() -> Result<(), Report>{
    let client = Client::new();

    let dog_breed = dog_breed(&client).await?;
    let cat_facts = cat_facts(&client).await?;
    let http_bin = http_bin(&client).await?;

    println!("{:#}\n{:#}\n{:#}", dog_breed, cat_facts, http_bin);
    
    return Ok(());
}

async fn dog_breed(client: &Client) -> Result<JsonValue, Report> {
    const API_URL: &str = "https://dog.ceo/api/breeds/list/all";

    let response = client.get(API_URL).send().await?;

    let json_value = response.text().await?.parse::<JsonValue>()?;

    return Ok(json_value);
}
async fn cat_facts(client: &Client) -> Result<JsonValue, Report> {
    const API_URL: &str = "https://cat-fact.herokuapp.com/facts";

    let response = client.get(API_URL).send().await?;

    let json_value = response.text().await?.parse::<JsonValue>()?;

    return Ok(json_value);
}
async fn http_bin(client: &Client) -> Result<JsonValue, Report> {
    const API_URL: &str = "https://httpbin.org/get";

    let response = client.get(API_URL).send().await?;

    let json_value = response.text().await?.parse::<JsonValue>()?;

    return Ok(json_value);
}
