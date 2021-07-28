#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@192.168.1.201
readonly TARGET_PATH=./rust_binaries/
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/pi_motor_control

cargo build --release --target=${TARGET_ARCH}

scp ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
