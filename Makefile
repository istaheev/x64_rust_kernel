.PHONY: all prepare rust image clean

ARCH = x86_64
RUST_TARGET = $(ARCH)-unknown-linux-gnu
CONFIG = debug

# Input files
INCLUDE = $(wildcard src/arch/$(ARCH)/*.h)
INCLUDE_DIR = src/arch/$(ARCH)

ASM_SRC = $(wildcard src/arch/$(ARCH)/*.S)
ASM_OBJS = $(patsubst src/arch/$(ARCH)/%.S,build/arch/$(ARCH)/%.o,$(ASM_SRC))

RUST_KERNEL = target/$(RUST_TARGET)/$(CONFIG)/libx64_rust_kernel.a

LINKER_LD = src/arch/$(ARCH)/linker.ld

# grub2-mkrescue for centos
GRUBMKRESCUE = grub-mkrescue

# Compilation flags

CC = gcc
CFLAGS = -g                   \
	     -pedantic            \
		 -Wall                \
		 -Wextra

LDFLAGS = -n                      \
	      -nostdlib               \
	      --gc-sections           \
		  -z max-page-size=0x1000

CARGOFLAGS.debug =
CARGOFLAGS.release = --release
CARGOFLAGS = --target $(RUST_TARGET) ${CARGOFLAGS.${CONFIG}}
# Rules

image: build/kernel image/boot/grub/grub.cfg
	rm -rf build/image
	cp -R image build/image
	cp build/kernel build/image/kernel
	$(GRUBMKRESCUE) -o build/image.iso build/image

# Link the kernel
build/kernel: prepare rust Makefile $(LINKER_LD) $(ASM_OBJS)
	ld $(LDFLAGS) -T $(LINKER_LD) -Map build/kernel.map -o build/kernel $(ASM_OBJS) $(RUST_KERNEL)

# Compile assembler files
$(ASM_OBJS): $(ASM_SRC)
	$(CC) $(CFLAGS) -I$(INCLUDE_DIR) -c $< -o $@

rust:
	cargo rustc $(CARGOFLAGS) -- -Z no-landing-pads -C target-feature=-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2

prepare:
	mkdir -p build/arch/$(ARCH)

clean:
	cargo clean
	-rm -rf build

all: clean prepare image

dump: bin/kernel
	objdump -D bin/kernel
