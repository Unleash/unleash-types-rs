[![Coverage Status](https://coveralls.io/repos/github/Unleash/unleash-types-rs/badge.svg?branch=main)](https://coveralls.io/github/Unleash/unleash-types-rs?branch=main)

# Unleash types

This project represents Unleash OpenAPI types as Rust structs.

## Usage

Add unleash-types to your Cargo file.

### Client features

- client_features::ClientFeatures struct represents the data as it comes back from an Unleash server from the `/api/client/features` endpoint.

### Client metrics

- client_metrics::ClientApplication struct represents the data expected by an Unleash server in a POST to the `/api/client/register` endpoint
- client_metrics::ClientMetrics struct represents the data expected by an Unleash server in a POST to the `/api/client/metrics` endpoint

### Frontend types

- frontend::FrontendResults struct represents the data expected by a proxy client GET call to the `/api/proxy` | `/api/frontend` endpoints.
  - This is intended to help us implement [Unleash Edge](https://github.com/Unleash/unleash-edge)
