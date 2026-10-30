MEMORY
{
    RAM   (rwx) : ORIGIN = 0x20006020, LENGTH = 0x61FE0
    XIP   (rx)  : ORIGIN = 0x04000020, LENGTH = 0x1FFFE0
    PSRAM (rwx) : ORIGIN = 0x60000020, LENGTH = 0x324FE0
}

ENTRY(Reset)

/*
 * cortex-m-rt 0.7 uses __EXCEPTIONS / __RESET_VECTOR statics with extern "C"
 * references that are resolved by these PROVIDE chains (mirrored from link.x.in).
 * We cannot include link.x.in directly because it conflicts with our custom layout.
 */
PROVIDE(DefaultHandler   = DefaultHandler_);
PROVIDE(NonMaskableInt   = DefaultHandler);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault         = DefaultHandler);
PROVIDE(UsageFault       = DefaultHandler);
PROVIDE(SecureFault      = DefaultHandler);
PROVIDE(SVCall           = DefaultHandler);
PROVIDE(DebugMon         = DefaultHandler);
PROVIDE(DebugMonitor     = DefaultHandler);
PROVIDE(PendSV           = DefaultHandler);
PROVIDE(SysTick          = DefaultHandler);

SECTIONS
{
    /* XIP flash code — empty while all code runs from SRAM */
    .xip_image2.text ORIGIN(XIP) :
    {
        __flash_text_start__ = .;
        *(.xip_image2.text .xip_image2.text.*)
        __flash_text_end__ = .;
    } > XIP

    /* SRAM image2 load base */
    __sram_image2_start__ = ORIGIN(RAM);

    /* Bootloader entry struct: 12 bytes at ORIGIN = 0x20006020
     * RamStartFun → cortex-m-rt Reset, so we get BSS/data init + pre_init. */
    .image2_entry ORIGIN(RAM) :
    {
        __image2_entry_func__ = .;
        KEEP(*(.image2.entry.data*))
    } > RAM

    /* Padding section to reach 0x20006200 (512-byte aligned, required for VTOR).
     * ORIGIN(RAM)=0x20006020, entry struct=12 bytes → pad 0x1E0 bytes to reach 0x20006200. */
    .image2_pad (NOLOAD) :
    {
        . = ORIGIN(RAM) + 0x1E0;
    } > RAM

    /* ARM Cortex-M33 vector table at 0x20006200 — address written to SCB.VTOR */
    .vector_table :
    {
        __vector_table = .;
        LONG(_stack_start);                    /* Entry 0: initial MSP        */
        KEEP(*(.vector_table.reset_vector));   /* Entry 1: Reset              */
        KEEP(*(.vector_table.exceptions));     /* Entries 2-15: NMI…SysTick  */
        /* Peripheral IRQs 0-78: emitted by the PAC's `__INTERRUPTS` array
         * (rt feature), so the table is generated from the single source of
         * truth instead of being hand-maintained here. KEEP is required —
         * --gc-sections would otherwise drop the whole array. */
        KEEP(*(.vector_table.interrupts));
    } > RAM

    /* Code and read-only data */
    .text :
    {
        *(.Reset)
        *(.text .text.*)
        *(.sram.only.text)
        *(.rodata .rodata.*)
        . = ALIGN(4);
    } > RAM

    /* Initialized data — LMA == VMA (bootloader loaded us directly to RAM).
     * cortex-m-rt's init_data copy is a no-op since __sidata == __sdata. */
    .data :
    {
        . = ALIGN(4);
        __sdata = .;
        *(.data .data.*)
        . = ALIGN(4);
        __edata = .;
    } > RAM
    __sidata = ADDR(.data);
    __sram_image2_end__ = .;

    /* Zero-initialized data */
    .bss (NOLOAD) :
    {
        . = ALIGN(4);
        __sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        __ebss = .;
    } > RAM

    /* Non-cacheable RAM region for MPU nocache mapping (ameba-hal/src/mpu.rs).
     * Empty for now — no driver currently places a static buffer here since
     * UART/SPI DMA already do manual cache maintenance per transfer. Mirrors
     * the SDK's own `region_size >= 32` guard in app_mpu_nocache_init().
     *
     * The ALIGN(32) pair matches the SDK's ameba_img2_all.ld and is required
     * once this region is non-empty: PMSAv8 RBAR/RLAR encode the base/limit
     * with a 32-byte granule, so an unaligned region would round outward and
     * mark neighbouring cacheable data non-cacheable. */
    .ram_nocache (NOLOAD) :
    {
        . = ALIGN(32);
        __ram_nocache_start__ = .;
        *(.ram_nocache .ram_nocache.*)
        . = ALIGN(32);
        __ram_nocache_end__ = .;
    } > RAM

    /* Stack descends from top of RAM */
    _stack_start = ORIGIN(RAM) + LENGTH(RAM);

    /* PSRAM code+data — empty while all code runs from SRAM */
    .psram_image2.text.data ORIGIN(PSRAM) :
    {
        __psram_image2_start__ = .;
        *(.psram_image2.text.data .psram_image2.text.data.*)
        __psram_image2_end__ = .;
    } > PSRAM

    /DISCARD/ :
    {
        *(.ARM.exidx*)
        *(.ARM.extab*)
        *(.eh_frame*)
    }
}

/* cortex-m-rt 0.7 on thumbv8m.main defines HardFault_ (no trampoline) */
PROVIDE(HardFault = HardFault_);

ASSERT(__image2_entry_func__ == ORIGIN(RAM),
       "__image2_entry_func__ is not at ORIGIN(RAM) = 0x20006020")
ASSERT(__vector_table == 0x20006200,
       "__vector_table not at 0x20006200 — VTOR alignment broken")

/* ROM function addresses — defined in ameba-rom-sys/ld/ameba_rom_symbol.x */
INCLUDE ameba_rom_symbol.x

/* Peripheral-interrupt default handlers: `device.x` (shipped by the PAC's rt
 * feature) supplies the 79 weak `PROVIDE(..._IRQ = DefaultHandler)` aliases.
 * Handlers defined by `bind_interrupts!` / `#[interrupt]` are strong symbols
 * and take precedence over these weak aliases.
 *
 * Kept in the same relative position as cortex-m-rt's own link.x.in (which
 * appends `INCLUDE device.x` after the SECTIONS block, following
 * `EXTERN(__INTERRUPTS)`); its build.rs warns that the weak aliases must come
 * after `EXTERN(__INTERRUPTS)` or the linker may satisfy the vector table from
 * the aliases and ignore user handlers. */
EXTERN(__INTERRUPTS);
INCLUDE device.x
