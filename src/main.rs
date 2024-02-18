use color_eyre::Report;
use reqwest::Client;
use serde_json::Value as JsonValue;

/// REST endpoint returning a list of dog breeds.
const DOG_API_URL: &str = "https://dog.ceo/api/breeds/list/all";

/// REST endpoint returning a cat facts.
const CAT_API_URL: &str = "https://cat-fact.herokuapp.com/facts";

/// Example REST endpoint.
const HTTP_BIN_URL: &str = "https://httpbin.org/get";

#[tokio::main]
async fn main() -> Result<(), Report> {
    // Create new client to handle HTTP/HTTPS requests
    let client = Client::new();

    let dog_breed = get_json(&client, DOG_API_URL).await?;
    let cat_facts = get_json(&client, CAT_API_URL).await?;
    let http_bin = get_json(&client, HTTP_BIN_URL).await?;

    // Display JSON values
    println!("{:#}\n{:#}\n{:#}", dog_breed, cat_facts, http_bin);

    return Ok(());
}

/// This function uses a [Client] to make a `GET` request to a `url` returning the [JsonValue] of the response.
/// # Parameters
/// - `client`: A reference to a [Client]. Used to handle `GET` request/response.
/// - `url`: Must be a valid REST Endpoint
/// # Errors
/// - If client fails to send `GET` request
/// - If response can not be parsed into a [JsonValue].
async fn get_json(client: &Client, url: &str) -> Result<JsonValue, Report> {
    let response = client
        .get(url) // Make `GET` request
        .send()
        .await?; // Send `GET` request

    let json_value = response
        .text()
        .await? // Extract text from response
        .parse::<JsonValue>()?; // Parse text into [JsonValue]

    // Return [JsonValue]
    return Ok(json_value);
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn dog_breed_ok() {
        let client = Client::new();
        let dog = get_json(&client, DOG_API_URL).await;
        assert!(dog.is_ok());
    }

    #[tokio::test]
    async fn cat_facts_ok() {
        let client = Client::new();
        let result = get_json(&client, CAT_API_URL).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn http_bin_ok() {
        let client = Client::new();
        let result = get_json(&client, HTTP_BIN_URL).await;
        assert!(result.is_ok());
    }
}
