
void serial_out()
{
	asm volatile("outb %b0, %w1" : : "a"('A'), "Nd"(0x3f8));
}

