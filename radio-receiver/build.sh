#!/bin/bash

cross build --target aarch64-unknown-linux-gnu --release
echo Uploading
sshpass -p "1234qwer!" scp target/aarch64-unknown-linux-gnu/release/radio-receiver orange@10.20.1.45:/home/orange
echo Uploaded
