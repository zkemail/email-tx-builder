#!/bin/bash

REPO_URL=${REPO_URL:-"https://github.com/your/repo.git"}
COMMIT_HASH=${COMMIT_HASH:-"your_commit_hash"}

echo "Cloning repository: $REPO_URL"
git clone $REPO_URL repo
cd repo

echo "Checking out commit: $COMMIT_HASH"
git checkout $COMMIT_HASH

echo "Installing dependencies"
yarn install --frozen-lockfile

echo "Running tests"
cd packages/circuits
yarn test-large --no-cache