#!/bin/bash

set -e

CTARGET="aarch64-unknown-linux-gnu"

cross build --target "$CTARGET" --release
