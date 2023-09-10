# Testing
## Development Installation and Build
Make sure you have the `wasm32-unknown-unknown` target for Rust so you can compile code to run WebAssembly in the browser.
```bash
rustup target add wasm32-unknown-unknown
```
Cargo should be able to install and build all necessary dependencies for building the main Rust package.

To test against DynamoDB, use the [DynamoDB Local](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBLocal.html) tool, which can be run as a Docker image, or run directly using the JRE.

To test the API or Lambda functions, use the [SAM CLI Tool](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html). Make sure you have read the [developer guide on building Rust for SAM](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/building-rust.html).

## Unit Tests

## Integration Tests

## Full Test Environment