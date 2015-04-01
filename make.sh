cargo build
as head.S -o head.o
ld -O0 -T linker.ld --nostdlib -e_start head.o target/libvos-*.a -o vos
objcopy -O binary vos vos.bin
