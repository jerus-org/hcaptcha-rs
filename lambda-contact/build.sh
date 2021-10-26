#!/bin/sh -eux

cd ..


if docker run --rm \
    -u "$(id -u)":"$(id -g)" \
    -v "${PWD}":/code \
    -v "${HOME}"/.cargo/registry:/cargo/registry \
    -v "${HOME}"/.cargo/git:/cargo/git \
    -w /code/lambda-contact \
    jerusdp/lambda-rust:beta; then
cp -f ./target/lambda/release/bootstrap.zip ./iac-lambda/rust.zip
cd lambda-contact || return
exit 0
fi
cd lambda-contact || return
exit 1

