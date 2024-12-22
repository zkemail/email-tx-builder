#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/.env"
USER="${SENDER_EMAIL}:${SENDER_PASSWORD}"
UPLOAD_FILE="${SCRIPT_DIR}/invalid_email_with_non_utf8.txt"

# curlコマンドを実行
curl -vvv --ssl-reqd \
  --url 'smtp://smtp.gmail.com:587' \
  --user "$USER" \
  --mail-from "$SENDER_EMAIL" \
  --mail-rcpt "$RECIPIENT_EMAIL" \
  --upload-file "$UPLOAD_FILE"