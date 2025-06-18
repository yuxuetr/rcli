# AI Command Usage Guide

The `ai` subcommand allows you to interact with Google Gemini API to get AI-generated responses.

## Prerequisites

You need a Google Gemini API key. You can get one from [Google AI Studio](https://makersuite.google.com/app/apikey).

## Setup

Set your API key as an environment variable:

```bash
export GEMINI_API_KEY="your-api-key-here"
```

Or you can provide it directly in the command using the `-k` flag.

## Usage

### Basic Usage

Chat with AI using the default model (gemini-2.0-flash):

```bash
rcli ai chat -p "What is the capital of France?"
```

### With API Key Flag

If you haven't set the environment variable:

```bash
rcli ai chat -p "What is the capital of France?" -k "your-api-key-here"
```

### Using Different Models

Specify a different Gemini model:

```bash
rcli ai chat -p "Explain quantum computing" -m "gemini-1.5-pro"
```

### Adjusting Response Parameters

Control the response generation with temperature and max tokens:

```bash
rcli ai chat -p "Write a short story" --temperature 0.9 -t 1000
```

- `--temperature`: Controls randomness (0.0 to 1.0). Higher values make output more creative.
- `-t, --max-tokens`: Maximum number of tokens in the response (default: 2048)

### Output Formats

Get response in JSON format:

```bash
rcli ai chat -p "What is 2+2?" -o json
```

The JSON output includes:

- The original prompt
- The model used
- The AI response
- Temperature and max tokens settings

### Examples

1. Simple question:

   ```bash
   rcli ai chat -p "What are the benefits of exercise?"
   ```

2. Creative writing with high temperature:

   ```bash
   rcli ai chat -p "Write a haiku about coding" --temperature 0.9
   ```

3. Technical explanation with more tokens:

   ```bash
   rcli ai chat -p "Explain how TCP/IP works" -t 3000
   ```

4. JSON output for scripting:
   ```bash
   rcli ai chat -p "List 5 programming languages" -o json | jq '.response'
   ```

## Available Models

- `gemini-2.0-flash`: Latest and fastest model (default)
- `gemini-1.5-flash`: Fast model with good performance
- `gemini-1.5-pro`: More capable model for complex tasks

## Error Handling

The command will show appropriate error messages for:

- Missing API key
- Invalid API key
- Network issues
- API rate limits
- Invalid model names

## Tips

1. Keep your API key secure - never commit it to version control
2. Use lower temperature (0.1-0.3) for factual responses
3. Use higher temperature (0.7-0.9) for creative content
4. Adjust max tokens based on expected response length
5. Use JSON output when integrating with other tools or scripts
