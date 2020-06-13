#ifndef KONFISCATOR_STATS_H
#define KONFISCATOR_STATS_H

#include <stddef.h>

typedef struct konfiscator_stats {
	int mmaps;
	int mallocs;
	int frees;
	int defrags;
} konfiscator_stats_t;

konfiscator_stats_t konfiscator_get_stats(void);

#endif /* KONFISCATOR_STATS_H */
