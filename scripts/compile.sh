#!/usr/bin/env bash
PKG_CONFIG_ALLOW_CROSS=1 OPENSSL_DIR=$1 cargo build --target=arm-unknown-linux-gnueabihf