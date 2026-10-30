#![no_std]
#![no_main]

use ameba_hal::ipc::{self, n2a};
use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use ameba_hal::println;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ────────────────────────────────────────────────────────────────────────────
// UART_LOG_BUF (from ameba_shell.h)
// ────────────────────────────────────────────────────────────────────────────

#[repr(C)]
struct UartLogBuf {
    buf_count: u16,
    buf_len: u16,
    buf_ptr: u32, // raw pointer (32-bit)
}

// ────────────────────────────────────────────────────────────────────────────
// Command dispatch table
// ────────────────────────────────────────────────────────────────────────────

struct Cmd {
    name: &'static str,
    func: fn(args: &[&str]),
}

/// Add new commands here.
static COMMANDS: &[Cmd] = &[
    Cmd {
        name: "dw",
        func: cmd_dw,
    },
    Cmd {
        name: "ew",
        func: cmd_ew,
    },
];

// dw <hex_addr>  — read a 32-bit word and print it
fn cmd_dw(args: &[&str]) {
    let Some(&addr_s) = args.first() else {
        println!("usage: dw <hex_addr>");
        return;
    };
    let Some(addr) = parse_hex(addr_s) else {
        println!("dw: bad address");
        return;
    };
    let val = unsafe { (addr as *const u32).read_volatile() };
    println!("[0x{:08x}] = 0x{:08x}", addr, val);
}

// ew <hex_addr> <hex_val>  — write a 32-bit word
fn cmd_ew(args: &[&str]) {
    if args.len() < 2 {
        println!("usage: ew <hex_addr> <hex_val>");
        return;
    }
    let (Some(addr), Some(val)) = (parse_hex(args[0]), parse_hex(args[1])) else {
        println!("ew: bad address or value");
        return;
    };
    unsafe {
        (addr as *mut u32).write_volatile(val);
    }
    println!("[0x{:08x}] <- 0x{:08x}", addr, val);
}

// ────────────────────────────────────────────────────────────────────────────
// Helpers: hex parser + tokenizer
// ────────────────────────────────────────────────────────────────────────────

fn parse_hex(s: &str) -> Option<u32> {
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    if s.is_empty() {
        return None;
    }
    let mut v: u32 = 0;
    for b in s.bytes() {
        let d = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        v = v.checked_mul(16)?.checked_add(d as u32)?;
    }
    Some(v)
}

/// Split `s` on ASCII whitespace; fills `out` and returns token count.
fn tokenize<'a>(s: &'a str, out: &mut [&'a str; 8]) -> usize {
    let mut n = 0usize;
    let mut start: Option<usize> = None;
    for (i, b) in s.bytes().enumerate() {
        let sep = matches!(b, b' ' | b'\t' | b'\r' | b'\n');
        if sep {
            if let Some(s0) = start.take()
                && n < 8
            {
                out[n] = &s[s0..i];
                n += 1;
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }
    if let Some(s0) = start
        && n < 8
    {
        out[n] = &s[s0..];
        n += 1;
    }
    n
}

fn dispatch(buf: &[u8]) {
    // Find null terminator or end of slice.
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    let Ok(s) = core::str::from_utf8(&buf[..end]) else {
        return;
    };

    let mut tokens = [""; 8];
    let n = tokenize(s, &mut tokens);
    if n == 0 {
        return;
    }

    let name = tokens[0];
    let args = &tokens[1..n];

    for cmd in COMMANDS {
        if cmd.name == name {
            (cmd.func)(args);
            return;
        }
    }

    // Unknown — print with a null-terminated copy of the original buffer.
    let s = core::str::from_utf8(&buf[..buf.iter().position(|&b| b == 0).unwrap_or(buf.len())])
        .unwrap_or("?");
    println!("shell: unknown cmd '{}'", s);
}

// ────────────────────────────────────────────────────────────────────────────
// IPC receive: copy to static buffer, signal main loop
// ────────────────────────────────────────────────────────────────────────────

const CMD_BUF_CAP: usize = 256;

static CMD_PENDING: AtomicBool = AtomicBool::new(false);
static CMD_LEN: AtomicU32 = AtomicU32::new(0);
// Written only from IPC IRQ (single-core), read only when CMD_PENDING is set.
static mut CMD_BUF: [u8; CMD_BUF_CAP] = [0u8; CMD_BUF_CAP];

unsafe fn shell_ipc_handler(_data: *mut (), _ipc_status: u32, channel: u8) {
    unsafe {
        let msg = ipc::get_message(channel);
        let log_buf_ptr = msg.msg as *const UartLogBuf;
        if log_buf_ptr.is_null() {
            return;
        }

        let mut cp = cortex_m::peripheral::Peripherals::steal();
        cp.SCB
            .invalidate_dcache_by_address(log_buf_ptr as usize, core::mem::size_of::<UartLogBuf>());

        let buf_count = core::ptr::read_volatile(&(*log_buf_ptr).buf_count) as usize;
        let buf_ptr = core::ptr::read_volatile(&(*log_buf_ptr).buf_ptr) as *const u8;
        if buf_ptr.is_null() || buf_count == 0 {
            return;
        }

        cp.SCB
            .invalidate_dcache_by_address(buf_ptr as usize, buf_count);

        // Copy into local buffer and null-terminate.
        let copy_len = buf_count.min(CMD_BUF_CAP - 1);
        #[allow(static_mut_refs)]
        core::ptr::copy_nonoverlapping(buf_ptr, CMD_BUF.as_mut_ptr(), copy_len);
        CMD_BUF[copy_len] = 0;
        CMD_LEN.store(copy_len as u32, Ordering::Release);
        CMD_PENDING.store(true, Ordering::Release);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Main
// ────────────────────────────────────────────────────────────────────────────

static TOTAL_RX: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("shell: start");
    ameba_hal::init(ameba_hal::Config::default());

    let ipc = unsafe { ameba_hal::ipc::Ipc::ap() };
    unsafe {
        ipc.register_handler(n2a::LOGUART_RX, shell_ipc_handler, core::ptr::null_mut());
    }
    println!("shell: ready on N2A ch{}", n2a::LOGUART_RX as u32);

    loop {
        Timer::after(Duration::from_millis(10)).await;

        if CMD_PENDING.load(Ordering::Acquire) {
            CMD_PENDING.store(false, Ordering::Relaxed);
            let len = CMD_LEN.load(Ordering::Relaxed) as usize;
            TOTAL_RX.fetch_add(1, Ordering::Relaxed);
            // SAFETY: IRQ already returned; CMD_BUF is stable until next CMD_PENDING.
            #[allow(static_mut_refs)]
            unsafe {
                dispatch(&CMD_BUF[..len + 1]);
            } // +1 to include null terminator
        }
    }
}
