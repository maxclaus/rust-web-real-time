#!/usr/bin/env bash
# Script used by https://render.com/ on deployments.
echo "Starting build"
pwd
ls -la
node --version
npm --version
(
  cd app
  npm ci
  npm run build
)
pwd
ls -la
ls -la app
ls -la app/dist
cargo build --release
