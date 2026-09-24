#include <stdio.h>
#include <stdint.h>

uint64_t add(uint64_t left, uint64_t right);

int main(int argc, char const *argv[])
{
	(void)argc;
	(void)argv;

	const char *hello = "Hello world";

	uint64_t result = add(12345, 1000000000000);

	printf("Test \"%s\", result: %lu\n", hello, result);

	return 0;
}
