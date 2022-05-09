#![feature(derive_default_enum)]
#![feature(crate_visibility_modifier)]

use crate::{api::*, client::Client, model::Model};

#[macro_use]
mod macros;
pub mod api;
pub mod client;
mod model;

const OPENAI_URL: &str = "https://api.openai.com/v1";
#[tokio::main]
async fn main() {
    let token = std::env::var("GPT_API_TOKEN").unwrap();

    let client = Client::new(token);

    let request = completions::Builder::default()
        .model(Model::Babbage)
        .prompt("what is 1 + 2?".into())
        .build()
        .unwrap();

    let response = client.request(request).await.unwrap();
    let answer = &response.choices[0].text;

    println!("{answer}");
}
