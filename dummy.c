#include <stddef.h>

int __errno;
int __stack_chk_fail;
int _Unwind_Resume;

void *memset(void *s, int c, size_t n)
{
	return (void *)0;
}

void *memcpy(void *dest, const void *src, size_t n)
{
	return (void *)0;
}

int memcmp(const void *s1, const void *s2, size_t n)
{
	return 0;
}
