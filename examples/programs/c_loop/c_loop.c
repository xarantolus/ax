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

void _start()
{
	int a = 6;
	for (int i = 0; i < 3; i++)
	{
		a = a + 1;
	}

	sys_exit(a);
}
