# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest (testnet) | ✅ |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

To report a vulnerability, open a [GitHub Security Advisory](../../security/advisories/new) on this repository. Include:

- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Any suggested mitigations

You can expect an acknowledgement within 48 hours and a resolution timeline within 7 days for critical issues.

## Scope

- `contracts/oracle-core` — Soroban smart contract (price manipulation, unauthorized access, storage exploits)
- `sdk/` — TypeScript SDK (key exposure, RPC injection)
- Publisher bot — secret key handling, API endpoint trust

## Out of Scope

- Issues in upstream dependencies (report to their maintainers)
- Theoretical attacks with no practical exploit path
