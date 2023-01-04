#define _GNU_SOURCE
#include <unistd.h>
#include <sys/syscall.h>
#include <sys/types.h>

int main(int argc, char const *argv[])
{
	syscall(SYS_write, 1, "Hello, World!\n", 14);
}
