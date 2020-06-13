#!/usr/bin/env python3

from prometheus_client import start_http_server, Gauge
from ctypes import CDLL
import time


if __name__ == '__main__':
    konfiscator_mallocs = Gauge(
        "konfiscator_malloc_total",
        "konfiscator malloc calls",
    )
    konfiscator_mmmaps = Gauge(
        "konfiscator_mmap_total",
        "konfiscator mmap calls",
    )
    konfiscator_frees = Gauge(
        "konfiscator_free_total",
        "konfiscator free calls",
    )
    konfiscator_frees = Gauge(
        "konfiscator_defrag_total",
        "konfiscator defragged blocks",
    )

    konfiscator_lib = CDLL("/libkonfiscator.so")

    start_http_server(8000)

    while True:
        # recheck konfiscator stats, expose as metrics
        print(konfiscator_lib.konfiscator_get_stats())
        time.sleep(5)
