#include <errno.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/mman.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include "konfiscator.h"
#include "list.h"

// https://www.cs.yale.edu/homes/aspnes/pinewiki/C(2f)Persistence.html
#define NUM_COUNTERS (4)
#define COUNTER_FILE "/tmp/konfiscator-stats"
#define NEW_COUNTER_FILE COUNTER_FILE "~"

#define ALLOC_HEADER_SZ offsetof(alloc_node_t, block)
#define MIN_ALLOC_SZ ALLOC_HEADER_SZ + 32
#define FIRST_BLOCK_SIZE 1024*1024

#ifndef align_up
#define align_up(num, align) (((num) + ((align)-1)) & ~((align)-1))
#endif

static LIST_HEAD(free_list);

// forward declarations
void defrag_free_list(void);
void increment_metric(int);

typedef struct {
  struct list_head node;
  size_t size;
  char *block;
} alloc_node_t;

enum metric_index { mallocs, frees, defrags, mmaps };

void *malloc1(size_t size) {
  if (list_empty(&free_list)) {
    void *addr = mmap((void *)0, FIRST_BLOCK_SIZE, PROT_READ + PROT_WRITE,
                      MAP_ANONYMOUS + MAP_SHARED, -1, 0);
    increment_metric(mmaps);

    // not doing munmap cause apparently it's done automatically
    if (addr == MAP_FAILED) {
      return NULL;
    }

    alloc_node_t *first_blk;
    first_blk = (void *)align_up((uintptr_t)addr, sizeof(void *));
    first_blk->size = FIRST_BLOCK_SIZE;
    list_add(&first_blk->node, &free_list);
  }

  void *ptr = NULL;
  alloc_node_t *blk = NULL;

  // try to find a big enough block to alloc
  list_for_each_entry(blk, &free_list, node) {
    if (blk->size >= size) {
      ptr = &blk->block;
      break;
    }
  }

  if ((blk->size - size) >= MIN_ALLOC_SZ) {
    alloc_node_t *new_blk;
    new_blk = (alloc_node_t *)((uintptr_t)(&blk->block) + size);
    new_blk->size = blk->size - size - ALLOC_HEADER_SZ;
    blk->size = size;
    list_add(&new_blk->node, &blk->node);
  }

  list_del(&blk->node);

  increment_metric(mallocs);
  return ptr;
}

void free(void *ptr) {
  if (!ptr) {
    return;
  }
  increment_metric(frees);

  alloc_node_t *blk, *free_blk;

  // we take the pointer and use container_of to get the corresponding block
  blk = container_of(ptr, alloc_node_t, block);

  // Let's put it back in the proper memory order
  list_for_each_entry(free_blk, &free_list, node) {
    if (free_blk > blk) {
      list_add_tail(&blk->node, &free_blk->node);
      goto blockadded; // break out
    }
  }

  // if we didn't break, add to the end of the list
  list_add_tail(&blk->node, &free_list);

blockadded:
  // Let's see if we can combine any memory
  defrag_free_list();
}

void defrag_free_list(void) {
  alloc_node_t *block, *last_block = NULL, *t;

  increment_metric(defrags);

  list_for_each_entry_safe(block, t, &free_list, node) {
    if (last_block) {
      if ((((uintptr_t)&last_block->block) + last_block->size) ==
          (uintptr_t)block) {
        last_block->size += sizeof(*block) + block->size;
        list_del(&block->node);
        continue;
      }
    }
    last_block = block;
  }
}

void increment_metric(int metric_index) {
    /* open and map the file */
    int *counts;
    int fd = open(COUNTER_FILE, O_RDWR);
    if (fd < 0) {
	    return;
    }

    // don't need PROT_READ, want to share it
    counts = mmap(0, sizeof(*counts) * NUM_COUNTERS, PROT_WRITE, MAP_SHARED, fd, 0);

    if (counts == 0) {
	    return;
    }

    ++counts[metric_index];

    munmap(counts, sizeof(*counts) * NUM_COUNTERS);
    close(fd);
}
