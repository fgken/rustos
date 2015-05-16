void console_out(unsigned char c)
{
	asm("mov %0, %%al" : : "a"(c));
	asm("mov $0x3f8, %dx");
	asm("out %al, %dx");
}

void test_main()
{
	console_out('A');
}
