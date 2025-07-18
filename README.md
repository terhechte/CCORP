# ccor - Anthropic to OpenAI/OpenRouter Adapter

This is a Rust application that acts as an adapter between the Anthropic API format and the OpenAI/OpenRouter API format. It spins up a webserver, receives requests in the Anthropic format, rewrites them to the OpenAI/OpenRouter format, sends them to OpenRouter, and streams the results back.

## Prerequisites

- Rust and Cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Configuration

1.  Create a `.env` file in the root of the project.
2.  Add the following environment variables to the `.env` file:

    ```
    OPENROUTER_API_KEY=your_openrouter_api_key
    ```

    Replace `your_openrouter_api_key` with your actual OpenRouter API key.

## Running the Application

To run the application, use the following command:

```bash
cargo run
```

The server will start on `0.0.0.0:3000`.

## Usage

You can make requests to the server using `curl` or any other HTTP client. The endpoint is `/v1/messages`.

Here's an example of a non-streaming request:

```bash
curl -X POST http://localhost:3000/v1/messages \
-H "Content-Type: application/json" \
-H "x-api-key: your_openrouter_api_key" \
-d '{
  "model": "anthropic/claude-3-haiku-20240307",
  "messages": [
    {
      "role": "user",
      "content": "Hello, world!"
    }
  ]
}'
```

Here's an example of a streaming request:

```bash
curl -X POST http://localhost:3000/v1/messages \
-H "Content-Type: application/json" \
-H "x-api-key: your_openrouter_api_key" \
-d '{
  "model": "anthropic/claude-3-haiku-20240307",
  "messages": [
    {
      "role": "user",
      "content": "Hello, world!"
    }
  ],
  "stream": true
}'
```