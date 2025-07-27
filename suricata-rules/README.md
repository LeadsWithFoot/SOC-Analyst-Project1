Suricata Rules

This folder contains custom Suricata IDS rules designed to detect Command and Control (C2) activity, obfuscation, reconnaissance, and exfiltration attempts over HTTP. Each rule is mapped to the MITRE ATT&CK framework for context and threat alignment.

## Included Rules

- **Execution via `cmd`**
  - Detects possible command execution in HTTP body.
  - `Technique: T1059 – Command and Scripting Interpreter`
  - `SID: 1002002`

- **Execution via `whoami`**
  - Identifies `whoami` output returned to an attacker.
  - `Technique: T1059 – Command and Scripting Interpreter`
  - `SID: 1003211`

- **Recon via UID/GID**
  - Flags system information (UID/GID) exfiltrated in HTTP request.
  - `Technique: T1082 – System Information Discovery`
  - `SID: 1003222`

- **Base64 Obfuscation**
  - Detects encoded strings in HTTP response bodies.
  - `Technique: T1027 – Obfuscated Files or Information`
  - `SID: 1000019`

- **PowerShell Script Download**
  - Detects `.ps1` file transfers over HTTP.
  - `Technique: T1105 – Ingress Tool Transfer`
  - `SID: 1003401`

- **System Info Exfiltration**
  - Looks for specific `uname` output in HTTP request body (e.g., `Linux`, kernel version).
  - `Technique: T1071 – Application Layer Protocol`
  - `SID: 1000044`

## Usage

These rules are intended for use in Suricata's IDS mode. Add them to your custom rules directory and reference them in your `suricata.yaml` config.

```bash
systemctl start suricata
