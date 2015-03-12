CC='gcc'
CFLAGS='-c -nostdlib -fno-asynchronous-unwind-tables -fno-builtin -fno-common'
RUSTC='rustc'
LD='ld'
ARCH='x86_64-unknown-linux-gnu'

task :default => 'vos.elf'

file 'vos.elf' => ['main.o', 'serial.o'] do
	sh "#{LD} -o vos.elf -T linker.ld main.o serial.o"
end

file 'main.o' => 'main.rs' do
	sh "#{RUSTC} -o main.o -O --target #{ARCH} --crate-type lib --emit obj -C target-cpu=generic main.rs"
end

file 'serial.o' => 'serial.c' do
	sh "#{CC} -o serial.o #{CFLAGS} serial.c"
end


task :clean do
	sh 'rm *.o *.elf'
end

#$(NASM) -f elf64 -o $@ $<

