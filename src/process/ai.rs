use crate::cli::AiOutputFormat;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

const GEMINI_API_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

// Gemini API request/response structures
#[derive(Debug, Serialize)]
pub struct GeminiRequest {
  pub contents: Vec<Content>,
  pub generation_config: GenerationConfig,
}

#[derive(Debug, Serialize)]
pub struct Content {
  pub parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
pub struct Part {
  pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
  pub temperature: f32,
  pub max_output_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
  pub candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
  pub content: ResponseContent,
}

#[derive(Debug, Deserialize)]
pub struct ResponseContent {
  pub parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
pub struct ResponsePart {
  pub text: String,
}

pub async fn process_ai_chat(
  prompt: &str,
  model: &str,
  api_key: Option<String>,
  max_tokens: u32,
  temperature: f32,
  output_format: AiOutputFormat,
) -> Result<()> {
  // Get API key from parameter or environment variable
  let api_key = api_key
        .or_else(|| std::env::var("GEMINI_API_KEY").ok())
        .context("API key not provided. Please set GEMINI_API_KEY environment variable or use --api-key parameter")?;

  // Build request body
  let request_body = GeminiRequest {
    contents: vec![Content {
      parts: vec![Part {
        text: prompt.to_string(),
      }],
    }],
    generation_config: GenerationConfig {
      temperature,
      max_output_tokens: max_tokens,
    },
  };

  // Make API request
  let client = Client::new();
  let url = format!("{}/{}:generateContent", GEMINI_API_BASE_URL, model);

  let response = client
    .post(&url)
    .header("Content-Type", "application/json")
    .query(&[("key", api_key.as_str())])
    .json(&request_body)
    .send()
    .await
    .context("Failed to send request to Gemini API")?;

  // Check response status
  let status = response.status();
  if !status.is_success() {
    let error_text = response
      .text()
      .await
      .unwrap_or_else(|_| "Unknown error".to_string());
    return Err(anyhow::anyhow!(
      "Gemini API request failed with status {}: {}",
      status,
      error_text
    ));
  }

  // Parse response
  let gemini_response: GeminiResponse = response
    .json()
    .await
    .context("Failed to parse Gemini API response")?;

  // Extract and display the response
  if let Some(candidate) = gemini_response.candidates.first() {
    if let Some(part) = candidate.content.parts.first() {
      match output_format {
        AiOutputFormat::Text => {
          println!("{}", part.text);
        }
        AiOutputFormat::Json => {
          let json_output = json!({
              "prompt": prompt,
              "model": model,
              "response": part.text,
              "temperature": temperature,
              "max_tokens": max_tokens,
          });
          println!("{}", serde_json::to_string_pretty(&json_output)?);
        }
      }
    } else {
      return Err(anyhow::anyhow!("No content in API response"));
    }
  } else {
    return Err(anyhow::anyhow!("No candidates in API response"));
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gemini_api_url() {
    let model = "gemini-pro";
    let expected_url = format!("{}/{}:generateContent", GEMINI_API_BASE_URL, model);
    assert_eq!(
      expected_url,
      "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent"
    );
  }
}
