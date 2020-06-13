#include "konfiscator.h"

// use the linux kernel doubly linked list
#include "list.h"

// for uintptr_t
#include <stdint.h>

#define ALLOC_HEADER_SZ offsetof(alloc_node_t, block)
#define MIN_ALLOC_SZ ALLOC_HEADER_SZ + 32

/**
* Simple macro for making sure memory addresses are aligned
* to the nearest power of two
*/
#ifndef align_up
#define align_up(num, align) \
	(((num) + ((align) - 1)) & ~((align) - 1))
#endif

static LIST_HEAD(free_list);

// forward declarations
void defrag_free_list(void);

typedef struct {
     struct list_head node;
     size_t size;
     char * block;
} alloc_node_t;

void malloc_addblock(void *addr, size_t size)
{
     alloc_node_t *blk;

     // align the start addr of our block to the next pointer aligned addr
     blk = (void *) align_up((uintptr_t)addr, sizeof(void*));

     // calculate actual size - mgmt overhead
     blk->size = (uintptr_t) addr + size - (uintptr_t) blk 
        - ALLOC_HEADER_SZ;

     //and now our giant block of memory is added to the list!
     list_add(&blk->node, &free_list);
}

void *malloc(size_t size) {
	void * ptr = NULL;
	alloc_node_t *blk = NULL;

	// try to find a big enough block to alloc
	list_for_each_entry(blk, &free_list, node)
	{
	    if(blk->size >= size)
	    {
		ptr = &blk->block;
		break;
	    }
	}

	if((blk->size - size) >= MIN_ALLOC_SZ)
	{
	    alloc_node_t *new_blk;
	    new_blk = (alloc_node_t *)((uintptr_t)(&blk->block) + size);
	    new_blk->size = blk->size - size - ALLOC_HEADER_SZ;
	    blk->size = size;
	    list_add(&new_blk->node, &blk->node);
	}

	list_del(&blk->node);

	return ptr;
}

void free(void *ptr)
{
	if(!ptr)
		return;

	alloc_node_t *blk, *free_blk;

	// we take the pointer and use container_of to get the corresponding block
	blk = container_of(ptr, alloc_node_t, block);

	//Let's put it back in the proper memory order
	list_for_each_entry(free_blk, &free_list, node)
	{
	   if(free_blk > blk)
	   {
		list_add_tail(&blk->node, &free_blk->node);
		goto blockadded; //break out
	   }
	}

	// if we didn't break, add to the end of the list
	list_add_tail(&blk->node, &free_list);

blockadded:
    // Let's see if we can combine any memory
    defrag_free_list();
}

void defrag_free_list(void)
{
	alloc_node_t *block, *last_block = NULL, *t;

	list_for_each_entry_safe(block, t, &free_list, node)
	{
		if(last_block)
		{
			if((((uintptr_t)&last_block->block) + last_block->size)
				== (uintptr_t)block)
			{
				last_block->size += sizeof(*block) + block->size;
				list_del(&block->node);
				continue;
			}
		}
		last_block = block;
	}
}
