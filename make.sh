cargo build
as head.S -o head.o
ld -T linker.ld --nostdlib -e_start head.o target/libvos-*.a -o vos
