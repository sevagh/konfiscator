#!/usr/bin/env python3

from prometheus_client import start_http_server, Gauge
import time
import mmap

KONFISCATOR_STATS_FILE = '/tmp/konfiscator-stats'


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

    print("starting up prometheus metrics endpoint")
    start_http_server(8000)

    with open(KONFISCATOR_STATS_FILE, 'r+') as f:
        mm = mmap.mmap(f.fileno(), 0)
        while True:
            print("updating konfiscator stats")
            mm.seek(0)
            for i in range(4):
                if i == 0:
                    konfiscator_mallocs.set(int(mm.readline()))
                elif i == 1:
                    konfiscator_mmaps.set(int(mm.readline()))
                elif i == 2:
                    konfiscator_frees.set(int(mm.readline()))
                elif i == 3:
                    konfiscator_defrags.set(int(mm.readline()))
            time.sleep(5)
        mm.close()
