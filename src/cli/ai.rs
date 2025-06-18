use crate::CmdExecutor;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum AiSubCommand {
  #[command(about = "Chat with AI using Google Gemini API")]
  Chat(AiChatOpts),
}

#[derive(Debug, Parser)]
pub struct AiChatOpts {
  /// The text prompt to send to the AI
  #[arg(short, long, value_name = "TEXT")]
  pub prompt: String,

  /// The Gemini model to use (e.g., gemini-2.0-flash, gemini-1.5-flash, gemini-1.5-pro)
  #[arg(short, long, default_value = "gemini-2.0-flash")]
  pub model: String,

  /// API key for Google Gemini (can also be set via GEMINI_API_KEY env var)
  #[arg(short = 'k', long)]
  pub api_key: Option<String>,

  /// Maximum tokens in the response
  #[arg(short = 't', long, default_value = "2048")]
  pub max_tokens: u32,

  /// Temperature for response generation (0.0 to 1.0)
  #[arg(long, default_value = "0.7")]
  pub temperature: f32,

  /// Output format (text or json)
  #[arg(short, long, default_value = "text")]
  pub output: AiOutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum AiOutputFormat {
  Text,
  Json,
}

impl std::fmt::Display for AiOutputFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AiOutputFormat::Text => write!(f, "text"),
      AiOutputFormat::Json => write!(f, "json"),
    }
  }
}

impl std::str::FromStr for AiOutputFormat {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "text" => Ok(AiOutputFormat::Text),
      "json" => Ok(AiOutputFormat::Json),
      _ => Err(anyhow::anyhow!("Invalid output format: {}", s)),
    }
  }
}

impl CmdExecutor for AiSubCommand {
  async fn execute(self) -> anyhow::Result<()> {
    match self {
      AiSubCommand::Chat(opts) => opts.execute().await,
    }
  }
}

impl CmdExecutor for AiChatOpts {
  async fn execute(self) -> anyhow::Result<()> {
    crate::process_ai_chat(
      &self.prompt,
      &self.model,
      self.api_key,
      self.max_tokens,
      self.temperature,
      self.output,
    )
    .await
  }
}
