task :default do
	puts "rake build"
	puts "rake run"
end

# -------------------------
# --- rake build ---
# -------------------------
BUILDDIR='bin/'

directory BUILDDIR
task :build => [BUILDDIR] do
	sh 'cargo build'
	sh "as head.S -o #{BUILDDIR}head.o"
	sh "ld -O0 -T linker.ld --nostdlib -e_start #{BUILDDIR}head.o target/libvos-*.a -o #{BUILDDIR}vos"
	sh "objcopy -O binary #{BUILDDIR}vos #{BUILDDIR}vos.bin"
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

