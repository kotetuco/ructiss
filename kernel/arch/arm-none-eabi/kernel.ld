OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

SECTIONS{
	/* 0x02000000: using boot cable */
	/* 0x08000000: using rom (GBA emulator) */
	.text 0x08000000 : {
		*(.text)
		/* *(.text.os_main) */
		/* *(.text*) */
	}
	.data : {
		*(.data)
	}
	.rodata : {
		*(.rodata*)
	}
	.bss : {
		*(.bss)
	}

	__exidx_start = .;
  .ARM.exidx : ALIGN(4)
  {
  	*(.ARM.exidx* .gnu.linkonce.armexidx.*)
  }
  __exidx_end = .;
}
