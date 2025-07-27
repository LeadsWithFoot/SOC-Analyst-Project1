Sigma Rules for Suricata-Based C2 Detection

This folder contains Sigma-format detection rules built on top of **Suricata alerts**, focused on identifying Command and Control (C2) activity, obfuscation, reconnaissance, and exfiltration attempts. Each rule maps directly to the MITRE ATT&CK framework and is designed to integrate into SIEM platforms that support Sigma.

## 🔍 Overview

These rules were developed based on Suricata alert signatures from a custom detection pipeline. They are intended to detect suspicious HTTP-based behavior often associated with malware, backdoors, or offensive tools during post-exploitation.

Each rule:
- Uses Suricata `eve.json` fields (via Filebeat or similar)
- Maps to relevant MITRE ATT&CK techniques
- Has clear descriptions and metadata
- Follows Sigma YAML schema

## 📄 Included Rules

| Rule Signature                                         | Suricata Signature                                      | MITRE Technique                         |
|--------------------------------------------------------|---------------------------------------------------------|------------------------------------------|
| Base64 in HTTP Body (Obfuscation)                      | `C2 Obfuscation - Base64 encoded string in HTTP body`   | T1027 – Obfuscated Files or Information  |
| `cmd` String in HTTP Response (Command Execution)      | `C2 Activity - Possible command in HTTP body`           | T1059 – Command and Scripting Interpreter|
| `whoami` Command in HTTP Body                          | `C2 Activity - whoami in HTTP body`                     | T1059 – Command and Scripting Interpreter|
| `uname` Output in HTTP Request (System Info)           | `C2 Exfiltration - uname output detected in HTTP body`  | T1071 – Application Layer Protocol       |
| UID/GID Recon Data in HTTP Request                     | `C2 Recon - UID/GID info returned`                      | T1082 – System Information Discovery     |
| PowerShell Script Download (.ps1)                      | `C2 Tool Transfer - Powershell script (.ps1) file download` | T1105 – Ingress Tool Transfer        |

