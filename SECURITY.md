# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please follow responsible disclosure practices.

### How to Report

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. Email security concerns to: [security@example.com]
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- Acknowledgment within 48 hours
- Regular updates on progress
- Credit in security advisory (if desired)
- Fix timeline based on severity

### Severity Levels

| Level | Response Time | Examples |
|-------|---------------|----------|
| Critical | 24 hours | Remote code execution, data breach |
| High | 72 hours | Privilege escalation, auth bypass |
| Medium | 1 week | XSS, CSRF, information disclosure |
| Low | 2 weeks | Minor issues, hardening suggestions |

## Security Measures

### Application Security

- Tauri's security model with strict CSP
- Input validation and sanitization
- Secure file handling for .idkit files
- No network requests without user consent

### Data Handling

- Composites stored locally only
- No telemetry or analytics
- No external service dependencies
- User data never leaves the device

## Best Practices for Users

1. Download only from official releases
2. Verify checksums when available
3. Keep the application updated
4. Report suspicious behavior

---

Thank you for helping keep Identikit secure.
