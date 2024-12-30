#!/usr/bin/env bash
# Script used by https://render.com/ on deployments.
echo "Running app"
node --version
npm --version
pwd
ls -la
ls -la app
ls -la app/dist
cargo run --release
