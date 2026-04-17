# Macro Marathon

## PRACTICE

Use a macro to convert each line from "key: value" to "KEY=value".
Record once, replay for all remaining lines.

host: localhost
port: 8080
debug: true
log_level: info
max_retries: 3
timeout: 30
workers: 4
cache: enabled
mode: production
version: 2.1

## EXPECTED

Use a macro to convert each line from "key: value" to "KEY=value".
Record once, replay for all remaining lines.

HOST=localhost
PORT=8080
DEBUG=true
LOG_LEVEL=info
MAX_RETRIES=3
TIMEOUT=30
WORKERS=4
CACHE=enabled
MODE=production
VERSION=2.1
