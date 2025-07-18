# CCOR - Anthropic to OpenAI/OpenRouter Adapter

This is a Rust application that acts as an adapter between the Anthropic API format and the OpenAI/OpenRouter API format. It spins up a webserver, receives requests in the Anthropic format, rewrites them to the OpenAI/OpenRouter format, sends them to OpenRouter, and streams the results back.

## Prerequisites

- Rust and Cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Configuration

1.  Create a `.env` file in the root of the project.
2.  Add the following environment variables to the `.env` file:

    ```
    OPENROUTER_MODEL_HAIKU=mistralai/devstral-small # or another model
    OPENROUTER_MODEL_SONNET=mistralai/devstral-small # or another model
    OPENROUTER_MODEL_OPUS=mistralai/devstral-small # or another model
    ```

## Running the Application

To run the application, use the following command:

```bash
cargo run
```

The server will start on `0.0.0.0:3000`.

## Using Claude Code

Start the proxy according to the docs which will run it in localhost:3073

export ANTHROPIC_BASE_URL=http://localhost:3073

export ANTHROPIC_AUTH_TOKEN="your openrouter api key"

run claude code

## Logging

CCOR can also log requests and responses to a specified directory. To enable this, pass the `--logging` flag followed by the path to the directory where you want the logs to be stored.

For example, to log requests and responses to the `logs` directory, run the following command:

```bash
cargo run --logging logs
```

This will start the server and log requests and responses to the `logs` directory.

## License

CCOR is licensed under the MIT License.
