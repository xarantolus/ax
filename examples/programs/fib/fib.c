#include <stdio.h>
#include <stdint.h>

// Print first 75 Fibonacci numbers
int main() {
	uint64_t a = 0;
	uint64_t b = 1;

	int counter = 0;

	while (counter < 75) {
		printf("%lu\n", a);
		int64_t c = a + b;
		a = b;
		b = c;
		counter++;
	}

	return 0;
}
