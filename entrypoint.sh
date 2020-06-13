#!/usr/bin/env bash

set -eoxu pipefail

env

gcc /driver-program/malloc_test.c -o /a.out

echo "hello from real malloc"
LD_PRELOAD=/libkonfiscator.so echo "hello from konfiscator"
LD_PRELOAD=/libkonfiscator.so /a.out

echo "konfiscator test done, exporting metrics"

/prometheus-metrics/konfiscator_metrics.py
