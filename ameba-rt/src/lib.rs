#![no_std]
#![allow(deprecated)]

use cortex_m_rt::pre_init;

// Bootloader entry table at ORIGIN(RAM) = 0x20006020.
// The linker script places this section at ORIGIN(RAM) via KEEP(*(.image2.entry.data*)).
// RamStartFun → ameba_entry_trampoline: switch stack to our SRAM, then jump to Reset.
//
// ── Why the trampoline is needed ────────────────────────────────────────────
//
// bootloader_km4tz.c :: BOOT_ClearMSP_NsStart():
//   __set_MSP(MSP_RAM_HP);   // 0x30001000 (Secure alias of 0x20001000, bit28=1)
//   pFunc();                 // direct BLX — MSP is NOT reloaded from vector table
//
// At the moment our entry is called:
//   MSP    = 0x30001000  (= 0x20001000 | BIT28 — Secure alias, same physical SRAM)
//   MSPLIM = 0x30000600  (ROM constant, hal_platform.h: MSPLIM_RAM_HP, cannot change)
//   usable = 0x30001000 - 0x30000600 = 2560 bytes = 2.5 KB
//
// Secure alias: 0x3xxxxxxx = 0x2xxxxxxx | BIT(28).  TrustZone is off (CONFIG_TRUSTZONE
// not defined), so the entire CPU address space is Secure — 0x30001000 and 0x20001000
// are the same physical word, accessed with Secure attribute.
//
// The problem: cortex-m-rt Reset, BSS clear, and our first function that allocates
// FlashSectorBuffer (4096 B) on the stack all happen within this 2.5 KB window.
// The trampoline must switch MSP to our application stack BEFORE Reset runs.
core::arch::global_asm!(
    // Entry table: first word is RamStartFun pointer
    ".section .image2.entry.data, \"a\", %progbits",
    ".balign 4",
    ".global Img2EntryFun0",
    "Img2EntryFun0:",
    ".word ameba_entry_trampoline", // RamStartFun → trampoline (.thumb_func sets Thumb bit)
    ".word 0",                      // RamWakeupFun → NULL
    ".word 0",                      // VectorNS     → 0
    // Trampoline: switch MSP to our SRAM stack, then tail-call cortex-m-rt Reset.
    ".section .sram.only.text, \"ax\", %progbits",
    ".balign 4",
    ".global ameba_entry_trampoline",
    ".thumb_func",
    "ameba_entry_trampoline:",
    // Clear MSPLIM first — must happen before MSP moves below current MSPLIM value,
    // otherwise the MSR MSP instruction itself would fault.
    // (MSPLIM_RAM_HP = 0x30000600, ROM-hardcoded, cannot be changed from bootloader.)
    "movs r0, #0",
    "msr msplim, r0",
    // Switch MSP to _stack_start = 0x20068000 (top of KM4TZ IMG2 RAM, TZ off).
    // This is the boundary between KM4TZ app RAM and KM4NS RAM in the 512 KB SRAM.
    "ldr r0, =_stack_start",
    "msr msp, r0",
    // Tail-call cortex-m-rt Reset — Reset will re-read VTOR via pre_init, clear BSS,
    // copy .data, and call main.  The stack is now correct for all of that.
    "b Reset",
);

// Called by cortex-m-rt Reset before BSS clear and data init.
// VTOR must point to our vector table (0x20006200) before any exception can fire.
#[allow(deprecated)]
#[pre_init]
unsafe fn ameba_pre_init() {
    unsafe {
        const SCB_VTOR: *mut u32 = 0xE000_ED08 as *mut u32;
        core::ptr::write_volatile(SCB_VTOR, 0x2000_6200);
    }
}

unsafe extern "C" {
    fn loguart_panic_puts(s: *const u8, len: u32);
}

/// `core::fmt::Write` sink that forwards each formatted chunk to the ROM/PAC
/// LogUART via `loguart_panic_puts`, so the panic handler can print the full
/// `PanicInfo` (message + location) without pulling in `ameba-hal`.
struct PanicWriter;

impl core::fmt::Write for PanicWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { loguart_panic_puts(s.as_ptr(), s.len() as u32) };
        Ok(())
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    // `PanicInfo: Display` prints `panicked at <file>:<line>:<col>:\n<message>`.
    // Delegating to it gives us the panic message and a correctly formatted line
    // number for any digit count (the previous hand-rolled printer both dropped
    // the message and mis-printed lines >= 100000).
    let _ = writeln!(PanicWriter, "\nPANIC: {}", info);
    loop {
        core::hint::spin_loop();
    }
}

/// A CPU fault (bad memory access, unaligned access, divide-by-zero, ...) does
/// not go through `#[panic_handler]` — cortex-m-rt's default `HardFault` is a
/// silent `loop {}` that runs at the highest exception priority, permanently
/// blocking every normal-priority interrupt (including `IPC_CPU0_IRQ`) with no
/// output at all. That silent-lockup signature is otherwise indistinguishable
/// from an AP-side stall, so print the fault status registers before spinning.
#[cortex_m_rt::exception]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    unsafe {
        use core::fmt::Write;
        const CFSR: *const u32 = 0xE000_ED28 as *const u32;
        const HFSR: *const u32 = 0xE000_ED2C as *const u32;
        const MMFAR: *const u32 = 0xE000_ED34 as *const u32;
        const BFAR: *const u32 = 0xE000_ED38 as *const u32;
        let cfsr = core::ptr::read_volatile(CFSR);
        let hfsr = core::ptr::read_volatile(HFSR);
        let mmfar = core::ptr::read_volatile(MMFAR);
        let bfar = core::ptr::read_volatile(BFAR);
        let _ = writeln!(
            PanicWriter,
            "\nHARDFAULT: pc={:#010x} lr={:#010x} cfsr={:#010x} hfsr={:#010x} mmfar={:#010x} bfar={:#010x}",
            frame.pc(),
            frame.lr(),
            cfsr,
            hfsr,
            mmfar,
            bfar,
        );
        loop {
            core::hint::spin_loop();
        }
    }
}
