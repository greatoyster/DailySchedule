
target := riscv64imac-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64
.PHONY: kernel build clean qemu run env

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview rustfmt
	rustup target add $(target)

kernel:
	cargo build

$(bin): kernel
	rust-objcopy target/riscv64imac-unknown-none-elf/debug/os \
	--strip-all -O binary \
	target/riscv64imac-unknown-none-elf/debug/kernel.bin
asm:
	$(objdump) -d $(kernel) | less

build: $(bin)
	
clean:
	cargo clean
qemu:
	qemu-system-riscv64 \
  -machine virt \
	-nographic \
	-bios default \
	-device loader,file=$(bin),addr=0x80200000
run: build qemu
