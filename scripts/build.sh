#!/usr/bin/env bash
# Script used by https://render.com/ on deployments.

echo "Building client"
(
  cd app
  npm ci
  npm run build
)

echo "Building server"
cargo build --release
