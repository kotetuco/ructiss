# Makefile for ructiss operating system.
# kotetuco, 2016

# i686-unknown-linux-gnu
QEMU_ARCH_i686 := i386
TARGET_ARCH_i686 := i686-unknown-linux-gnu
BUILD_NAME_i686 := ructiss-$(TARGET_ARCH_i686)
BUILD_DIR_i686 := ./build/$(BUILD_NAME_i686)

# arm-none-eabi
TARGET_ARCH_GBA := arm-none-eabi
BUILD_NAME_GBA := ructiss-$(TARGET_ARCH_GBA)
BUILD_DIR_GBA := ./build/$(BUILD_NAME_GBA)

UNAME := $(shell uname)

BUILD_MODE=debug

# default target
default:
	make i686

#
# i686-unknown-linux-gnu
#

# make image file
$(BUILD_DIR_i686)/$(BUILD_NAME_i686).img: $(BUILD_DIR_i686)/ipl.bin $(BUILD_DIR_i686)/$(BUILD_NAME_i686).sys Makefile
	mformat -f 1440 -C -B $(BUILD_DIR_i686)/ipl.bin -i $(BUILD_DIR_i686)/$(BUILD_NAME_i686).img ::
	mcopy -i $(BUILD_DIR_i686)/$(BUILD_NAME_i686).img $(BUILD_DIR_i686)/$(BUILD_NAME_i686).sys ::

$(BUILD_DIR_i686)/$(BUILD_NAME_i686).sys: $(BUILD_DIR_i686)/kernel.bin  $(BUILD_DIR_i686)/secondboot.bin
	cat $(BUILD_DIR_i686)/secondboot.bin $(BUILD_DIR_i686)/kernel.bin > $(BUILD_DIR_i686)/$(BUILD_NAME_i686).sys

$(BUILD_DIR_i686)/kernel.bin:target/$(TARGET_ARCH_i686)-rust/$(BUILD_MODE)/libructiss.a $(BUILD_DIR_i686)/osfunc.o ./kernel/arch/$(TARGET_ARCH_i686)/kernel.ld
	$(TARGET_ARCH_i686)-ld --gc-sections -t -nostdlib -Tdata=0x00310000 -T ./kernel/arch/$(TARGET_ARCH_i686)/kernel.ld -o $(BUILD_DIR_i686)/kernel.bin $(BUILD_DIR_i686)/osfunc.o --library-path=target/$(TARGET_ARCH_i686)-rust/$(BUILD_MODE) -lructiss -Map $(BUILD_DIR_i686)/kernel.map

# ipl (i386 only)
$(BUILD_DIR_i686)/ipl.bin: ./kernel/arch/$(TARGET_ARCH_i686)/asm/ipl.asm
	nasm -f bin -o $(BUILD_DIR_i686)/ipl.bin ./kernel/arch/$(TARGET_ARCH_i686)/asm/ipl.asm -l $(BUILD_DIR_i686)/ipl.lst

# secondboot (i386 only)
$(BUILD_DIR_i686)/secondboot.bin:./kernel/arch/$(TARGET_ARCH_i686)/asm/secondboot.asm
	nasm -f bin -o $(BUILD_DIR_i686)/secondboot.bin ./kernel/arch/$(TARGET_ARCH_i686)/asm/secondboot.asm -l $(BUILD_DIR_i686)/secondboot.lst

