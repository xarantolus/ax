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

void _start()
{
	uint64_t a = 0;
	uint64_t b = 1;

	int counter = 0;

	char buf[128];

	while (counter < 25)
	{
		int64_t c;
		if (__builtin_add_overflow(a, b, &c))
		{
			sys_write(2, "Overflow\n", 9);
			sys_exit(1);
		}
		a = b;
		b = c;
		counter++;

		int len = to_hex(buf, a);
		sys_write(1, buf, len);
	}

	sys_exit(0);
}
