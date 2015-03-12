OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(main)

SECTIONS {
    /*. = 0x7e00;*/
    . = 0x4000;
	.bss : { *(.bss) }

	. = 0x7c00;
    .text : { *(.text) }

	. = 0xA000;
	.data : { *(.data) }

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
    }
}
