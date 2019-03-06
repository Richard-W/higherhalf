ENTRY (kmain)

KERNEL_BASE = 0xffff800000000000;

SECTIONS
{
	. = KERNEL_BASE;

	.text ALIGN(4K) : {
		*(.text*)
	}

	.rodata ALIGN(4K) : {
		*(.rodata*)
	}

	.bss ALIGN(4K) : { 
		*(.bss*)
	}

	.data ALIGN(4K) : {
		*(.data*)
	}
}

