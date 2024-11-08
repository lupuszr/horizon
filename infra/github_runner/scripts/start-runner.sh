#!/bin/bash
set -e

RUNNER_VERSION="2.314.1"

# Generate a unique runner name if not specified
if [ -z "$RUNNER_NAME" ]; then
    export RUNNER_NAME="${RUNNER_NAME_PREFIX}-$(hostname)-$(date +%s)"
fi

# Download and extract the runner if it's not already installed
if [ ! -f "${RUNNER_HOME}/config.sh" ]; then
    echo "Downloading runner..."
    curl -o "actions-runner-linux-x64-${RUNNER_VERSION}.tar.gz" -L "https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-x64-${RUNNER_VERSION}.tar.gz"
    tar xzf "./actions-runner-linux-x64-${RUNNER_VERSION}.tar.gz"
    rm "actions-runner-linux-x64-${RUNNER_VERSION}.tar.gz"
fi

# Get registration token
echo "Getting registration token..."
TOKEN=$(curl -s -X POST -H "Authorization: token ${GITHUB_PAT}" \
    "https://api.github.com/repos/${GITHUB_REPO}/actions/runners/registration-token" \
    | jq -r .token)

# Configure the runner
./config.sh \
    --unattended \
    --url "https://github.com/${GITHUB_REPO}" \
    --token "${TOKEN}" \
    --name "${RUNNER_NAME}" \
    --labels "${RUNNER_LABELS:-self-hosted}" \
    --replace

# Remove the token after configuration
unset TOKEN

# Start the runner
exec ./run.sh
