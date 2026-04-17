
use reqwest;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(serde::Serialize)]
struct AnthropicRequestBody {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(serde::Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[derive(serde::Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(serde::Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(serde::Deserialize)]
struct Choice {
    message: Message,
}

#[derive(serde::Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const HEADER_AUTHORIZATION: &str = "Authorization";
const HEADER_CONTENT_TYPE: &str = "Content-Type";
const HEADER_CONTENT_TYPE_VALUE: &str = "application/json";

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const HEADER_X_API_KEY: &str = "x-api-key";
const HEADER_ANTHROPIC_VERSION: &str = "anthropic-version";
const HEADER_ANTHROPIC_VERSION_VALUE: &str = "2023-06-01";

pub async fn call_openai(prompt: &str, token: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let request_body = RequestBody {
        model: model.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    let response = client
        .post(OPENAI_API_URL)
        .header(HEADER_AUTHORIZATION, format!("Bearer {}", token))
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_VALUE)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("OpenAI API error: {}", error_text).into());
    }

    let api_response: Response = response.json().await?;

    let content = api_response.choices.first()
        .map(|choice| choice.message.content.clone())
        .ok_or("No response from OpenAI")?;

    Ok(content)
}

pub async fn call_anthropic(prompt: &str, token: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let request_body = AnthropicRequestBody {
        model: model.to_string(),
        max_tokens: 4096,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    let response = client
        .post(ANTHROPIC_API_URL)
        .header(HEADER_X_API_KEY, token)
        .header(HEADER_ANTHROPIC_VERSION, HEADER_ANTHROPIC_VERSION_VALUE)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_VALUE)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("Anthropic API error: {}", error_text).into());
    }

    let api_response: AnthropicResponse = response.json().await?;

    let content = api_response.content.first()
        .filter(|block| block.block_type == "text")
        .map(|block| block.text.clone())
        .ok_or("No response from Anthropic")?;

    Ok(content)
}
