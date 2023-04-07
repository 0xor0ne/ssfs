#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
ASSETS_DIR=${SCRIPT_DIR}/../assets


openssl req -x509 -nodes -newkey rsa:4096 \
    -keyout ${ASSETS_DIR}/key.pem \
    -out ${ASSETS_DIR}/cert.pem \
    -days 365000 \
    -subj "/C=XX/ST=California/CN=sshttps"

