/* AuroraOS Kernel Linker Script */

ENTRY(_start)

SECTIONS
{
    . = 1M;
    
    /* Bootloader puts physical memory map at this address */
    _boot_info = .;
    
    /* Code section */
    .text : ALIGN(4K)
    {
        *(.text._start)
        *(.text*)
    }
    
    /* Read-only data section */
    .rodata : ALIGN(4K)
    {
        *(.rodata*)
    }
    
    /* Data section */
    .data : ALIGN(4K)
    {
        *(.data*)
    }
    
    /* BSS section */
    .bss : ALIGN(4K)
    {
        _bss_start = .;
        *(COMMON)
        *(.bss*)
        _bss_end = .;
    }
    
    /* Kernel stack */
    .stack : ALIGN(16)
    {
        _stack_bottom = .;
        . += 0x100000;  /* 1MB stack */
        _stack_top = .;
    }
    
    /* Heap section - starts after kernel, grows upward */
    .heap : ALIGN(4K)
    {
        _heap_start = .;
        . += 0x10000000;  /* 256MB initial heap */
        _heap_end = .;
    }
    
    /* Discard unwanted sections */
    /DISCARD/ :
    {
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.gnu.hash)
        *(.note.*)
    }
}
