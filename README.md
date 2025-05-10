# Axum Boot

## Axum Boot is a set of crates designed to simplify the process of building and securing REST applications using [Axum](https://github.com/tokio-rs/axum). It provides utilities and modules that streamline common application needs, such as route security, JWT validation, and more.

## Security module

#### Axum Boot includes a robust security module that integrates seamlessly with your Axum application. This module provides:

- Route-level Security: Secure your routes by specifying roles or permissions required for access. You can easily extend this to implement fine-grained security rules.
- JWT Authentication: Validate JWT tokens to ensure the user is authenticated. The system supports RSA signature verification.
