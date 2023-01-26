#include <syscall.h>

#define SYSCALL_EXIT 60
#define SYSCALL_WRITE 1

void sys_exit(int error_code)
{
	asm volatile(
		"syscall"
		:
		: "a"(SYSCALL_EXIT), "D"(error_code)
		: "rcx", "r11", "memory");
}

int sys_write(unsigned fd, const char *buf, unsigned count)
{
	unsigned ret;

	asm volatile(
		"syscall"
		: "=a"(ret)
		: "a"(SYSCALL_WRITE), "D"(fd), "S"(buf), "d"(count)
		: "rcx", "r11", "memory");

	return ret;
}



void _start()
{
	sys_write(1, "Hello, World!\n", 14);
	sys_exit(0);
}
