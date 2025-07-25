# 🧪 Attacker Simulation Tools

This folder contains the custom attacker-side tools used to simulate command-and-control (C2) activity for detection testing.

### 🔧 Components

- `C2_File.rs` – A custom **Rust-based C2 beacon** that simulates a compromised host. It periodically sends HTTP requests to the C2 server to fetch commands, executes them, and sends the output back.
  
- `server.py` – A lightweight **Flask-based C2 server** that receives incoming requests from the beacon and delivers commands for execution.

- `send_commands.sh` – A **Bash script** that cycles through a series of pre-defined commands, using `curl` to POST them to the Python server. This emulates attacker input during beacon polling.

---

### 📡 How It Works

1. The Rust beacon (`C2_File.rs`) runs on the "compromised" machine.
2. It contacts the Flask server (`server.py`) at regular intervals to request a command.
3. The Flask server returns a command that was previously sent via the `rotate_tasks.sh` script.
4. The beacon executes the command and sends the response/output back to the server.
5. This activity is detected via custom Suricata and Sigma rules.

---

### ⚠️ Disclaimer

> These files are intended strictly for **educational and detection engineering purposes** only.  
> Do **not** deploy or distribute them in unauthorized or production environments.

---

### 📎 Related

- [Custom Suricata Rules](../suricata_rules/)
- [Detection Report (PDF)](../Detection_Report.pdf)
- [Sigma Rules](../sigma_rules/)
