LD='ld'
RUSTC='rustc'
ARCH='x86_64-unknown-linux-gnu'

task :default => 'vos.elf'

file 'vos.elf' => ['main.o'] do
	sh "#{LD} -o vos.elf -T linker.ld main.o"
end

file 'main.o' =>'main.rs' do
	sh "#{RUSTC} -o main.o -O --target #{ARCH} --crate-type lib --emit obj -C target-cpu=generic main.rs"
end


task :clean do
	sh 'rm *.o *.elf'
end

#$(NASM) -f elf64 -o $@ $<

