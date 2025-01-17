# Changelog

## [v0.5.0-dev.4](https://github.com/iotaledger/identity.rs/tree/v0.5.0-dev.4) (2022-02-14)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.5.0-dev.3...v0.5.0-dev.4)

### Changed

- Remove unused `Account` milestone option [\#645](https://github.com/iotaledger/identity.rs/pull/645)
- Change document controller type to `OneOrSet` [\#638](https://github.com/iotaledger/identity.rs/pull/638)
- Rename `MethodQuery` to `DIDUrlQuery`, move `OrderedSet`, `KeyComparable` [\#634](https://github.com/iotaledger/identity.rs/pull/634)
- Change `also_known_as` type to `OrderedSet` [\#632](https://github.com/iotaledger/identity.rs/pull/632)

### Patch

- Support verification methods with the same fragment [\#623](https://github.com/iotaledger/identity.rs/pull/623)

## [v0.5.0-dev.3](https://github.com/iotaledger/identity.rs/tree/v0.5.0-dev.3) (2022-01-25)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.5.0-dev.2...v0.5.0-dev.3)
 
This release introduces a breaking change for diff updates created by versions `v0.5.0-dev.1` and `v0.5.0-dev.2` (previous diff updates from `<=v0.4.0` are already incompatible due to breaking changes to the document and message structure in `v0.5.0-dev.1`). To migrate, please publish an integration update containing all diff changes to prevent unexpected changes to resolved DID Documents.

### Changed

- Move verification functionality from `DocumentVerifier` to  `CoreDocument`  [\#606](https://github.com/iotaledger/identity.rs/pull/606)
- Fix dependent diff updates being rejected [\#605](https://github.com/iotaledger/identity.rs/pull/605)
- Change `Account::state` visibility to `pub(crate)` [\#604](https://github.com/iotaledger/identity.rs/pull/604)
- Remove JSON string escaping in diff messages [\#598](https://github.com/iotaledger/identity.rs/pull/598)

### Added

- Implement `FromIterator` for `OneOrMany` [\#602](https://github.com/iotaledger/identity.rs/pull/602)

### Patch

- Fix diff properties \(de\)serialization [\#611](https://github.com/iotaledger/identity.rs/pull/611)

## [v0.5.0-dev.2](https://github.com/iotaledger/identity.rs/tree/v0.5.0-dev.2) (2022-01-14)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.5.0-dev.1...v0.5.0-dev.2)

### Changed

- Replace `ClientMap` with `Client` in `Account` [\#582](https://github.com/iotaledger/identity.rs/pull/582)
- Add signature `created`, `expires`, `challenge`, `domain`, `purpose` [\#548](https://github.com/iotaledger/identity.rs/pull/548)

### Added

- Add account synchronization method [\#544](https://github.com/iotaledger/identity.rs/pull/544)

### Patch

- Enable local proof-of-work fallback [\#579](https://github.com/iotaledger/identity.rs/pull/579)

## [v0.5.0-dev.1](https://github.com/iotaledger/identity.rs/tree/v0.5.0-dev.1) (2021-12-15)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.4.0...v0.5.0-dev.1)
 
This release introduces multiple breaking changes to the structure of IOTA DID Documents and their Tangle messages, rendering any identity created with a prior version incompatible and unresolvable. A versioning system has been introduced so any new identities should hopefully be forward compatible with any future breaking changes to the message structure.

### Changed

- Refactor document metadata [\#540](https://github.com/iotaledger/identity.rs/pull/540)
- Replace `chrono` with `time` [\#529](https://github.com/iotaledger/identity.rs/pull/529)
- Enable access to the low-level API from the `Account` [\#522](https://github.com/iotaledger/identity.rs/pull/522)
- Update to `rsa` 0.5 in libjose [\#517](https://github.com/iotaledger/identity.rs/pull/517)
- Rename `DocumentDiff` to `DiffMessage` [\#511](https://github.com/iotaledger/identity.rs/pull/511)
- Deterministic ordering of competing messages [\#506](https://github.com/iotaledger/identity.rs/pull/506)
- Check for existence & duplication of methods in `CoreDocument` [\#504](https://github.com/iotaledger/identity.rs/pull/504)
- Move `dropsave` from `Account` to `Stronghold` [\#500](https://github.com/iotaledger/identity.rs/pull/500)
- Add `ExplorerUrl` to replace `Network` explorer methods [\#496](https://github.com/iotaledger/identity.rs/pull/496)
- Update `ServiceEndpoint` to support sets and maps [\#485](https://github.com/iotaledger/identity.rs/pull/485)
- Enable deep equality in `OrderedSet` [\#481](https://github.com/iotaledger/identity.rs/pull/481)
- Add message compression and versioning [\#466](https://github.com/iotaledger/identity.rs/pull/466)
- Update document signing key constraints and methods [\#458](https://github.com/iotaledger/identity.rs/pull/458)
- Refactor the `Account`: internal state, one identity [\#453](https://github.com/iotaledger/identity.rs/pull/453)

### Added

- Filter out DiffMessages updating signing methods [\#519](https://github.com/iotaledger/identity.rs/pull/519)
- Add publish with retry method [\#455](https://github.com/iotaledger/identity.rs/pull/455)

### Patch

- Add `identity-diff` derive feature gate [\#516](https://github.com/iotaledger/identity.rs/pull/516)
- Improve client error messages [\#512](https://github.com/iotaledger/identity.rs/pull/512)
- Make `create_signature` and `sign` async for `RemoteEd25519` [\#491](https://github.com/iotaledger/identity.rs/pull/491)
- Fix credential validation failing for documents with diff updates [\#490](https://github.com/iotaledger/identity.rs/pull/490)
- Upgrade to the Rust 2021 edition [\#449](https://github.com/iotaledger/identity.rs/pull/449)

## [v0.4.0](https://github.com/iotaledger/identity.rs/tree/v0.4.0) (2021-11-01)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.3.0...v0.4.0)

## [v0.3.0](https://github.com/iotaledger/identity.rs/tree/v0.3.0) (2021-05-10)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.2.0...v0.3.0)
 
This release introduces the high-level `Account` API for creating and managing IOTA identities.

## [v0.2.0](https://github.com/iotaledger/identity.rs/tree/v0.2.0) (2021-02-18)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/v0.1.0...v0.2.0)

## [v0.1.0](https://github.com/iotaledger/identity.rs/tree/v0.1.0) (2020-11-12)

[Full Changelog](https://github.com/iotaledger/identity.rs/compare/360bf5ce64a7f418249cdeadccb22b9aea7daeb6...v0.1.0)



\* *This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)*
