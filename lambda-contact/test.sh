#!/bin/sh

    # verify packaged artifact by invoking it using the lambdaci "provided.al2" docker image
    rm -rf /tmp/lambda > /dev/null 2>&1
    unzip -o \
        target/lambda/release/bootstrap.zip \
        -d /tmp/lambda > /dev/null 2>&1 && \
    docker run \
        -i -e DOCKER_LAMBDA_USE_STDIN=1 \
        --rm \
        -v /tmp/lambda:/var/task \
        lambci/lambda:provided.al2 < test-event.json | grep -v RequestId | grep -v '^\W*$' > test-out.log
