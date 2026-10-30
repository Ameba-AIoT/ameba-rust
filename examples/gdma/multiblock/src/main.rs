#![no_std]
#![no_main]

use core::ptr::{addr_of, addr_of_mut};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::cache::{dcache_clean, dcache_invalidate};
use ameba_hal::dma::{Async, Blocking, DmaChannel, GdmaIrqEvent, GdmaIrqHandler, ReloadConfig};
use ameba_hal::println;

// SDK reference: raw_gdma_multiblock — DATA_SIZE=128, BLOCK_SIZE=32 (4 blocks),
// GDMA_ReloadDst=1: DST resets to its fixed base address at the end of every
// block while SRC keeps advancing, so one reused 32-byte DST window streams
// the whole 128-byte SRC buffer through automatically, without CPU
// intervention between blocks. Each block fires its own interrupt; this
// example validates/consumes the data directly from inside that per-block
// ISR (`MultiblockHandler::on_irq`) — the streaming/ping-pong use case a
// callback that only fires once at final transfer completion cannot serve.

const DATA_SIZE: usize = 128;
const BLOCK_SIZE: usize = 32;
const MAX_BLOCKS: usize = DATA_SIZE / BLOCK_SIZE;

#[repr(align(32))]
struct SrcBuf([u8; DATA_SIZE]);
#[repr(align(32))]
struct DstBuf([u8; BLOCK_SIZE]);

static mut SRC: SrcBuf = SrcBuf([0u8; DATA_SIZE]);
static mut DST: DstBuf = DstBuf([0u8; BLOCK_SIZE]);

struct MultiblockHandler {
    block_count: AtomicUsize,
    done: AtomicBool,
    ok: AtomicBool,
}

impl GdmaIrqHandler for MultiblockHandler {
    fn on_irq(&mut self, event: GdmaIrqEvent) {
        if event.error {
            println!("[GDMA-IRQ] error isr=0x{:08X} (from ISR)", event.isr);
            self.ok.store(false, Ordering::Relaxed);
            self.done.store(true, Ordering::Release);
            return;
        }
        if !event.block_done && !event.transfer_done {
            return;
        }

        let idx = self.block_count.load(Ordering::Relaxed);

        // Mirrors SDK raw_gdma_multiblock's `MaxMuliBlock == MuliBlockCunt + 2`
        // check: disable auto-reload while block `idx` is the second-to-last
        // one, so the write lands before the real final block completes —
        // that block's completion then fires transfer_done instead of
        // reload-and-continue.
        if idx + 2 == MAX_BLOCKS {
            let mut ch = unsafe { DmaChannel::<0, Async>::steal() };
            ch.clean_auto_reload(ReloadConfig {
                reload_src: false,
                reload_dst: true,
            });
        }

        unsafe {
            dcache_invalidate(addr_of!(DST.0) as *const u8, BLOCK_SIZE);
            let src = core::slice::from_raw_parts(
                (addr_of!(SRC.0) as *const u8).add(idx * BLOCK_SIZE),
                BLOCK_SIZE,
            );
            let dst = core::slice::from_raw_parts(addr_of!(DST.0) as *const u8, BLOCK_SIZE);
            if src != dst {
                self.ok.store(false, Ordering::Relaxed);
                println!("[GDMA-IRQ] block {} mismatch (from ISR)", idx);
            }
            // DST is reused every block — clear it so a stale match can't
            // hide a skipped block on the next round, then flush the clear
            // out of D-Cache before the DMA controller reuses the buffer.
            addr_of_mut!(DST.0).write_bytes(0, 1);
            dcache_clean(addr_of!(DST.0) as *const u8, BLOCK_SIZE);
        }

        println!(
            "[GDMA-IRQ] block {} done isr=0x{:02X} (from ISR)",
            idx, event.isr
        );
        self.block_count.store(idx + 1, Ordering::Relaxed);

        if event.transfer_done {
            self.done.store(true, Ordering::Release);
        }
    }
}

static mut HANDLER: MultiblockHandler = MultiblockHandler {
    block_count: AtomicUsize::new(0),
    done: AtomicBool::new(false),
    ok: AtomicBool::new(true),
};

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[GDMA multiblock] start");
    ameba_hal::init(ameba_hal::Config::default());

    unsafe {
        let src_ptr = addr_of_mut!(SRC.0) as *mut u8;
        for i in 0..DATA_SIZE {
            src_ptr
                .add(i)
                .write((i as u8).wrapping_mul(5).wrapping_add(0x11));
        }
        dcache_clean(addr_of!(SRC.0) as *const u8, DATA_SIZE);
        dcache_invalidate(addr_of!(DST.0) as *const u8, BLOCK_SIZE);
    }

    let mut ch: DmaChannel<0, Async> = unsafe { DmaChannel::<0, Blocking>::steal().into_async() };

    let handler = unsafe { &mut *addr_of_mut!(HANDLER) };
    ch.register_irq(handler);

    unsafe {
        ch.mem_to_mem_reload_start(
            addr_of!(SRC.0) as *const u8,
            addr_of_mut!(DST.0) as *mut u8,
            BLOCK_SIZE,
            ReloadConfig {
                reload_src: false,
                reload_dst: true,
            },
        );
    }

    while !unsafe { (*addr_of!(HANDLER)).done.load(Ordering::Acquire) } {
        cortex_m::asm::wfi();
    }

    ch.unregister_irq();

    let blocks = unsafe { (*addr_of!(HANDLER)).block_count.load(Ordering::Relaxed) };
    let ok = unsafe { (*addr_of!(HANDLER)).ok.load(Ordering::Relaxed) };

    if ok && blocks == MAX_BLOCKS {
        println!(
            "[PASS] GDMA multiblock: {} blocks of {} bytes streamed via per-block ISR OK",
            blocks, BLOCK_SIZE
        );
    } else {
        println!(
            "[FAIL] GDMA multiblock: ok={} blocks={} (expected {})",
            ok, blocks, MAX_BLOCKS
        );
    }

    loop {
        cortex_m::asm::wfi();
    }
}
