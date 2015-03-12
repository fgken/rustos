
void serial_out(unsigned char c)
{
	asm volatile("outb %b0, %w1" : : "a"(c), "Nd"(0x3f8));
}