# kernel code
target/$(TARGET_ARCH_i686)-rust/$(BUILD_MODE)/libructiss.a: $(TARGET_ARCH_i686)-rust.json ./kernel/Cargo.toml ./kernel/src/*.rs
	RUST_TARGET_PATH=$(PWD) rustup run nightly `which xargo` build -v --target=$(TARGET_ARCH_i686)-rust --manifest-path kernel/Cargo.toml

$(BUILD_DIR_i686)/%.o:./kernel/arch/$(TARGET_ARCH_i686)/asm/%.asm
	nasm -f elf32 ./kernel/arch/$(TARGET_ARCH_i686)/asm/$*.asm -o $(BUILD_DIR_i686)/$*.o -l $(BUILD_DIR_i686)/$*.lst

#
# arm-none-eabi(GBA)
#

$(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).gba: $(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).elf
	$(TARGET_ARCH_GBA)-objcopy -O binary $(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).elf $(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).gba

$(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).elf: $(BUILD_DIR_GBA)/crt.o ./kernel/arch/$(TARGET_ARCH_GBA)/kernel.ld target/$(TARGET_ARCH_GBA)-rust/$(BUILD_MODE)/libructiss.a
	$(TARGET_ARCH_GBA)-ld -t -T ./kernel/arch/$(TARGET_ARCH_GBA)/kernel.ld -o $(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).elf  $(BUILD_DIR_GBA)/crt.o --library-path=target/$(TARGET_ARCH_GBA)-rust/$(BUILD_MODE) -lructiss -Map $(BUILD_DIR_GBA)/kernel.map

target/$(TARGET_ARCH_GBA)-rust/$(BUILD_MODE)/libructiss.a: $(TARGET_ARCH_GBA)-rust.json ./kernel/Cargo.toml ./kernel/src/*.rs
	RUST_TARGET_PATH=$(PWD) rustup run nightly `which xargo` build -v --target=$(TARGET_ARCH_GBA)-rust --manifest-path kernel/Cargo.toml

$(BUILD_DIR_GBA)/crt.o:./kernel/arch/$(TARGET_ARCH_GBA)/asm/crt.S
	$(TARGET_ARCH_GBA)-as ./kernel/arch/$(TARGET_ARCH_GBA)/asm/crt.S -o $(BUILD_DIR_GBA)/crt.o

# build target

i686:
	mkdir -p $(BUILD_DIR_i686)
	make $(BUILD_DIR_i686)/$(BUILD_NAME_i686).img

gba:
	mkdir -p $(BUILD_DIR_GBA)
	make $(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).gba

run:
	make i686
	make qemu

qemu:
	qemu-system-$(QEMU_ARCH_i686) -m 32 -localtime -vga std -fda $(BUILD_DIR_i686)/$(BUILD_NAME_i686).img -monitor stdio

tools:
	ifeq ($(UNAME),Darwin)
		brew install qemu
		brew install mtools	# mformat, mcopy
	endif

dump-i686:
	od $(BUILD_DIR_i686)/$(BUILD_NAME).img -t x1z -A x

dump-gba-file:
	arm-none-eabi-objdump -D -b binary -m arm --adjust-vma=0x08000000 $(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).gba > $(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).gba.lst
	arm-none-eabi-objdump -D build/ructiss-arm-none-eabi/ructiss-arm-none-eabi.elf > $(BUILD_DIR_GBA)/$(BUILD_NAME_GBA).elf.lst

clean:
	make clean-i686

clean-i686:
	rm -f $(BUILD_DIR_i686)/*.bin
	rm -f $(BUILD_DIR_i686)/*.o
	rm -f $(BUILD_DIR_i686)/*.a
	rm -f $(BUILD_DIR_i686)/*.d
	rm -f $(BUILD_DIR_i686)/*.rlib
	rm -f $(BUILD_DIR_i686)/*.lst
	rm -rf $(BUILD_DIR_i686)
	xargo clean --manifest-path ./kernel/Cargo.toml

clean-gba:
	rm -f $(BUILD_DIR_GBA)/*.gba
	rm -f $(BUILD_DIR_GBA)/*.o
	rm -f $(BUILD_DIR_GBA)/*.a
	rm -f $(BUILD_DIR_GBA)/*.d
	rm -f $(BUILD_DIR_GBA)/*.rlib
	rm -f $(BUILD_DIR_GBA)/*.lst
	rm -rf $(BUILD_DIR_GBA)
	xargo clean --manifest-path ./kernel/Cargo.toml
