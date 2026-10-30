#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};

use ameba_hal::cache::{dcache_clean, dcache_invalidate};
use ameba_hal::dma::{Async, Blocking, DmaChannel};
use ameba_hal::println;

#[repr(align(32))]
struct AlignedBuf([u8; 64]);

static mut SRC: AlignedBuf = AlignedBuf([0u8; 64]);
static mut DST: AlignedBuf = AlignedBuf([0u8; 64]);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());

    println!("[GDMA] start");

    // Fill source buffer with known pattern
    unsafe {
        let src_ptr = core::ptr::addr_of_mut!(SRC.0) as *mut u8;
        for i in 0..64usize {
            src_ptr
                .add(i)
                .write((i as u8).wrapping_mul(3).wrapping_add(0xA5));
        }
        // Flush SRC from D-Cache to SRAM so DMA can read it
        dcache_clean(core::ptr::addr_of!(SRC.0) as *const u8, 64);
        // Invalidate DST in D-Cache before DMA writes
        dcache_invalidate(core::ptr::addr_of!(DST.0) as *const u8, 64);
    }

    // ── Async path ────────────────────────────────────────────────────────────
    let mut ch: DmaChannel<0, Async> = unsafe { DmaChannel::<0, Blocking>::steal().into_async() };

    let t0 = Instant::now();
    unsafe {
        ch.mem_to_mem(
            core::ptr::addr_of!(SRC.0) as *const u8,
            core::ptr::addr_of_mut!(DST.0) as *mut u8,
            64,
        )
        .await;
    }
    let elapsed = t0.elapsed().as_millis();

    // Invalidate DST D-Cache so CPU reads fresh DMA-written SRAM
    unsafe {
        dcache_invalidate(core::ptr::addr_of!(DST.0) as *const u8, 64);
    }

    println!("[GDMA] async done in {}ms", elapsed);

    // ── Verify ────────────────────────────────────────────────────────────────
    let ok = unsafe {
        let src = core::slice::from_raw_parts(core::ptr::addr_of!(SRC.0) as *const u8, 64);
        let dst = core::slice::from_raw_parts(core::ptr::addr_of!(DST.0) as *const u8, 64);
        src == dst
    };
    if !ok {
        unsafe {
            let src = core::slice::from_raw_parts(core::ptr::addr_of!(SRC.0) as *const u8, 64);
            let dst = core::slice::from_raw_parts(core::ptr::addr_of!(DST.0) as *const u8, 64);
            for i in 0..64usize {
                if src[i] != dst[i] {
                    println!(
                        "[FAIL] GDMA: mismatch at [{}] expected=0x{:02x} got=0x{:02x}",
                        i, src[i], dst[i]
                    );
                    break;
                }
            }
        }
    } else {
        println!("[PASS] GDMA: mem-to-mem 64 bytes OK");
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
