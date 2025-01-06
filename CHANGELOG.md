# Changelog

All notable changes to this project will be documented in this file.

## [0.15.0] - 2025-01-03

### 🚀 Features

- Derives the hash impl for query ([#43](https://github.com/unleash/unleash-types-rs/issues/43))

## [0.14.0] - 2024-10-18

### ⚙️ Miscellaneous Tasks

- Dependency maintenance ([#42](https://github.com/unleash/unleash-types-rs/issues/42))

## [0.13.0] - 2024-07-19

### 🚀 Features

- Extend metrics and registration with optional metadata ([#40](https://github.com/unleash/unleash-types-rs/issues/40))

### 🐛 Bug Fixes

- Re-add snake-case of impression_data ([#41](https://github.com/unleash/unleash-types-rs/issues/41))

### ⚙️ Miscellaneous Tasks

- Clippy/cargo linting cleanups ([#38](https://github.com/unleash/unleash-types-rs/issues/38))
- Change tests so that we can add fields to clientapplication without a lot of noise ([#39](https://github.com/unleash/unleash-types-rs/issues/39))
- Chrono security bump

## [0.12.0] - 2024-04-25

### 🐛 Bug Fixes

- Convert frontend responses to camel case ([#36](https://github.com/unleash/unleash-types-rs/issues/36))

## [0.11.0] - 2024-01-23

### 🐛 Bug Fixes

- Add stickiness to strategy variants ([#34](https://github.com/unleash/unleash-types-rs/issues/34))

## [0.10.6] - 2023-11-30

### 🐛 Bug Fixes

- None strategy variants should serialize as empty array ([#32](https://github.com/unleash/unleash-types-rs/issues/32))

### ⚙️ Miscellaneous Tasks

- Release unleash-types version 0.10.6

## [0.10.5] - 2023-10-12

### 🚀 Features

- Add `dependencies` property to client feature struct ([#30](https://github.com/unleash/unleash-types-rs/issues/30))

## [0.10.4] - 2023-07-14

### 🚀 Features

- Add support for strategy variants ([#28](https://github.com/unleash/unleash-types-rs/issues/28))

## [0.10.3] - 2023-06-28

### ⚙️ Miscellaneous Tasks

- Cargo update

## [0.10.2] - 2023-06-28

### 🐛 Bug Fixes

- Skip serializing if option is none ([#27](https://github.com/unleash/unleash-types-rs/issues/27))

## [0.10.1] - 2023-05-30

### 🐛 Bug Fixes

- Patch ClientFeatures upsert behaviour to correctly take updated … ([#26](https://github.com/unleash/unleash-types-rs/issues/26))

## [0.10.0] - 2023-04-18

### 🐛 Bug Fixes

- Updated to not flatten properties map ([#25](https://github.com/unleash/unleash-types-rs/issues/25))

## [0.9.4] - 2023-04-14

### 🐛 Bug Fixes

- Gather unknown fields in Context into properties

## [0.9.3] - 2023-04-14

### 🐛 Bug Fixes

- Make properties an object in OpenAPI

## [0.9.2] - 2023-04-14

### 🚀 Features

- Openapi intoparam details for properties field

## [0.9.1] - 2023-03-08

### 🚀 Features

- Make EvaluatedToggle cloneable

## [0.9.0] - 2023-03-02

### 🚀 Features

- Adds an upsert method that prioritizes new incoming data ([#24](https://github.com/unleash/unleash-types-rs/issues/24))

### ⚙️ Miscellaneous Tasks

- Use u32 and camelCase ([#23](https://github.com/unleash/unleash-types-rs/issues/23))

## [0.8.3] - 2023-02-13

### 🚀 Features

- Derive IntoParams for Query and Context

### ⚙️ Miscellaneous Tasks

- U64 might require special treatment ([#21](https://github.com/unleash/unleash-types-rs/issues/21))

## [0.8.2] - 2023-02-09

### 🚀 Features

- Implements a few sugar traits for working with the underlying domain specific vecs types - deduplicate and merge ([#20](https://github.com/unleash/unleash-types-rs/issues/20))

### 🐛 Bug Fixes

- Handle legacy metrics format ([#17](https://github.com/unleash/unleash-types-rs/issues/17))

### ⚙️ Miscellaneous Tasks

- Bump version to 0.8.1 ([#18](https://github.com/unleash/unleash-types-rs/issues/18))

## [0.8.0] - 2023-02-07

### 🚀 Features

- Add logic for batching metrics ([#14](https://github.com/unleash/unleash-types-rs/issues/14))

### ⚙️ Miscellaneous Tasks

- Bump version to v0.8.0 ([#15](https://github.com/unleash/unleash-types-rs/issues/15))

## [0.7.1] - 2023-02-01

### 🐛 Bug Fixes

- Use BASE64_URL_SAFE to hash client_features

## [0.7.0] - 2023-01-27

### 🚀 Features

- Deterministic serialization ([#13](https://github.com/unleash/unleash-types-rs/issues/13))

## [0.6.0] - 2023-01-26

### 🚀 Features

- Add openapi feature adding utoipa ToSchema ([#12](https://github.com/unleash/unleash-types-rs/issues/12))

## [0.5.1] - 2023-01-24

### 🚀 Features

- Add a default implementation for Context struct ([#11](https://github.com/unleash/unleash-types-rs/issues/11))

## [0.5.0] - 2023-01-24

### 🚀 Features

- Add Context struct ([#10](https://github.com/unleash/unleash-types-rs/issues/10))

## [0.4.1] - 2023-01-20

### 🐛 Bug Fixes

- Setup weight type to handle lowercased variants of the enum ([#7](https://github.com/unleash/unleash-types-rs/issues/7))

### 💼 Other

- Updated parameters to cargo-smart-release ([#8](https://github.com/unleash/unleash-types-rs/issues/8))

### ⚙️ Miscellaneous Tasks

- Add workflows for testcoverage and release ([#5](https://github.com/unleash/unleash-types-rs/issues/5))
- Try to use rust-cache instead of github cache action ([#6](https://github.com/unleash/unleash-types-rs/issues/6))
- Fix typo in workflow
- Only run codequality and test-coverage when rust,toml or the workflow has changed
- Fetch the full repo and cache on failure
- Use correct property for git fetch depth in build ([#9](https://github.com/unleash/unleash-types-rs/issues/9))

## [0.4.0] - 2023-01-18

### 🚀 Features

- Add structs for ClientMetrics

### 🐛 Bug Fixes

- Remove unnecessary .into call ([#2](https://github.com/unleash/unleash-types-rs/issues/2))

### 💼 Other

- Add README ([#4](https://github.com/unleash/unleash-types-rs/issues/4))

### ⚙️ Miscellaneous Tasks

- Updated repo links after transferring ownership
- Add -rs suffix
- Add workflow for clippy and testing ([#1](https://github.com/unleash/unleash-types-rs/issues/1))
- Add CHANGELOG

## [0.3.0] - 2022-12-21

### ⚙️ Miscellaneous Tasks

- Release unleash-types version 0.3.0

## [0.2.1] - 2022-11-24

### ⚙️ Miscellaneous Tasks

- Release unleash-types version 0.2.1

## [0.2.0] - 2022-11-22

### 💼 Other

- Move client_features to separate mod

### ⚙️ Miscellaneous Tasks

- Release unleash-types version 0.2.0

<!-- generated by git-cliff -->
