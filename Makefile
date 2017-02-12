# Makefile for ructiss operating system.
# kotetuco, 2016

ARCH=i386
TARGET_ARCH=i686-unknown-linux-gnu
BUILD_NAME=ructiss-$(ARCH)

BUILD_DIR=./build/$(BUILD_NAME)

# source directory path
SRC_BOOT=./arch/$(ARCH)/boot

UNAME := $(shell uname)

BUILD_MODE=debug

RUST_TARGET_PATH=${PWD}

# default
default:
	make image

# make image file
$(BUILD_DIR)/$(BUILD_NAME).img: $(BUILD_DIR)/ipl.bin $(BUILD_DIR)/$(BUILD_NAME).sys Makefile
	mformat -f 1440 -C -B $(BUILD_DIR)/ipl.bin -i $(BUILD_DIR)/$(BUILD_NAME).img ::
	mcopy -i $(BUILD_DIR)/$(BUILD_NAME).img $(BUILD_DIR)/$(BUILD_NAME).sys ::

$(BUILD_DIR)/$(BUILD_NAME).sys: $(BUILD_DIR)/kernel.bin  $(BUILD_DIR)/secondboot.bin
	cat $(BUILD_DIR)/secondboot.bin $(BUILD_DIR)/kernel.bin > $(BUILD_DIR)/$(BUILD_NAME).sys

$(BUILD_DIR)/kernel.bin:target/$(TARGET_ARCH)-rust/$(BUILD_MODE)/libructiss.a $(BUILD_DIR)/osfunc.o ./kernel/arch/$(TARGET_ARCH)/kernel.ld
	$(TARGET_ARCH)-ld --gc-sections -t -nostdlib -Tdata=0x00310000 -T ./kernel/arch/$(TARGET_ARCH)/kernel.ld -o $(BUILD_DIR)/kernel.bin $(BUILD_DIR)/osfunc.o --library-path=target/$(TARGET_ARCH)-rust/$(BUILD_MODE) -lructiss -Map $(BUILD_DIR)/kernel.map

# ipl (i386 only)
$(BUILD_DIR)/ipl.bin: ./kernel/arch/$(TARGET_ARCH)/asm/ipl.asm
	nasm -f bin -o $(BUILD_DIR)/ipl.bin ./kernel/arch/$(TARGET_ARCH)/asm/ipl.asm -l $(BUILD_DIR)/ipl.lst

# secondboot (i386 only)
$(BUILD_DIR)/secondboot.bin:./kernel/arch/$(TARGET_ARCH)/asm/secondboot.asm
	nasm -f bin -o $(BUILD_DIR)/secondboot.bin ./kernel/arch/$(TARGET_ARCH)/asm/secondboot.asm -l $(BUILD_DIR)/secondboot.lst

# kernel code
target/$(TARGET_ARCH)-rust/$(BUILD_MODE)/libructiss.a: $(TARGET_ARCH)-rust.json ./kernel/Cargo.toml ./kernel/src/*.rs
	rustup run nightly `which xargo` build -v --target=$(TARGET_ARCH)-rust --manifest-path kernel/Cargo.toml

$(BUILD_DIR)/%.o:./kernel/arch/$(TARGET_ARCH)/asm/%.asm
	nasm -f elf32 ./kernel/arch/$(TARGET_ARCH)/asm/$*.asm -o $(BUILD_DIR)/$*.o -l $(BUILD_DIR)/$*.lst

# build target
image:
	mkdir -p $(BUILD_DIR)
	make $(BUILD_DIR)/$(BUILD_NAME).img

run:
	make image
	make qemu

qemu:
	qemu-system-$(ARCH) -m 32 -localtime -vga std -fda $(BUILD_DIR)/$(BUILD_NAME).img -monitor stdio

tools:
	ifeq ($(UNAME),Darwin)
		brew install qemu
		brew install mtools	# mformat, mcopy
	endif

dump:
	od $(BUILD_DIR)/$(BUILD_NAME).img -t x1z -A x

clean:
	rm -f $(BUILD_DIR)/*.bin
	rm -f $(BUILD_DIR)/*.o
	rm -f $(BUILD_DIR)/*.a
	rm -f $(BUILD_DIR)/*.d
	rm -f $(BUILD_DIR)/*.rlib
	rm -f $(BUILD_DIR)/*.lst
	rm -rf $(BUILD_DIR)
	rm -rf ./build
	xargo clean --manifest-path ./kernel/Cargo.toml
