MITRE Detection Report

This folder contains a detailed detection report aligned with the MITRE ATT&CK framework. The report covers observed techniques, detection strategies, and corresponding visualizations from Kibana.

## Contents
- `detection-report.pdf` — Full report with explanation of detection rules, attack simulations, and dashboard analysis.
- `screenshots/` — Visuals from dashboards (Threat Detection, MITRE Coverage, Network Monitoring).

## Summary
This report demonstrates how detection rules and visualizations can be used to map real-world threats to MITRE ATT&CK techniques. It includes:
- Custom-built rules for detecting C2 traffic, obfuscation, and persistence techniques.
- Dashboard coverage of key tactics.
- Mapping to ATT&CK IDs like T1027 (Obfuscated Files) and T1071 (Application Layer Protocol). [`suricata-rules/`](../suricata-rules)

## Tools Used
- Suricata
- Filebeat + Elasticsearch + Kibana
- MITRE ATT&CK Framework
- Rust, Python, Bash
- Flask
