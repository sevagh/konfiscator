#!/usr/bin/env bash

set -eoxu pipefail

container_name="konfiscator-test"

./compile.sh

docker build -t "${container_name}" .
docker run --rm "${container_name}"
