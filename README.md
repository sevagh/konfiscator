# konfiscator

konfiscator is simple implementation of malloc using mmap and a First-Fit Free List (based on [embeddedartistry's article](https://embeddedartistry.com/blog/2017/02/15/implementing-malloc-first-fit-free-list/)) with the Linux kernel's [doubly linked list](./src/list.h). The only APIs implemented are malloc and free.

The script `docker_ci.sh` launches a Docker container and uses the [LD_PRELOAD](https://www.gnu.org/software/libc/manual/html_node/Replacing-malloc.html) trick to launch a simple driver program, which mallocs an array of ints to print, and launches a blocking Prometheus metrics script (exit with ctrl-c), binding to the host's port 8000:

```
index 44: value 44
index 45: value 45
index 46: value 46
index 47: value 47
index 48: value 48
index 49: value 49
+ echo 'konfiscator test done, exporting metrics'
konfiscator test done, exporting metrics
+ /prometheus-metrics/konfiscator_metrics.py
starting up prometheus metrics endpoint
updating konfiscator stats
updating konfiscator stats
updating konfiscator stats
```

There's a fun trick of using a mmapped file, `/tmp/konfiscator-stats`, to write simple integer counters to, which is then read by a Python [Prometheus metrics exporter](./prometheus-metrics/konfiscator_metrics.py) that reads from the same mmapped file. The 4 metrics collected are counters for some of konfiscator's internals, browsable at http://localhost:8000:

```
# HELP konfiscator_malloc_total konfiscator malloc calls
# TYPE konfiscator_malloc_total gauge
konfiscator_malloc_total 2.0
# HELP konfiscator_mmap_total konfiscator mmap calls
# TYPE konfiscator_mmap_total gauge
konfiscator_mmap_total 0.0
# HELP konfiscator_free_total konfiscator free calls
# TYPE konfiscator_free_total gauge
konfiscator_free_total 1.0
# HELP konfiscator_defrag_total konfiscator defragged blocks
# TYPE konfiscator_defrag_total gauge
konfiscator_defrag_total 0.0
```

After joining the running container with `docker exec`, run `LD_PRELOAD=/libkonfiscator.so ./a.out` and reload the metrics endpoint to watch the stats get updated.

docker exec:

```
sevagh:konfiscator $ docker exec -it adoring_curran /bin/bash
[root@e5668ad09f0f /]# LD_PRELOAD=/libkonfiscator.so ./a.out
index 0: value 0
index 1: value 1
index 2: value 2
```

Updated metrics:

```
# HELP konfiscator_malloc_total konfiscator malloc calls
# TYPE konfiscator_malloc_total gauge
konfiscator_malloc_total 4.0
# HELP konfiscator_mmap_total konfiscator mmap calls
# TYPE konfiscator_mmap_total gauge
konfiscator_mmap_total 0.0
# HELP konfiscator_free_total konfiscator free calls
# TYPE konfiscator_free_total gauge
konfiscator_free_total 2.0
# HELP konfiscator_defrag_total konfiscator defragged blocks
# TYPE konfiscator_defrag_total gauge
konfiscator_defrag_total 0.0
```
