# CCORP - Anthropic to OpenAI/OpenRouter Adapter

## Use Claude Code with any OpenRouter model.

CCORP (Claude Code OpenRouter Proxy) is a high-performance Rust application that acts as an adapter between the Anthropic API format and the OpenAI/OpenRouter API format. It provides a seamless bridge for applications expecting Anthropic's API to work with OpenRouter's extensive model collection.

## Features

- **API Translation**: Converts Anthropic API requests to OpenAI/OpenRouter format and vice versa
- **Streaming Support**: Full support for both streaming and non-streaming API calls
- **Model Mapping**: Flexible configuration to map Claude models to any OpenRouter-supported model
- **Web UI**: Built-in web interface for easy model switching at runtime
- **Request Logging**: Optional logging of all requests and responses for debugging

## Installation

![asset/image.jpg](assets/image.jpg)

### Via Cargo

#### Prerequisites
- Rust and Cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

```bash
cargo install --git https://github.com/terhechte/CCORP --bin ccor
```

### Via Releases

Download the latest binary from the [releases page](https://github.com/terhechte/CCORP/releases).

## Configuration

### Step 1: Environment Setup

Create a `.env` file in the root directory with your OpenRouter API key:

```env
OPENROUTER_API_KEY=your_openrouter_api_key_here
```

### Step 2: Model Configuration

Create a `config.json` file to configure the port and model mappings:

```json
{
  "port": 3000,
  "models": {
    "haiku": "mistralai/mistral-7b-instruct",
    "sonnet": "meta-llama/llama-3.2-90b-vision-instruct",
    "opus": "openai/gpt-4o"
  }
}
```

You can map Claude models to any model available on OpenRouter.

## Running the Application

### Basic Usage

```bash
cargo run
```

The server will start on `0.0.0.0:3000` (or the port specified in `config.json`).

### With Request Logging

To enable logging of all requests and responses:

```bash
cargo run -- --logging logs
```

This creates timestamped JSON files in the `logs` directory for each request/response pair.

## Using with Claude Code CLI

CCORP is designed to work seamlessly with Anthropic's Claude Code CLI:

1. Start CCORP (it will run on port 3000 by default)
2. Set environment variables:

   ```bash
   export ANTHROPIC_BASE_URL=http://localhost:3000
   export ANTHROPIC_AUTH_TOKEN="your_openrouter_api_key"
   ```

3. Run Claude Code as normal:

   ```bash
   claude
   ```

## Web UI for Model Management

CCORP includes a web interface for dynamically switching models without restarting the server.

Visit `http://localhost:3000/switch-model` in your browser to:
- View all available OpenRouter models
- Change model mappings for Haiku, Sonnet, and Opus

Changes are saved to `config.json` and take effect immediately.

## API Usage Examples

### Non-Streaming Request

```bash
curl -X POST http://localhost:3000/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: your_openrouter_api_key" \
  -d '{
    "model": "claude-3-5-haiku-20241022",
    "messages": [{"role": "user", "content": "Hello, world!"}]
  }'
```

### Streaming Request

```bash
curl -X POST http://localhost:3000/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: your_openrouter_api_key" \
  -d '{
    "model": "claude-3-5-sonnet-20241022",
    "messages": [{"role": "user", "content": "Tell me a story"}],
    "stream": true
  }'
```

## Architecture

CCORP is built with:
- **Rust**: For high performance and memory safety
- **Axum**: Modern async web framework
- **Tokio**: Async runtime
- **Minijinja**: Template engine for the web UI

The request flow:
1. Receive Anthropic-formatted request
2. Map Claude model to configured OpenRouter model
3. Transform request to OpenAI format
4. Forward to OpenRouter
5. Transform response back to Anthropic format
6. Stream or return to client

## Development

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

CCORP is licensed under the MIT License. See [LICENSE](LICENSE) for details.
