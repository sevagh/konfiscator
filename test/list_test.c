#include <stdarg.h>
#include <stddef.h>
#include <setjmp.h>
#include <cmocka.h>
#include "list.h"

static void ll_test_1(void **state)
{
	ll_t *list = ll_new();
	ll_t *a = ll_new();
	ll_t *b = ll_new();
	ll_set_next(list, a);
	ll_set_next(a, b);

	assert_true(ll_prev(b) == a);
	assert_true(ll_next(b) == NULL);
	assert_true(ll_next(a) == b);
}

int main(void)
{
	const struct CMUnitTest tests[] = {
		cmocka_unit_test(ll_test_1),
	};

	return cmocka_run_group_tests(tests, NULL, NULL);
}
