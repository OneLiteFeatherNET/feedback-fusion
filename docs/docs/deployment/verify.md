# Verify the deployment

To verify and test the deployment of FeedbackFusion, you can run the integration tests. The following prerequisites and instructions will guide you through the process.

## Prerequisites

- **Rust** installed on your machine.
- **cargo-make** installed. You can install it using the following command:
  ```sh
  cargo install cargo-make
  ```
- The FeedbackFusion application must be accessible on port 8000 on the machine where the tests are executed. This can be achieved via port-forwarding if using Kubernetes.
- Override the `OIDC_PROVIDER`, `CLIENT_ID` and `CLIENT_SECRET` in the Makefile.

## Running Integration Tests

To run the integration tests, execute the following command:

```sh
cargo make integration_test
