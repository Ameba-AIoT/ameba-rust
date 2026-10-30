//! 4-byte aligned stack buffer for Flash I/O.
//! Uses `union + repr(align(4))` because ROM Flash functions require 4-byte aligned pointers.

use core::mem::MaybeUninit;

/// # Safety
///
/// Call `assume_init_bytes` / `assume_init_bytes_mut` only after the buffer
/// has been fully written (e.g. by a Flash read call).
#[repr(C, align(4))]
pub union FlashBuffer<const N: usize> {
    bytes: [MaybeUninit<u8>; N],
}

impl<const N: usize> FlashBuffer<N> {
    pub const fn uninit() -> Self {
        Self {
            bytes: [MaybeUninit::uninit(); N],
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [MaybeUninit<u8>; N] {
        // SAFETY: `bytes` is the only variant and always valid to access.
        unsafe { &mut self.bytes }
    }

    /// # Safety
    ///
    /// The caller must guarantee the buffer has been fully written.
    pub unsafe fn assume_init_bytes(&self) -> &[u8; N] {
        unsafe { &*(self.bytes.as_ptr() as *const [u8; N]) }
    }

    /// # Safety
    ///
    /// The caller must guarantee the buffer has been fully written.
    pub unsafe fn assume_init_bytes_mut(&mut self) -> &mut [u8; N] {
        unsafe { &mut *(self.bytes.as_mut_ptr() as *mut [u8; N]) }
    }
}

/// 4 KiB sector buffer.
pub type FlashSectorBuffer = FlashBuffer<4096>;
