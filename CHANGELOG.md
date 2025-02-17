# Changelog

All notable changes to this project will be documented in this file.

## [0.15.8](https://github.com/Unleash/unleash-types-rs/compare/v0.15.7...v0.15.8) - 2025-02-17

### 🐛 Bug Fixes
- make event id public and rename (#73) (by @sjaanus) - #73

### Contributors

* @sjaanus

## [0.15.7](https://github.com/Unleash/unleash-types-rs/compare/v0.15.6...v0.15.7) - 2025-02-17

### 🚀 Features
- event_id finder for delta event (#71) (by @sjaanus) - #71

### Contributors

* @sjaanus

## [0.15.6](https://github.com/Unleash/unleash-types-rs/compare/v0.15.5...v0.15.6) - 2025-01-30

### ⚙️ Miscellaneous Tasks
- refactor delta applying to be in generic function (#69) (by @sjaanus) - #69

### Contributors

* @sjaanus

## [0.15.5](https://github.com/Unleash/unleash-types-rs/compare/v0.15.4...v0.15.5) - 2025-01-29

### 🚀 Features
- rework delta api to new schema (#66) (by @sjaanus) - #66

### Contributors

* @sjaanus

## [0.15.4](https://github.com/Unleash/unleash-types-rs/compare/v0.15.3...v0.15.4) - 2025-01-14

### 🚀 Features
- added from ClientFeaturesDelta for ClientFeatures (#63) (by @chriswk) - #63

### 🐛 Bug Fixes
- apply delta not updating features (#64) (by @sjaanus) - #64
- *(ci)* Use private key, not client token (by @chriswk)

### ⚙️ Miscellaneous Tasks
- *(ci)* Updated to use unleash-bot (#61) (by @chriswk)

### Contributors

* @sjaanus
* @chriswk

## [0.15.3](https://github.com/Unleash/unleash-types-rs/compare/v0.15.2...v0.15.3) - 2025-01-07

### 🚀 Features
- allow ClientFeatures to apply deltas to itself (#59) (by @chriswk) - #59

### Contributors

* @chriswk

## [0.15.2](https://github.com/Unleash/unleash-types-rs/compare/v0.15.1...v0.15.2) - 2025-01-07

### 🚀 Features
- client features delta schema (#57) (by @chriswk) - #57

### Contributors

* @chriswk

## [0.15.1](https://github.com/Unleash/unleash-types-rs/compare/v0.15.0...v0.15.1) - 2025-01-06

### 🚀 Features
- added meta field to ClientFeatures (#44) (by @chriswk) - #44

### 💼 Other
- migrate to git cliff for our changelog (#54) (by @chriswk) - #54
- Release plz workflow ([#45](https://github.com/Unleash/unleash-types-rs/pull/45)) (by @chriswk) - #45

### Dependency updates
- *(deps)* bump serde from 1.0.210 to 1.0.217 (#52) (by @dependabot[bot])
- *(deps)* bump actions/checkout from 3 to 4 (#49) (by @dependabot[bot])
- *(deps)* bump serde_json from 1.0.128 to 1.0.134 (#51) (by @dependabot[bot])
- *(deps)* bump github/codeql-action from 2 to 3 (#48) (by @dependabot[bot])
- *(deps)* bump xxhash-rust from 0.8.12 to 0.8.15 (#47) (by @dependabot[bot])
- *(deps)* bump chrono from 0.4.38 to 0.4.39 (#53) (by @dependabot[bot])

### ⚙️ Miscellaneous Tasks
- added build(deps) as a dependency update group parser (by @chriswk)
- Trust [changelog] section (by @chriswk)
- Add dep-update prefix for dependabot upgrades (#55) (by @chriswk) - #55

### Contributors

* @chriswk
* @dependabot[bot]

## [0.15.0] - 2025-01-03

### 🚀 Features

- Derives the hash impl for query ([#43](https://github.com/unleash/unleash-types-rs/issues/43)) by @thomasheartman in #43


### 💼 Other

- Clippy --fix ([#33](https://github.com/unleash/unleash-types-rs/issues/33)) by @gastonfournier in #33

- Release unleash-types v0.15.0 by @thomasheartman


## [0.14.0] - 2024-10-18

### 💼 Other

- Release unleash-types v0.14.0 by @chriswk


### ⚙️ Miscellaneous Tasks

- Dependency maintenance ([#42](https://github.com/unleash/unleash-types-rs/issues/42)) by @chriswk in #42


## [0.13.0] - 2024-07-19

### 🚀 Features

- Extend metrics and registration with optional metadata ([#40](https://github.com/unleash/unleash-types-rs/issues/40)) by @sighphyre in #40


### 🐛 Bug Fixes

- Re-add snake-case of impression_data ([#41](https://github.com/unleash/unleash-types-rs/issues/41)) by @chriswk in #41


### 💼 Other

- Release unleash-types v0.13.0 by @chriswk


### ⚙️ Miscellaneous Tasks

- Clippy/cargo linting cleanups ([#38](https://github.com/unleash/unleash-types-rs/issues/38)) by @sighphyre in #38

- Change tests so that we can add fields to clientapplication without a lot of noise ([#39](https://github.com/unleash/unleash-types-rs/issues/39)) by @sighphyre in #39

- Chrono security bump by @chriswk


## [0.12.0] - 2024-04-25

### 🐛 Bug Fixes

- Convert frontend responses to camel case ([#36](https://github.com/unleash/unleash-types-rs/issues/36)) by @sighphyre in #36


### 💼 Other

- Release unleash-types v0.12.0 by @sighphyre


## [0.11.0] - 2024-01-23

### 🐛 Bug Fixes

- Add stickiness to strategy variants ([#34](https://github.com/unleash/unleash-types-rs/issues/34)) by @sighphyre in #34


### 💼 Other

- Release unleash-types v0.11.0 by @chriswk


## [0.10.6] - 2023-11-30

### 🐛 Bug Fixes

- None strategy variants should serialize as empty array ([#32](https://github.com/unleash/unleash-types-rs/issues/32)) by @gastonfournier in #32


### ⚙️ Miscellaneous Tasks

- Release unleash-types version 0.10.6 by @gastonfournier


## [0.10.5] - 2023-10-12

### 🚀 Features

- Add `dependencies` property to client feature struct ([#30](https://github.com/unleash/unleash-types-rs/issues/30)) by @thomasheartman in #30


### 💼 Other

- Release unleash-types v0.10.5 by @thomasheartman

- Release unleash-types v0.10.5 by @thomasheartman

- Release unleash-types v0.10.5 by @thomasheartman


## [0.10.4] - 2023-07-14

### 🚀 Features

- Add support for strategy variants ([#28](https://github.com/unleash/unleash-types-rs/issues/28)) by @sighphyre in #28


### 💼 Other

- Release unleash-types v0.10.4 by @sighphyre


## [0.10.3] - 2023-06-28

### 💼 Other

- Release unleash-types v0.10.3 by @chriswk


### ⚙️ Miscellaneous Tasks

- Cargo update by @chriswk


## [0.10.2] - 2023-06-28

### 🐛 Bug Fixes

- Skip serializing if option is none ([#27](https://github.com/unleash/unleash-types-rs/issues/27)) in #27


### 💼 Other

- Release unleash-types v0.10.2 by @chriswk


## [0.10.1] - 2023-05-30

### 🐛 Bug Fixes

- Patch ClientFeatures upsert behaviour to correctly take updated … ([#26](https://github.com/unleash/unleash-types-rs/issues/26)) by @sighphyre in #26


### 💼 Other

- Release unleash-types v0.10.1 by @sighphyre


## [0.10.0] - 2023-04-18

### 🐛 Bug Fixes

- Updated to not flatten properties map ([#25](https://github.com/unleash/unleash-types-rs/issues/25)) in #25


### 💼 Other

- Release unleash-types v0.10.0 by @chriswk


## [0.9.4] - 2023-04-14

### 🐛 Bug Fixes

- Gather unknown fields in Context into properties by @chriswk


### 💼 Other

- Release unleash-types v0.9.4 by @chriswk


## [0.9.3] - 2023-04-14

### 🐛 Bug Fixes

- Make properties an object in OpenAPI by @chriswk


### 💼 Other

- Release unleash-types v0.9.3 by @chriswk


## [0.9.2] - 2023-04-14

### 🚀 Features

- Openapi intoparam details for properties field by @chriswk


### 💼 Other

- Release unleash-types v0.9.2 by @chriswk


## [0.9.1] - 2023-03-08

### 🚀 Features

- Make EvaluatedToggle cloneable by @chriswk


### 💼 Other

- Create LICENSE by @sighphyre

- Release unleash-types v0.9.1 by @chriswk


## [0.9.0] - 2023-03-02

### 🚀 Features

- Adds an upsert method that prioritizes new incoming data ([#24](https://github.com/unleash/unleash-types-rs/issues/24)) in #24


### 💼 Other

- Release unleash-types v0.9.0


### ⚙️ Miscellaneous Tasks

- Use u32 and camelCase ([#23](https://github.com/unleash/unleash-types-rs/issues/23)) by @gastonfournier in #23


## [0.8.3] - 2023-02-13

### 🚀 Features

- Derive IntoParams for Query and Context by @chriswk


### 💼 Other

- Release unleash-types v0.8.3 by @chriswk

- Release unleash-types v0.8.3 by @chriswk


### ⚙️ Miscellaneous Tasks

- U64 might require special treatment ([#21](https://github.com/unleash/unleash-types-rs/issues/21)) by @gastonfournier in #21


## [0.8.2] - 2023-02-09

### 🚀 Features

- Implements a few sugar traits for working with the underlying domain specific vecs types - deduplicate and merge ([#20](https://github.com/unleash/unleash-types-rs/issues/20)) by @sighphyre in #20


### 🐛 Bug Fixes

- Handle legacy metrics format ([#17](https://github.com/unleash/unleash-types-rs/issues/17)) by @sighphyre in #17


### 💼 Other

- Release unleash-types v0.8.1 ([#19](https://github.com/unleash/unleash-types-rs/issues/19)) by @sighphyre in #19

- Release unleash-types v0.8.2


### ⚙️ Miscellaneous Tasks

- Bump version to 0.8.1 ([#18](https://github.com/unleash/unleash-types-rs/issues/18)) by @sighphyre in #18


## [0.8.0] - 2023-02-07

### 🚀 Features

- Add logic for batching metrics ([#14](https://github.com/unleash/unleash-types-rs/issues/14)) by @sighphyre in #14


### ⚙️ Miscellaneous Tasks

- Bump version to v0.8.0 ([#15](https://github.com/unleash/unleash-types-rs/issues/15)) by @nunogois in #15


## [0.7.1] - 2023-02-01

### 🐛 Bug Fixes

- Use BASE64_URL_SAFE to hash client_features


### 💼 Other

- Release unleash-types v0.7.1


## [0.7.0] - 2023-01-27

### 🚀 Features

- Deterministic serialization ([#13](https://github.com/unleash/unleash-types-rs/issues/13)) in #13


### 💼 Other

- Release unleash-types v0.7.0


## [0.6.0] - 2023-01-26

### 🚀 Features

- Add openapi feature adding utoipa ToSchema ([#12](https://github.com/unleash/unleash-types-rs/issues/12)) in #12


### 💼 Other

- Release unleash-types v0.6.0


## [0.5.1] - 2023-01-24

### 🚀 Features

- Add a default implementation for Context struct ([#11](https://github.com/unleash/unleash-types-rs/issues/11)) by @sighphyre in #11


### 💼 Other

- Release unleash-types v0.5.1


## [0.5.0] - 2023-01-24

### 🚀 Features

- Add Context struct ([#10](https://github.com/unleash/unleash-types-rs/issues/10)) by @sighphyre in #10


### 💼 Other

- Release unleash-types v0.5.0


## [0.4.1] - 2023-01-20

### 🐛 Bug Fixes

- Setup weight type to handle lowercased variants of the enum ([#7](https://github.com/unleash/unleash-types-rs/issues/7)) in #7


### 💼 Other

- Updated parameters to cargo-smart-release ([#8](https://github.com/unleash/unleash-types-rs/issues/8)) in #8

- Release unleash-types v0.4.1

- Release unleash-types v0.4.1


### ⚙️ Miscellaneous Tasks

- Add workflows for testcoverage and release ([#5](https://github.com/unleash/unleash-types-rs/issues/5)) in #5

- Try to use rust-cache instead of github cache action ([#6](https://github.com/unleash/unleash-types-rs/issues/6)) in #6

- Fix typo in workflow

- Only run codequality and test-coverage when rust,toml or the workflow has changed

- Fetch the full repo and cache on failure

- Use correct property for git fetch depth in build ([#9](https://github.com/unleash/unleash-types-rs/issues/9)) by @sighphyre in #9


## [0.4.0] - 2023-01-18

### 🚀 Features

- Add structs for ClientMetrics


### 🐛 Bug Fixes

- Remove unnecessary .into call ([#2](https://github.com/unleash/unleash-types-rs/issues/2)) in #2


### 💼 Other

- Add README ([#4](https://github.com/unleash/unleash-types-rs/issues/4)) in #4

- Release unleash-types v0.4.0

- Release unleash-types v0.4.0


### ⚙️ Miscellaneous Tasks

- Updated repo links after transferring ownership

- Add -rs suffix

- Add workflow for clippy and testing ([#1](https://github.com/unleash/unleash-types-rs/issues/1)) in #1

- Add CHANGELOG


## [0.3.0] - 2022-12-21

### 💼 Other

- Added type for frontend results


### ⚙️ Miscellaneous Tasks

- Release unleash-types version 0.3.0


## [0.2.1] - 2022-11-24

### 💼 Other

- Patch constraint operators to correctly handle StrEndsWith and unknown cases by @sighphyre


### ⚙️ Miscellaneous Tasks

- Release unleash-types version 0.2.1 by @sighphyre


## [0.2.0] - 2022-11-22

### 💼 Other

- Move client_features to separate mod

- Bump version

- Revert "bump version"


### ⚙️ Miscellaneous Tasks

- Release unleash-types version 0.2.0


## [0.1.0] - 2022-11-22

### 💼 Other

- Initial work with types from Client API response

- Start working with tests

- Add tests for deserialization by @sighphyre


<!-- generated by git-cliff -->
