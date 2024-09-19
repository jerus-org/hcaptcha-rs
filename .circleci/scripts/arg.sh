#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <cargo-arg> <arg-value>"
    exit 1
fi

if [ -z "$2" ]; then
    exit 0
else
    echo "$1 $2"
fi
