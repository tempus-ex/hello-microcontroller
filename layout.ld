MEMORY
{
  FLASH_FPGA (r) : ORIGIN = 0x40000, LENGTH = 2M
  FLASH (rx) : ORIGIN = 0x2000, LENGTH = 0x00040000-0x2000 /* first 8KB used by bootloader */
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00008000
}

ENTRY(reset_handler)

SECTIONS
{
	.text :
	{
		. = ALIGN(0x2000);
		KEEP(*(.isr_vector))
		*(.text*)
	} > FLASH

    /DISCARD/ : {
        *(.ARM.exidx .ARM.exidx.*);
    }
}
