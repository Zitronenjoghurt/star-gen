# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cluster info window: Sortable table with properties of all stars
- Control settings window
- Camera will now automatically zoom into the selected star, orbiting around focused stars on a consistent radius (
  configurable in the control settings)

### Fixed

- Stars not generating on startup and trough generation options

### Changed

- Improved selected star window property table UI
- Increased the amount of compute threads used scrapping unused/idling I/O threads
- Reverted build opt-level z back to 3

[unreleased]: https://github.com/Zitronenjoghurt/star-gen/compare/v0.0.1...develop