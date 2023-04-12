#include <stdio.h>
#include <stdint.h>

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

int to_hex(char *buf, uint64_t num)
{
	int i = 0;

	if (num == 0)
	{
		buf[i] = '0';
		i++;
	}
	else
	{
		char stack[32] = {0};
		int stack_i = 0;

		while (num > 0)
		{
			uint64_t digit = num % 16;
			num /= 16;

			if (digit < 10)
			{
				stack[stack_i] = '0' + digit;
			}
			else
			{
				stack[stack_i] = 'a' + digit - 10;
			}

			stack_i++;
		}

		while (stack_i > 0)
		{
			stack_i--;
			buf[i] = stack[stack_i];
			i++;
		}
	}
	buf[i] = '\n';
	i++;
	buf[i] = '\0';

	return i;
}

void print_hex(int x)
{
	char buf[16];

	int len = to_hex(buf, x);
	sys_write(1, buf, len);
}

void _start()
{
	uint64_t a = 0;

	while (a < 20)
	{
		print_hex(a);

		a++;
	}

	sys_exit(0);
}
