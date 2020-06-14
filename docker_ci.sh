#!/usr/bin/env bash

set -eoxu pipefail

container_name="konfiscator-test"

./compile.sh

docker build -t "${container_name}" .
docker run --security-opt seccomp=unconfined --rm -it -p 8000:8000 "${container_name}"
