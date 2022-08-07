#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
bash -c 'cd blogship_ui && ./run_dev.sh' & \
bash -c 'cd blogship_srv && ./run_dev.sh'
)

