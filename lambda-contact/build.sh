#!/bin/sh
docker run --rm \
    -u $(id -u):$(id -g) \
    -v ${PWD}:/code \
    -v ${HOME}/.cargo/registry:/cargo/registry \
    -v ${HOME}/.cargo/git:/cargo/git \
    -w /code \
    jerusdp/lambda-rust

if [ $? -ne 0 ]; then
    exit 1
fi

cp -f ./target/lambda/release/bootstrap.zip ./iac-lambda/rust.zip
