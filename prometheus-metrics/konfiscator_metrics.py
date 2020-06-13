#!/usr/bin/env python3

from prometheus_client import start_http_server, Gauge
from ctypes import CDLL, Structure, c_int
import time

class KonfiscatorStats(Structure):
    _fields_ = [
        ('mallocs', c_int),
        ('frees', c_int),
        ('mmaps', c_int),
        ('defrags', c_int),
    ]

if __name__ == '__main__':
    konfiscator_mallocs = Gauge(
        "konfiscator_malloc_total",
        "konfiscator malloc calls",
    )
    konfiscator_mmaps = Gauge(
        "konfiscator_mmap_total",
        "konfiscator mmap calls",
    )
    konfiscator_frees = Gauge(
        "konfiscator_free_total",
        "konfiscator free calls",
    )
    konfiscator_defrags = Gauge(
        "konfiscator_defrag_total",
        "konfiscator defragged blocks",
    )

    konfiscator_lib = CDLL("/libkonfiscator.so")
    konfiscator_lib.konfiscator_get_stats.restype = KonfiscatorStats

    start_http_server(8000)

    while True:
        # recheck konfiscator stats, expose as metrics
        ks = konfiscator_lib.konfiscator_get_stats()
        konfiscator_mallocs.set(ks.mallocs)
        konfiscator_mmaps.set(ks.mmaps)
        konfiscator_frees.set(ks.frees)
        konfiscator_defrags.set(ks.defrags)
        time.sleep(5)
