#!/usr/bin/env bash

set -eoxu pipefail

container_name="konfiscator-test"

./compile.sh

docker build -t "${container_name}" .
docker run --rm -it -p 8000:8000 --rm "${container_name}"
