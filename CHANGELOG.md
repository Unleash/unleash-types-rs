# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.8.1 (2023-02-08)

### Chore

 - <csr-id-8f07c238092ffe729ecc110abc12d389f70f26f7/> bump version to 0.8.1

### Bug Fixes

 - <csr-id-e4793a9f653c78bca55e63736cb94d210f72fa41/> handle legacy metrics format

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#17](https://github.com/Unleash/unleash-types-rs/issues/17), [#18](https://github.com/Unleash/unleash-types-rs/issues/18)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#17](https://github.com/Unleash/unleash-types-rs/issues/17)**
    - handle legacy metrics format ([`e4793a9`](https://github.com/Unleash/unleash-types-rs/commit/e4793a9f653c78bca55e63736cb94d210f72fa41))
 * **[#18](https://github.com/Unleash/unleash-types-rs/issues/18)**
    - bump version to 0.8.1 ([`8f07c23`](https://github.com/Unleash/unleash-types-rs/commit/8f07c238092ffe729ecc110abc12d389f70f26f7))
</details>

## v0.8.0 (2023-02-07)

### Chore

 - <csr-id-ff2837e50d7812b821949565cc4c00f44f37a8be/> bump version to v0.8.0

### New Features

 - <csr-id-323b65caa87788b200325f67ba63c7c7ef966fd6/> add logic for batching metrics

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 6 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#14](https://github.com/Unleash/unleash-types-rs/issues/14), [#15](https://github.com/Unleash/unleash-types-rs/issues/15)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#14](https://github.com/Unleash/unleash-types-rs/issues/14)**
    - add logic for batching metrics ([`323b65c`](https://github.com/Unleash/unleash-types-rs/commit/323b65caa87788b200325f67ba63c7c7ef966fd6))
 * **[#15](https://github.com/Unleash/unleash-types-rs/issues/15)**
    - bump version to v0.8.0 ([`ff2837e`](https://github.com/Unleash/unleash-types-rs/commit/ff2837e50d7812b821949565cc4c00f44f37a8be))
</details>

## v0.7.1 (2023-02-01)

### Bug Fixes

 - <csr-id-12d3de2edaf3c182f747b60408441dc80a7a65df/> Use BASE64_URL_SAFE to hash client_features

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 5 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release unleash-types v0.7.1 ([`239488a`](https://github.com/Unleash/unleash-types-rs/commit/239488a030e9dd450464dbe377975085ad2cf5b9))
    - Use BASE64_URL_SAFE to hash client_features ([`12d3de2`](https://github.com/Unleash/unleash-types-rs/commit/12d3de2edaf3c182f747b60408441dc80a7a65df))
</details>

## v0.7.0 (2023-01-27)

### New Features

 - <csr-id-ac9da78eb2a58b78580308268f5ee9e79c67e5c2/> deterministic serialization
   * task: make sure ClientFeatures to_str is deterministic
* fix: Serialize maps as BTreeMaps
* fix: variants should be ordered by name

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#13](https://github.com/Unleash/unleash-types-rs/issues/13)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#13](https://github.com/Unleash/unleash-types-rs/issues/13)**
    - deterministic serialization ([`ac9da78`](https://github.com/Unleash/unleash-types-rs/commit/ac9da78eb2a58b78580308268f5ee9e79c67e5c2))
 * **Uncategorized**
    - Release unleash-types v0.7.0 ([`4587020`](https://github.com/Unleash/unleash-types-rs/commit/45870207ccf3f1d3630cd71f9c8388f512b0dfb8))
</details>

## v0.6.0 (2023-01-26)

### New Features

 - <csr-id-387d1e27522b797e8d241d550cf2cef5ef5e1e83/> add openapi feature adding utoipa ToSchema

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#12](https://github.com/Unleash/unleash-types-rs/issues/12)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#12](https://github.com/Unleash/unleash-types-rs/issues/12)**
    - add openapi feature adding utoipa ToSchema ([`387d1e2`](https://github.com/Unleash/unleash-types-rs/commit/387d1e27522b797e8d241d550cf2cef5ef5e1e83))
 * **Uncategorized**
    - Release unleash-types v0.6.0 ([`430059d`](https://github.com/Unleash/unleash-types-rs/commit/430059db169a18c70fe570fe5e11e165f398a713))
</details>

## v0.5.1 (2023-01-24)

### New Features

 - <csr-id-9e6aa99a49d694a9348121f83bf38961b5f5eaeb/> add a default implementation for Context struct

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#11](https://github.com/Unleash/unleash-types-rs/issues/11)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#11](https://github.com/Unleash/unleash-types-rs/issues/11)**
    - add a default implementation for Context struct ([`9e6aa99`](https://github.com/Unleash/unleash-types-rs/commit/9e6aa99a49d694a9348121f83bf38961b5f5eaeb))
 * **Uncategorized**
    - Release unleash-types v0.5.1 ([`c251815`](https://github.com/Unleash/unleash-types-rs/commit/c251815a5072cfe3a3b9ac5ea3ce159a8ce95736))
</details>

## v0.5.0 (2023-01-24)

### New Features

 - <csr-id-8ab0650d22051bba48d4b685e7d25d90722ec716/> Add Context struct
   * feat: Add Context struct
* chore: add test for context deserializing
* feat: add serialize on context

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#10](https://github.com/Unleash/unleash-types-rs/issues/10)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/Unleash/unleash-types-rs/issues/10)**
    - Add Context struct ([`8ab0650`](https://github.com/Unleash/unleash-types-rs/commit/8ab0650d22051bba48d4b685e7d25d90722ec716))
 * **Uncategorized**
    - Release unleash-types v0.5.0 ([`f095fac`](https://github.com/Unleash/unleash-types-rs/commit/f095facd4199277d453206f79f5a26aa161f2324))
</details>

## v0.4.1 (2023-01-20)

<csr-id-d8c299bef00283760126f1e37cea94242f369727/>
<csr-id-9a47f754ab0e6025b41abde61bd00b457505c9fa/>
<csr-id-34f471e18fa7f0ddd028b3a2ff56caf472061169/>
<csr-id-96554d828eb4c7aaec898da7ada887d6b26cce85/>
<csr-id-66f019c249fdfec4d6867971e916eb31325c81e5/>
<csr-id-11027d0d82e6c83c9598b3f3beb5e8a1ecb3c0ac/>
<csr-id-21a93f0cd88076e0a94941f05587b5ad5e15cf37/>

### Chore

 - <csr-id-d8c299bef00283760126f1e37cea94242f369727/> use correct property for git fetch depth in build
 - <csr-id-9a47f754ab0e6025b41abde61bd00b457505c9fa/> fetch the full repo and cache on failure
 - <csr-id-34f471e18fa7f0ddd028b3a2ff56caf472061169/> only run codequality and test-coverage when rust,toml or the workflow has changed
 - <csr-id-96554d828eb4c7aaec898da7ada887d6b26cce85/> fix typo in workflow
 - <csr-id-66f019c249fdfec4d6867971e916eb31325c81e5/> Add workflows for testcoverage and release

### Bug Fixes

 - <csr-id-040cf879382a59989c27dca9c5c938418879a4c3/> Setup weight type to handle lowercased variants of the enum

### Other

 - <csr-id-11027d0d82e6c83c9598b3f3beb5e8a1ecb3c0ac/> updated parameters to cargo-smart-release
 - <csr-id-21a93f0cd88076e0a94941f05587b5ad5e15cf37/> try to use rust-cache instead of github cache action

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#5](https://github.com/Unleash/unleash-types-rs/issues/5), [#6](https://github.com/Unleash/unleash-types-rs/issues/6), [#7](https://github.com/Unleash/unleash-types-rs/issues/7), [#8](https://github.com/Unleash/unleash-types-rs/issues/8), [#9](https://github.com/Unleash/unleash-types-rs/issues/9)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#5](https://github.com/Unleash/unleash-types-rs/issues/5)**
    - Add workflows for testcoverage and release ([`66f019c`](https://github.com/Unleash/unleash-types-rs/commit/66f019c249fdfec4d6867971e916eb31325c81e5))
 * **[#6](https://github.com/Unleash/unleash-types-rs/issues/6)**
    - try to use rust-cache instead of github cache action ([`21a93f0`](https://github.com/Unleash/unleash-types-rs/commit/21a93f0cd88076e0a94941f05587b5ad5e15cf37))
 * **[#7](https://github.com/Unleash/unleash-types-rs/issues/7)**
    - Setup weight type to handle lowercased variants of the enum ([`040cf87`](https://github.com/Unleash/unleash-types-rs/commit/040cf879382a59989c27dca9c5c938418879a4c3))
 * **[#8](https://github.com/Unleash/unleash-types-rs/issues/8)**
    - updated parameters to cargo-smart-release ([`11027d0`](https://github.com/Unleash/unleash-types-rs/commit/11027d0d82e6c83c9598b3f3beb5e8a1ecb3c0ac))
 * **[#9](https://github.com/Unleash/unleash-types-rs/issues/9)**
    - use correct property for git fetch depth in build ([`d8c299b`](https://github.com/Unleash/unleash-types-rs/commit/d8c299bef00283760126f1e37cea94242f369727))
 * **Uncategorized**
    - Release unleash-types v0.4.1 ([`8339c42`](https://github.com/Unleash/unleash-types-rs/commit/8339c42b99a3223a832586defd54347a59385d02))
    - Release unleash-types v0.4.1 ([`f6d59de`](https://github.com/Unleash/unleash-types-rs/commit/f6d59dee03a61f838a438a1fa522c9f202908709))
    - fetch the full repo and cache on failure ([`9a47f75`](https://github.com/Unleash/unleash-types-rs/commit/9a47f754ab0e6025b41abde61bd00b457505c9fa))
    - only run codequality and test-coverage when rust,toml or the workflow has changed ([`34f471e`](https://github.com/Unleash/unleash-types-rs/commit/34f471e18fa7f0ddd028b3a2ff56caf472061169))
    - fix typo in workflow ([`96554d8`](https://github.com/Unleash/unleash-types-rs/commit/96554d828eb4c7aaec898da7ada887d6b26cce85))
</details>

## v0.4.0 (2023-01-18)

<csr-id-6a91af13c2d1a107e9f7260e2af5ec9f26294a4d/>
<csr-id-0a251eaa2bcfec307df4b6aef1dcda28ec8a9838/>
<csr-id-48f001e044f058cdce0bfa07fe7c839c7f37748b/>
<csr-id-2f4dde564fa4822bb86fd7c395de6da1b1babde3/>
<csr-id-406c9fd5bbaa11f79d47e1cbe95f641a987922b2/>

### Chore

 - <csr-id-6a91af13c2d1a107e9f7260e2af5ec9f26294a4d/> Add workflow for clippy and testing
 - <csr-id-0a251eaa2bcfec307df4b6aef1dcda28ec8a9838/> add -rs suffix
 - <csr-id-48f001e044f058cdce0bfa07fe7c839c7f37748b/> Updated repo links after transferring ownership

### Chore

 - <csr-id-406c9fd5bbaa11f79d47e1cbe95f641a987922b2/> Add CHANGELOG

### New Features

 - <csr-id-47a1cfec6023e4bb18a61142c69c771e5ca78bef/> Add structs for ClientMetrics
   Added structs with helper methods allowing us to increment enabled/disabled occurrences of toggles.

### Bug Fixes

 - <csr-id-85f8a586de7ffd9542fb0f5c949825c271718956/> Remove unnecessary .into call

### Other

 - <csr-id-2f4dde564fa4822bb86fd7c395de6da1b1babde3/> add README

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 28 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#1](https://github.com/Unleash/unleash-types-rs/issues/1), [#2](https://github.com/Unleash/unleash-types-rs/issues/2), [#4](https://github.com/Unleash/unleash-types-rs/issues/4)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1](https://github.com/Unleash/unleash-types-rs/issues/1)**
    - Add workflow for clippy and testing ([`6a91af1`](https://github.com/Unleash/unleash-types-rs/commit/6a91af13c2d1a107e9f7260e2af5ec9f26294a4d))
 * **[#2](https://github.com/Unleash/unleash-types-rs/issues/2)**
    - Remove unnecessary .into call ([`85f8a58`](https://github.com/Unleash/unleash-types-rs/commit/85f8a586de7ffd9542fb0f5c949825c271718956))
 * **[#4](https://github.com/Unleash/unleash-types-rs/issues/4)**
    - add README ([`2f4dde5`](https://github.com/Unleash/unleash-types-rs/commit/2f4dde564fa4822bb86fd7c395de6da1b1babde3))
 * **Uncategorized**
    - Release unleash-types v0.4.0 ([`cac15d9`](https://github.com/Unleash/unleash-types-rs/commit/cac15d93e19878884c0d83e8d7a54603bc50db0c))
    - Add CHANGELOG ([`406c9fd`](https://github.com/Unleash/unleash-types-rs/commit/406c9fd5bbaa11f79d47e1cbe95f641a987922b2))
    - Release unleash-types v0.4.0 ([`c5dbbdf`](https://github.com/Unleash/unleash-types-rs/commit/c5dbbdfcd2f518c45a9dda20c449bf38b5e2d692))
    - Add structs for ClientMetrics ([`47a1cfe`](https://github.com/Unleash/unleash-types-rs/commit/47a1cfec6023e4bb18a61142c69c771e5ca78bef))
    - add -rs suffix ([`0a251ea`](https://github.com/Unleash/unleash-types-rs/commit/0a251eaa2bcfec307df4b6aef1dcda28ec8a9838))
    - Updated repo links after transferring ownership ([`48f001e`](https://github.com/Unleash/unleash-types-rs/commit/48f001e044f058cdce0bfa07fe7c839c7f37748b))
</details>

## v0.3.0 (2022-12-21)

<csr-id-322465f4ebb3a94e3dd37306724f71c499d2d40e/>

### Chore

 - <csr-id-322465f4ebb3a94e3dd37306724f71c499d2d40e/> Release unleash-types version 0.3.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 27 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release unleash-types version 0.3.0 ([`322465f`](https://github.com/Unleash/unleash-types-rs/commit/322465f4ebb3a94e3dd37306724f71c499d2d40e))
    - Added type for frontend results ([`8b43e86`](https://github.com/Unleash/unleash-types-rs/commit/8b43e8631d1afdb24ce7181feb39f5754ed8fbd3))
</details>

## v0.2.1 (2022-11-24)

<csr-id-3338859afa3650bf35f63a5f5ec4a1b48908b1b3/>

### Chore

 - <csr-id-3338859afa3650bf35f63a5f5ec4a1b48908b1b3/> Release unleash-types version 0.2.1

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release unleash-types version 0.2.1 ([`3338859`](https://github.com/Unleash/unleash-types-rs/commit/3338859afa3650bf35f63a5f5ec4a1b48908b1b3))
    - patch constraint operators to correctly handle StrEndsWith and unknown cases ([`9ccf3e6`](https://github.com/Unleash/unleash-types-rs/commit/9ccf3e6f1039756506ff6e1e4a6ee7387035cfae))
</details>

## v0.2.0 (2022-11-22)

<csr-id-b17a4af499f1775185955eefe0d52eaff20fa860/>
<csr-id-bdc0da41e2025894d76486130961747e0028fb87/>

### Chore

 - <csr-id-b17a4af499f1775185955eefe0d52eaff20fa860/> Release unleash-types version 0.2.0

### Other

 - <csr-id-bdc0da41e2025894d76486130961747e0028fb87/> move client_features to separate mod

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release unleash-types version 0.2.0 ([`b17a4af`](https://github.com/Unleash/unleash-types-rs/commit/b17a4af499f1775185955eefe0d52eaff20fa860))
    - Revert "bump version" ([`bfac2fb`](https://github.com/Unleash/unleash-types-rs/commit/bfac2fb8675ad660c042e0174df745140d8f52bf))
    - bump version ([`6e1172a`](https://github.com/Unleash/unleash-types-rs/commit/6e1172a4ee82470d3fdfe55d22cf8dafd8ebe8db))
    - move client_features to separate mod ([`bdc0da4`](https://github.com/Unleash/unleash-types-rs/commit/bdc0da41e2025894d76486130961747e0028fb87))
</details>

## v0.1.0 (2022-11-22)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add tests for deserialization ([`f3d1f88`](https://github.com/Unleash/unleash-types-rs/commit/f3d1f8893547b056fde96db46ae0c89947813e40))
    - Start working with tests ([`130a113`](https://github.com/Unleash/unleash-types-rs/commit/130a1139db8f0dd9cb8f38b631b6e55f3e4cfa58))
    - Initial work with types from Client API response ([`dacb34d`](https://github.com/Unleash/unleash-types-rs/commit/dacb34d1105d0782a7d839c02a18d877814d720c))
</details>

