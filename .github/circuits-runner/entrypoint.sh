#!/bin/bash

REPO_URL=${REPO_URL:-"https://github.com/your/repo.git"}
COMMIT_HASH=${COMMIT_HASH:-"your_commit_hash"}

git clone $REPO_URL .
git checkout $COMMIT_HASH

yarn install --frozen-lockfile && cd packages/circuits && yarn test --maxWorkers=75% --no-cache