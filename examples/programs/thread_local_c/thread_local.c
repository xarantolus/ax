#include <errno.h>
#include <stdio.h>
int main(int argc, char const *argv[])
{
	errno = 5;
	printf("errno address: %p\n", &errno);
	return 0;
}
