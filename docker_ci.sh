#!/usr/bin/env bash

USE_KONFISCATOR="yes"

DOCKER_RUN_ARGS=""

if [ "$USE_KONFISCATOR" == "yes" ]; then
	DOCKER_RUN_ARGS="-e LD_PRELOAD=/libkonfiscator.so"
fi

set -eoxu pipefail
container_name="konfiscator-test"

./compile.sh

docker build -t "${container_name}" .
docker run "$DOCKER_RUN_ARGS" --rm "${container_name}"
