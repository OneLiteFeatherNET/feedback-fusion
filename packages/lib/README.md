<h1 align="center">
    FeedbackFusion <br />

  <img src="https://img.shields.io/badge/built_with-Rust-dca282" />
  <img src="https://img.shields.io/badge/License-MIT-blue" />
</h1>

## About

FeedbackFusion is a cloud-native application meticulously crafted for seamless compatibility with Kubernetes. It is designed to efficiently collect and manage user feedback at a large scale.

## Documentation

[Click here for the lib documentation](https://onelitefeathernet.github.io/feedback-fusion/nightly/docs/frontend-library)

## Key Features

- **Cloud-native and Kubernetes-compatible**: The application operates in a fully stateless manner (except for the database connection), ensuring smooth integration into Kubernetes environments.

- **Support for Multiple Databases**: Utilizing the rbatis ORM, FeedbackFusion supports a variety of databases, including MySQL, MariaDB, PostgreSQL, and MSSQL.

- **High-Scale Feedback Collection**: Engineered for high scalability, the application leverages gRPC for efficient, high-performance communication, enabling the collection of large volumes of user feedback.

- **Universal Frontend**: The frontend is built on the lit library and adheres to Open-WC standards, providing flexibility and independence from specific frameworks.

## License

FeedbackFusion is licensed under the MIT License.
