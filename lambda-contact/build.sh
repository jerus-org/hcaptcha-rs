#!/bin/sh +eux
if docker run --rm \
    -u "$(id -u)":"$(id -g)" \
    -v "${PWD}":/code \
    -v "${HOME}"/.cargo/registry:/cargo/registry \
    -v "${HOME}"/.cargo/git:/cargo/git \
    -w /code \
    jerusdp/lambda-rust ; then
cp -f ./target/lambda/release/bootstrap.zip ./iac-lambda/rust.zip
exit 0
fi
exit 1

