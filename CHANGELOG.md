# Changelog

All notable changes to this project will be documented in this file.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added
- **Security**: Implemented **Circuit Breaker** (max deviation limits) in `submit_price` to prevent price manipulation.
- **Docs**: Comprehensive documentation overhaul with premium branding, ecosystem diagrams, and technical deep-dives.
- **SDK**: Added `OraclePublisher` and `OracleConsumer` classes for seamless integration.
- **Infrastructure**: Automated publisher bot with multi-asset support (XLM, BTC, ETH).
- **Architecture**: Hybrid storage model utilizing `Instance` and `Temporary` storage for gas efficiency.

### Fixed
- Fixed TTL extension logic for publisher entries to ensure reliable ~7-day persistence.
- Improved error handling for unauthorized publisher submissions.

### Deployment
- **Testnet**: `CA76KLJ2CDD5OHVGD6MUV3QVZYRNJJQLIHBMWD353J6ES4JZXCO4L5OQ`
