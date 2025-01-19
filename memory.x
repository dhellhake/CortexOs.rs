EXTERN(DefaultHandler);

PROVIDE(NonMaskableInt = DefaultHandler);
PROVIDE(HardFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);
PROVIDE(DefaultHandler = DefaultHandler_);

MEMORY
{
  rom      (rx)  : ORIGIN = 0x00000000, LENGTH = 0x00040000
  ram      (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00008000
}

STACK_SIZE = DEFINED(STACK_SIZE) ? STACK_SIZE : DEFINED(__stack_size__) ? __stack_size__ : 0x1000;

SECTIONS
{
    PROVIDE(_ram_end = ORIGIN(ram) + LENGTH(ram));
    PROVIDE(_stack_start = _ram_end);
    
    .vectors ORIGIN(rom) :
    {
        LONG(_stack_start & 0xFFFFFFF8);

        /* Exception Table */
        KEEP(*(.vectors.exception_table));

        /* Interrupt Table */
        KEEP(*(.vectors.interrupt_table));
    } > rom

    
    PROVIDE(_stext = ADDR(.vectors) + SIZEOF(.vectors));

    .text _stext :
    {
        __stext = .;

        *(.text .text.*);
        *(.rodata .rodata*)

        . = ALIGN(4);
        __etext = .;
    } > rom

    .ARM.exidx :
    {
      *(.ARM.exidx*)
    } > rom

    . = ALIGN(4);
    _etext = .;

    .bss (NOLOAD) :
    {
        . = ALIGN(4);
        _sbss = . ;
        _szero = .;
        *(.bss .bss.*)

        . = ALIGN(4);
        _ebss = . ;
        _ezero = .;
    } > ram

    /* stack section */
    .stack (NOLOAD):
    {
        . = ALIGN(8);
        _sstack = .;
        . = . + STACK_SIZE;
        . = ALIGN(8);
        _estack = .;
    } > ram

    . = ALIGN(4);
    _end = . ;
}