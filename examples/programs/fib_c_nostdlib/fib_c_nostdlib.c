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

void to_hex(char *buf, uint64_t num)
{
	int i = 0;

	if (num == 0)
	{
		buf[i] = '0';
	}
	else
	{
		char stack[16] = {0};
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
	buf[i++] = '\n';
	buf[i] = '\0';
}

// Print first 75 Fibonacci numbers
void _start()
{
	uint64_t a = 0;
	uint64_t b = 1;

	int counter = 0;

	char buf[128];

	while (counter < 75)
	{
		int64_t c = a + b;
		a = b;
		b = c;
		counter++;

		to_hex(buf, a);
		sys_write(1, buf, 128);
	}

	sys_exit(0);
}
