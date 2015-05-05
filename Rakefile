task :default do
	puts "rake build"
	puts "rake run"
end

# -------------------------
# --- rake build ---
# -------------------------
BUILDDIR='bin/'

directory BUILDDIR
task :build => [BUILDDIR, 'bin/libc.a', 'bin/dummy.o'] do
	sh 'cargo build'
	sh "as head.S -o #{BUILDDIR}head.o"
	sh "ld -O0 -T linker.ld -nostdlib -e_start #{BUILDDIR}head.o target/libvos-*.a #{BUILDDIR}libc.a #{BUILDDIR}dummy.o -o #{BUILDDIR}vos"
	sh "objcopy -O binary #{BUILDDIR}vos #{BUILDDIR}vos.bin"
end

file BUILDDIR+'libc.a' do
	sh 'wget ftp://sourceware.org/pub/newlib/newlib-2.2.0-1.tar.gz'
	sh 'tar xf newlib-2.2.0-1.tar.gz'
	rm 'newlib-2.2.0-1.tar.gz'
	cd 'newlib-2.2.0-1/newlib/' do
		sh './configure'
		sh 'make -j 3'
	end
	cp 'newlib-2.2.0-1/newlib/libm.a', 'bin/libc.a'
end

file BUILDDIR+'dummy.o' => ['dummy.c'] do
	sh "gcc -c -nostdlib dummy.c -o #{BUILDDIR}dummy.o"
end

# -------------------------
# --- rake run ---
# -------------------------
RUNDIR='run-dir/'
UEFIDIR='image/EFI/BOOT/'

task :run => [:build, RUNDIR+'OVMF.fd', RUNDIR+UEFIDIR+'UefiBootLoader.efi'] do
	cp BUILDDIR+'vos', RUNDIR+'image/'
	sh 'qemu-system-x86_64 -nographic -bios OVMF.fd -hda fat:image -net none', {chdir: RUNDIR}, {}
end

directory RUNDIR
file RUNDIR+'OVMF.fd' => [RUNDIR] do
	cd RUNDIR do
		sh 'wget http://sourceforge.net/projects/edk2/files/OVMF/OVMF-X64-r15214.zip'
		sh 'unzip OVMF-X64-r15214.zip OVMF.fd'
		sh 'rm OVMF-X64-r15214.zip'
	end
end

directory RUNDIR+UEFIDIR
file RUNDIR+UEFIDIR+'UefiBootLoader.efi' => [RUNDIR+UEFIDIR] do
	cd RUNDIR+UEFIDIR do
		sh 'wget https://github.com/fgken/uefi-bootloader/releases/download/v1.0.0/UefiBootLoader.efi'
	end
end

# -------------------------
# --- rake clean ---
# -------------------------
task :clean do
	rm 'Cargo.lock'
	rm_r 'bin/'
	rm_r 'run-dir/'
	rm_r 'newlib-2.2.0-1/'
end
