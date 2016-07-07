# Makefile for ructiss operating system.
# kotetuco, 2016

ARCH=i386
BUILD_NAME=ructiss-$(ARCH)

BUILD_DIR=./build/$(BUILD_NAME)

# source directory path
SRC_BOOT=./arch/$(ARCH)/boot

UNAME := $(shell uname)

# default
default:
	make image

# ipl (i386 only)
$(BUILD_DIR)/ipl.bin: $(SRC_BOOT)/ipl.asm
	nasm -f bin -o $(BUILD_DIR)/ipl.bin $(SRC_BOOT)/ipl.asm -l $(BUILD_DIR)/ipl.lst

$(BUILD_DIR)/$(BUILD_NAME).img: $(BUILD_DIR)/ipl.bin Makefile
	mformat -f 1440 -C -B $(BUILD_DIR)/ipl.bin -i $(BUILD_DIR)/$(BUILD_NAME).img ::

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
	rm -f $(BUILD_DIR)/*.lst
	rm -rf $(BUILD_DIR)
	rm -rf ./build
