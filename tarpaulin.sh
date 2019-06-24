#!/bin/bash
time \
  docker run \
    --security-opt seccomp=unconfined \
    -v "$PWD:/volume" \
    xd009642/tarpaulin \
        sh -c '\
            apt-get update \
            && apt-get install -y \
                zlib1g-dev \
                libsdl2-dev \
            && cargo tarpaulin'
