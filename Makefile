# Makefile for ructiss operating system.
# kotetuco, 2016

ARCH=i386
TARGET_ARCH=i686-unknown-linux-gnu
BUILD_NAME=ructiss-$(ARCH)

BUILD_DIR=./build/$(BUILD_NAME)

# source directory path
SRC_BOOT=./arch/$(ARCH)/boot

UNAME := $(shell uname)

# default
default:
	make image

# make image file
$(BUILD_DIR)/$(BUILD_NAME).img: $(BUILD_DIR)/ipl.bin $(BUILD_DIR)/$(BUILD_NAME).sys Makefile
	mformat -f 1440 -C -B $(BUILD_DIR)/ipl.bin -i $(BUILD_DIR)/$(BUILD_NAME).img ::
	mcopy -i $(BUILD_DIR)/$(BUILD_NAME).img $(BUILD_DIR)/$(BUILD_NAME).sys ::

$(BUILD_DIR)/$(BUILD_NAME).sys: $(BUILD_DIR)/kernel.bin  $(BUILD_DIR)/secondboot.bin
	cat $(BUILD_DIR)/secondboot.bin $(BUILD_DIR)/kernel.bin > $(BUILD_DIR)/$(BUILD_NAME).sys

$(BUILD_DIR)/kernel.bin:$(BUILD_DIR)/init_os.o
	$(TARGET_ARCH)-ld -v -nostdlib -Tdata=0x00310000 $(BUILD_DIR)/init_os.o -T kernel.ld -o $(BUILD_DIR)/kernel.bin -Map $(BUILD_DIR)/kernel.map

# ipl (i386 only)
$(BUILD_DIR)/ipl.bin: $(SRC_BOOT)/ipl.asm
	nasm -f bin -o $(BUILD_DIR)/ipl.bin $(SRC_BOOT)/ipl.asm -l $(BUILD_DIR)/ipl.lst

# secondboot (i386 only)
$(BUILD_DIR)/secondboot.bin:$(SRC_BOOT)/secondboot.asm
	nasm -f bin -o $(BUILD_DIR)/secondboot.bin $(SRC_BOOT)/secondboot.asm -l $(BUILD_DIR)/secondboot.lst

# libcore (For Rust code)
$(BUILD_DIR)/libcore.rlib:./rust-lang/src/libcore/lib.rs
	rustc --verbose --target=i686-unknown-linux-gnu --crate-type=rlib --emit=link,dep-info -C opt-level=2 -C no-prepopulate-passes -C no-stack-check -Z no-landing-pads -o $(BUILD_DIR)/libcore.rlib ./rust-lang/src/libcore/lib.rs

# kernel code
$(BUILD_DIR)/%.o:$(SRC_BOOT)/%.rs $(BUILD_DIR)/libcore.rlib
	rustc --target=i686-unknown-linux-gnu --crate-type=staticlib --emit=obj -C lto -C opt-level=2 -C no-prepopulate-passes -C no-stack-check -Z verbose -Z no-landing-pads -o $@ $< --extern core=$(BUILD_DIR)/libcore.rlib

# build target
image:
	mkdir -p $(BUILD_DIR)
	make $(BUILD_DIR)/$(BUILD_NAME).img

run:
	make image
	qemu-system-$(ARCH) -m 32 -localtime -vga std  -fda $(BUILD_DIR)/$(BUILD_NAME).img

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
