//! RTL8721F USB Video Class (UVC) driver.

use bitflags::bitflags;
use rtl8721f_pac::{UvcReg, uvc_reg::RegisterBlock};

#[inline(always)]
fn regs() -> &'static RegisterBlock {
    unsafe { &*UvcReg::PTR }
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const TOTAL_CHANNEL_NUM: u32 = 2;
const MAX_CMD_NUM: usize = 6;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Frame-done trigger mode.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FrameDoneMode {
    /// Frame done when frame-id changes (FID mode).
    FrameIdChange = 0,
    /// Frame done when xfercomp at EOF flag.
    Eof = 1,
}

/// Buffer index for a UVC channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BufIndex {
    Buf0 = 0,
    Buf1 = 1,
}

bitflags! {
    /// Interrupt mask bits written to REG_UVC_INTR[31:16].
    ///
    /// Each bit enables the corresponding status interrupt source.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct IntrMask: u32 {
        const CH0_BUF0_FRM_DONE      = 1 << 16;
        const CH0_BUF1_FRM_DONE      = 1 << 17;
        const CH0_BUF0_FRM_OVERSIZE  = 1 << 18;
        const CH0_BUF1_FRM_OVERSIZE  = 1 << 19;
        const CH0_BUF0_HEADER_ERR    = 1 << 20;
        const CH0_BUF1_HEADER_ERR    = 1 << 21;
        const CH1_BUF0_FRM_DONE      = 1 << 24;
        const CH1_BUF1_FRM_DONE      = 1 << 25;
        const CH1_BUF0_FRM_OVERSIZE  = 1 << 26;
        const CH1_BUF1_FRM_OVERSIZE  = 1 << 27;
        const CH1_BUF0_HEADER_ERR    = 1 << 28;
        const CH1_BUF1_HEADER_ERR    = 1 << 29;
    }
}

bitflags! {
    /// RW1C status bits in REG_UVC_INTR[13:0].
    ///
    /// Pass to [`Uvc::int_clear`] to clear specific status flags.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct IntrStatus: u32 {
        const CH0_BUF0_FRM_DONE      = 1 << 0;
        const CH0_BUF1_FRM_DONE      = 1 << 1;
        const CH0_BUF0_FRM_OVERSIZE  = 1 << 2;
        const CH0_BUF1_FRM_OVERSIZE  = 1 << 3;
        const CH0_BUF0_HEADER_ERR    = 1 << 4;
        const CH0_BUF1_HEADER_ERR    = 1 << 5;
        const CH1_BUF0_FRM_DONE      = 1 << 8;
        const CH1_BUF1_FRM_DONE      = 1 << 9;
        const CH1_BUF0_FRM_OVERSIZE  = 1 << 10;
        const CH1_BUF1_FRM_OVERSIZE  = 1 << 11;
        const CH1_BUF0_HEADER_ERR    = 1 << 12;
        const CH1_BUF1_HEADER_ERR    = 1 << 13;
    }
}

/// UVC command register configuration.
#[derive(Clone, Copy, Default)]
pub struct CmdReg {
    /// Byte offset within the UVC header (12-bit field).
    pub offset: u16,
    /// Command type (3-bit field).
    pub cmd_type: u8,
    /// Enable this command slot.
    pub enable: bool,
    /// 32-bit data written to the CMDx_DATA register.
    pub data: u32,
}

/// Configuration for a single UVC channel, used with [`Uvc::channel_init`].
pub struct ChannelConfig {
    /// USB endpoint number for this channel.
    pub ep_num: u8,
    /// DMA address (HCDMAi value from OTG core).
    pub hcdmai: u32,
    /// Buffer 0 start address.
    pub buf0_start: u32,
    /// Buffer 0 end address (exclusive).
    pub buf0_end: u32,
    /// Buffer 1 start address.
    pub buf1_start: u32,
    /// Buffer 1 end address (exclusive).
    pub buf1_end: u32,
    /// Frame-done trigger mode.
    pub eof: FrameDoneMode,
    /// USB bInterval value (must be > 0).
    pub binterval: u8,
    /// Enable EP auto-enable function.
    pub ep_auto_en: bool,
    /// Command registers (up to 6 slots).
    pub cmds: [CmdReg; MAX_CMD_NUM],
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            ep_num: 0,
            hcdmai: 0,
            buf0_start: 0,
            buf0_end: 0,
            buf1_start: 0,
            buf1_end: 0,
            eof: FrameDoneMode::FrameIdChange,
            binterval: 1,
            ep_auto_en: true,
            cmds: [CmdReg::default(); MAX_CMD_NUM],
        }
    }
}

// ---------------------------------------------------------------------------
// Type-state tokens for channel allocation
// ---------------------------------------------------------------------------

/// Type-state: channel slot is free (returned by [`Uvc::alloc_channel`]).
pub struct Free;
/// Type-state: channel slot is allocated and ready to program.
pub struct Allocated;

/// A handle to a UVC hardware channel.
///
/// Dropping this returns the channel slot to the allocator.
pub struct Channel {
    index: u32,
}

impl Channel {
    /// Return the channel index (0 or 1).
    pub fn index(&self) -> u32 {
        self.index
    }
}

// ---------------------------------------------------------------------------
// Software channel allocator
// ---------------------------------------------------------------------------

/// Per-instance channel allocation bitmap.
/// Uses a `u32` with bits 0..TOTAL_CHANNEL_NUM as allocation flags.
struct ChannelAllocator(u32);

impl ChannelAllocator {
    const fn new() -> Self {
        Self(0)
    }

    fn alloc(&mut self) -> Option<Channel> {
        for i in 0..TOTAL_CHANNEL_NUM {
            if self.0 & (1 << i) == 0 {
                self.0 |= 1 << i;
                return Some(Channel { index: i });
            }
        }
        None
    }

    fn free(&mut self, ch: Channel) {
        self.0 &= !(1 << ch.index);
    }
}

// ---------------------------------------------------------------------------
// Driver struct
// ---------------------------------------------------------------------------

/// RTL8721F UVC peripheral driver.
///
/// Obtain via [`Uvc::steal`].
pub struct Uvc {
    allocator: ChannelAllocator,
}

impl Uvc {
    /// Construct the driver by stealing the UVC peripheral.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other code accesses the UVC peripheral
    /// concurrently, and that the peripheral clock has been enabled before
    /// any register-accessing methods are called.
    pub unsafe fn steal() -> Self {
        Self {
            allocator: ChannelAllocator::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Channel allocation
    // -----------------------------------------------------------------------

    /// Allocate a free UVC channel.
    ///
    /// Returns `None` if all channels are in use.
    pub fn alloc_channel(&mut self) -> Option<Channel> {
        self.allocator.alloc()
    }

    /// Release a previously allocated channel.
    pub fn free_channel(&mut self, ch: Channel) {
        self.allocator.free(ch);
    }

    // -----------------------------------------------------------------------
    // Interrupt mask configuration
    // -----------------------------------------------------------------------

    /// Enable or disable one or more interrupt sources.
    ///
    /// `mask` selects which interrupt mask bits (`REG_UVC_INTR[31:16]`) to set
    /// or clear. The RW1C status bits `[13:0]` are kept zero in the written value
    /// so they are not accidentally cleared.
    pub fn int_config(&self, mask: IntrMask, enable: bool) {
        let r = regs();
        // Read the current register value and strip the RW1C status bits before
        // writing back — writing a 1 to a status bit clears it, so we must not
        // carry those bits through a modify().
        let current_mask_half = r.reg_uvc_intr().read().bits() & 0xFFFF_0000;
        let m = mask.bits();
        let new_mask_half = if enable {
            current_mask_half | m
        } else {
            current_mask_half & !m
        };
        // Write only the mask half; status half is zero (no unintended RW1C clear).
        unsafe {
            r.reg_uvc_intr().as_ptr().write_volatile(new_mask_half);
        }
    }

    /// Clear one or more RW1C interrupt status bits.
    ///
    /// The status bits `[13:0]` of REG_UVC_INTR are RW1C; the PAC marks
    /// them read-only, so this uses a raw volatile write of only the status
    /// portion, leaving the mask bits `[31:16]` unaffected.
    pub fn int_clear(&self, status: IntrStatus) {
        let r = regs();
        // Read the current mask half so we don't disturb it.
        let current = r.reg_uvc_intr().read().bits();
        let mask_half = current & 0xFFFF_0000;
        // Write: preserve mask bits, set only the requested RW1C status bits.
        unsafe {
            r.reg_uvc_intr()
                .as_ptr()
                .write_volatile(mask_half | status.bits());
        }
    }

    /// Read the raw interrupt status word (bits `[13:0]`).
    pub fn int_status(&self) -> IntrStatus {
        let raw = regs().reg_uvc_intr().read().bits() & 0x0000_3FFF;
        IntrStatus::from_bits_truncate(raw)
    }

    /// Read the current interrupt mask configuration (bits `[29:16]` of REG_UVC_INTR).
    pub fn int_mask_status(&self) -> IntrMask {
        let raw = regs().reg_uvc_intr().read().bits() & 0x3FFF_0000;
        IntrMask::from_bits_truncate(raw)
    }

    // -----------------------------------------------------------------------
    // Per-channel register setters
    // -----------------------------------------------------------------------

    /// Set the USB endpoint number for a channel.
    pub fn set_ep_num(&self, ch: &Channel, ep_num: u8) {
        let r = regs();
        match ch.index {
            0 => r
                .reg_uvc_ctrl()
                .modify(|_, w| unsafe { w.ch0_ep_num().bits(ep_num & 0xF) }),
            _ => r
                .reg_uvc_ctrl()
                .modify(|_, w| unsafe { w.ch1_ep_num().bits(ep_num & 0xF) }),
        };
    }

    /// Configure the frame-done trigger mode for a channel.
    pub fn set_eof(&self, ch: &Channel, mode: FrameDoneMode) {
        let r = regs();
        let bit = mode == FrameDoneMode::Eof;
        match ch.index {
            0 => r
                .reg_uvc_ctrl()
                .modify(|_, w| w.ch0_frm_done_opt().bit(bit)),
            _ => r
                .reg_uvc_ctrl()
                .modify(|_, w| w.ch1_frm_done_opt().bit(bit)),
        };
    }

    /// Enable or disable the concat mode for a channel.
    pub fn concat_en(&self, ch: &Channel, enable: bool) {
        let r = regs();
        match ch.index {
            0 => r
                .reg_uvc_ctrl()
                .modify(|_, w| w.ch0_concat_en().bit(enable)),
            _ => r
                .reg_uvc_ctrl()
                .modify(|_, w| w.ch1_concat_en().bit(enable)),
        };
    }

    /// Enable or disable the EP auto-enable function for a channel.
    pub fn ep_auto_en(&self, ch: &Channel, enable: bool) {
        let r = regs();
        match ch.index {
            0 => r
                .reg_ch0_ep_ctrl()
                .modify(|_, w| w.ch0_ep_auto_en().bit(enable)),
            _ => r
                .reg_ch1_ep_ctrl()
                .modify(|_, w| w.ch1_ep_auto_en().bit(enable)),
        };
    }

    /// Set the bInterval value for a channel.
    pub fn set_interval(&self, ch: &Channel, binterval: u8) {
        let r = regs();
        match ch.index {
            0 => r
                .reg_ch0_ep_ctrl()
                .modify(|_, w| unsafe { w.ch0_ep_binterval().bits(binterval) }),
            _ => r
                .reg_ch1_ep_ctrl()
                .modify(|_, w| unsafe { w.ch1_ep_binterval().bits(binterval) }),
        };
    }

    /// Set the HCDMAi DMA address for a channel.
    pub fn set_hcdmai(&self, ch: &Channel, value: u32) {
        let r = regs();
        match ch.index {
            0 => r
                .reg_uvc_chnl0_ctrl()
                .write(|w| unsafe { w.ch0_dma_addr().bits(value) }),
            _ => r
                .reg_uvc_chnl1_ctrl()
                .write(|w| unsafe { w.ch1_dma_addr().bits(value) }),
        };
    }

    /// Set the frame buffer start and end addresses for a channel and buffer index.
    pub fn set_buffer_addr(&self, ch: &Channel, buf: BufIndex, start_addr: u32, end_addr: u32) {
        let r = regs();
        match (ch.index, buf) {
            (0, BufIndex::Buf0) => {
                r.reg_ch0_buf0_start_addr()
                    .write(|w| unsafe { w.ch0_buf0_start_addr().bits(start_addr) });
                r.reg_ch0_buf0_end_addr()
                    .write(|w| unsafe { w.ch0_buf0_end_addr().bits(end_addr) });
            }
            (0, BufIndex::Buf1) => {
                r.reg_ch0_buf1_start_addr()
                    .write(|w| unsafe { w.ch0_buf1_start_addr().bits(start_addr) });
                r.reg_ch0_buf1_end_addr()
                    .write(|w| unsafe { w.ch0_buf1_end_addr().bits(end_addr) });
            }
            (_, BufIndex::Buf0) => {
                r.reg_ch1_buf0_start_addr()
                    .write(|w| unsafe { w.ch1_buf0_start_addr().bits(start_addr) });
                r.reg_ch1_buf0_end_addr()
                    .write(|w| unsafe { w.ch1_buf0_end_addr().bits(end_addr) });
            }
            (_, BufIndex::Buf1) => {
                r.reg_ch1_buf1_start_addr()
                    .write(|w| unsafe { w.ch1_buf1_start_addr().bits(start_addr) });
                r.reg_ch1_buf1_end_addr()
                    .write(|w| unsafe { w.ch1_buf1_end_addr().bits(end_addr) });
            }
        }
    }

    // -----------------------------------------------------------------------
    // Per-channel register getters
    // -----------------------------------------------------------------------

    /// Get the frame buffer start address for a channel and buffer index.
    pub fn get_buffer_addr(&self, ch: &Channel, buf: BufIndex) -> u32 {
        let r = regs();
        match (ch.index, buf) {
            (0, BufIndex::Buf0) => r
                .reg_ch0_buf0_start_addr()
                .read()
                .ch0_buf0_start_addr()
                .bits(),
            (0, BufIndex::Buf1) => r
                .reg_ch0_buf1_start_addr()
                .read()
                .ch0_buf1_start_addr()
                .bits(),
            (_, BufIndex::Buf0) => r
                .reg_ch1_buf0_start_addr()
                .read()
                .ch1_buf0_start_addr()
                .bits(),
            (_, BufIndex::Buf1) => r
                .reg_ch1_buf1_start_addr()
                .read()
                .ch1_buf1_start_addr()
                .bits(),
        }
    }

    /// Get the received frame size for a channel and buffer index.
    pub fn get_frame_size(&self, ch: &Channel, buf: BufIndex) -> u32 {
        let r = regs();
        match (ch.index, buf) {
            (0, BufIndex::Buf0) => r.reg_ch0_buf0_frm_size().read().ch0_buf0_frm_size().bits(),
            (0, BufIndex::Buf1) => r.reg_ch0_buf1_frm_size().read().ch0_buf1_frm_size().bits(),
            (_, BufIndex::Buf0) => r.reg_ch1_buf0_frm_size().read().ch1_buf0_frm_size().bits(),
            (_, BufIndex::Buf1) => r.reg_ch1_buf1_frm_size().read().ch1_buf1_frm_size().bits(),
        }
    }

    /// Get the PTS value of the first isochronous packet in a frame.
    pub fn get_pts(&self, ch: &Channel, buf: BufIndex) -> u32 {
        let r = regs();
        match (ch.index, buf) {
            (0, BufIndex::Buf0) => r.reg_ch0_buf0_pts().read().ch0_buf0_pts().bits(),
            (0, BufIndex::Buf1) => r.reg_ch0_buf1_pts().read().ch0_buf1_pts().bits(),
            (_, BufIndex::Buf0) => r.reg_ch1_buf0_pts().read().ch1_buf0_pts().bits(),
            (_, BufIndex::Buf1) => r.reg_ch1_buf1_pts().read().ch1_buf1_pts().bits(),
        }
    }

    /// Get the SCR value of the first isochronous packet in a frame.
    pub fn get_scr_first(&self, ch: &Channel, buf: BufIndex) -> u32 {
        let r = regs();
        match (ch.index, buf) {
            (0, BufIndex::Buf0) => r
                .reg_ch0_buf0_scr_first()
                .read()
                .ch0_buf0_scr_first()
                .bits(),
            (0, BufIndex::Buf1) => r
                .reg_ch0_buf1_scr_first()
                .read()
                .ch0_buf1_scr_first()
                .bits(),
            (_, BufIndex::Buf0) => r
                .reg_ch1_buf0_scr_first()
                .read()
                .ch1_buf0_scr_first()
                .bits(),
            (_, BufIndex::Buf1) => r
                .reg_ch1_buf1_scr_first()
                .read()
                .ch1_buf1_scr_first()
                .bits(),
        }
    }

    /// Get the SCR value of the last isochronous packet in a frame.
    pub fn get_scr_last(&self, ch: &Channel, buf: BufIndex) -> u32 {
        let r = regs();
        match (ch.index, buf) {
            (0, BufIndex::Buf0) => r.reg_ch0_buf0_scr_last().read().ch0_buf0_scr_last().bits(),
            (0, BufIndex::Buf1) => r.reg_ch0_buf1_scr_last().read().ch0_buf1_scr_last().bits(),
            (_, BufIndex::Buf0) => r.reg_ch1_buf0_scr_last().read().ch1_buf0_scr_last().bits(),
            (_, BufIndex::Buf1) => r.reg_ch1_buf1_scr_last().read().ch1_buf1_scr_last().bits(),
        }
    }

    // -----------------------------------------------------------------------
    // Command register programming
    // -----------------------------------------------------------------------

    /// Write a command slot for the given channel.
    pub fn set_cmd(&self, ch: &Channel, cmd_num: usize, cmd: &CmdReg) {
        assert!(cmd_num < MAX_CMD_NUM);
        let r = regs();
        let ofst = cmd.offset;
        let ctype = cmd.cmd_type;
        let en = cmd.enable;
        let data = cmd.data;

        match ch.index {
            0 => match cmd_num {
                0 => {
                    r.reg_ch0_cmd0_ctrl().write(|w| unsafe {
                        w.ch0_cmd0_ofst()
                            .bits(ofst)
                            .ch0_cmd0_type()
                            .bits(ctype)
                            .ch0_cmd0_en()
                            .bit(en)
                    });
                    r.reg_ch0_cmd0_data()
                        .write(|w| unsafe { w.ch0_cmd0_data().bits(data) });
                }
                1 => {
                    r.reg_ch0_cmd1_ctrl().write(|w| unsafe {
                        w.ch0_cmd1_ofst()
                            .bits(ofst)
                            .ch0_cmd1_type()
                            .bits(ctype)
                            .ch0_cmd1_en()
                            .bit(en)
                    });
                    r.reg_ch0_cmd1_data()
                        .write(|w| unsafe { w.ch0_cmd1_data().bits(data) });
                }
                2 => {
                    r.reg_ch0_cmd2_ctrl().write(|w| unsafe {
                        w.ch0_cmd2_ofst()
                            .bits(ofst)
                            .ch0_cmd2_type()
                            .bits(ctype)
                            .ch0_cmd2_en()
                            .bit(en)
                    });
                    r.reg_ch0_cmd2_data()
                        .write(|w| unsafe { w.ch0_cmd2_data().bits(data) });
                }
                3 => {
                    r.reg_ch0_cmd3_ctrl().write(|w| unsafe {
                        w.ch0_cmd3_ofst()
                            .bits(ofst)
                            .ch0_cmd3_type()
                            .bits(ctype)
                            .ch0_cmd3_en()
                            .bit(en)
                    });
                    r.reg_ch0_cmd3_data()
                        .write(|w| unsafe { w.ch0_cmd3_data().bits(data) });
                }
                4 => {
                    r.reg_ch0_cmd4_ctrl().write(|w| unsafe {
                        w.ch0_cmd4_ofst()
                            .bits(ofst)
                            .ch0_cmd4_type()
                            .bits(ctype)
                            .ch0_cmd4_en()
                            .bit(en)
                    });
                    r.reg_ch0_cmd4_data()
                        .write(|w| unsafe { w.ch0_cmd4_data().bits(data) });
                }
                _ => {
                    r.reg_ch0_cmd5_ctrl().write(|w| unsafe {
                        w.ch0_cmd5_ofst()
                            .bits(ofst)
                            .ch0_cmd5_type()
                            .bits(ctype)
                            .ch0_cmd5_en()
                            .bit(en)
                    });
                    r.reg_ch0_cmd5_data()
                        .write(|w| unsafe { w.ch0_cmd5_data().bits(data) });
                }
            },
            _ => match cmd_num {
                0 => {
                    r.reg_ch1_cmd0_ctrl().write(|w| unsafe {
                        w.ch1_cmd0_ofst()
                            .bits(ofst)
                            .ch1_cmd0_type()
                            .bits(ctype)
                            .ch1_cmd0_en()
                            .bit(en)
                    });
                    r.reg_ch1_cmd0_data()
                        .write(|w| unsafe { w.ch1_cmd0_data().bits(data) });
                }
                1 => {
                    r.reg_ch1_cmd1_ctrl().write(|w| unsafe {
                        w.ch1_cmd1_ofst()
                            .bits(ofst)
                            .ch1_cmd1_type()
                            .bits(ctype)
                            .ch1_cmd1_en()
                            .bit(en)
                    });
                    r.reg_ch1_cmd1_data()
                        .write(|w| unsafe { w.ch1_cmd1_data().bits(data) });
                }
                2 => {
                    r.reg_ch1_cmd2_ctrl().write(|w| unsafe {
                        w.ch1_cmd2_ofst()
                            .bits(ofst)
                            .ch1_cmd2_type()
                            .bits(ctype)
                            .ch1_cmd2_en()
                            .bit(en)
                    });
                    r.reg_ch1_cmd2_data()
                        .write(|w| unsafe { w.ch1_cmd2_data().bits(data) });
                }
                3 => {
                    r.reg_ch1_cmd3_ctrl().write(|w| unsafe {
                        w.ch1_cmd3_ofst()
                            .bits(ofst)
                            .ch1_cmd3_type()
                            .bits(ctype)
                            .ch1_cmd3_en()
                            .bit(en)
                    });
                    r.reg_ch1_cmd3_data()
                        .write(|w| unsafe { w.ch1_cmd3_data().bits(data) });
                }
                4 => {
                    r.reg_ch1_cmd4_ctrl().write(|w| unsafe {
                        w.ch1_cmd4_ofst()
                            .bits(ofst)
                            .ch1_cmd4_type()
                            .bits(ctype)
                            .ch1_cmd4_en()
                            .bit(en)
                    });
                    r.reg_ch1_cmd4_data()
                        .write(|w| unsafe { w.ch1_cmd4_data().bits(data) });
                }
                _ => {
                    r.reg_ch1_cmd5_ctrl().write(|w| unsafe {
                        w.ch1_cmd5_ofst()
                            .bits(ofst)
                            .ch1_cmd5_type()
                            .bits(ctype)
                            .ch1_cmd5_en()
                            .bit(en)
                    });
                    r.reg_ch1_cmd5_data()
                        .write(|w| unsafe { w.ch1_cmd5_data().bits(data) });
                }
            },
        }
    }

    // -----------------------------------------------------------------------
    // High-level channel initialisation
    // -----------------------------------------------------------------------

    /// Initialise a channel using a [`ChannelConfig`].
    ///
    /// Applies EP number, DMA address, frame buffers, interval, EOF mode,
    /// EP auto-enable, and all 6 command slots in one call.
    pub fn channel_init(&self, ch: &Channel, cfg: &ChannelConfig) {
        self.set_ep_num(ch, cfg.ep_num);
        self.set_hcdmai(ch, cfg.hcdmai);
        self.set_buffer_addr(ch, BufIndex::Buf0, cfg.buf0_start, cfg.buf0_end);
        self.set_buffer_addr(ch, BufIndex::Buf1, cfg.buf1_start, cfg.buf1_end);
        self.set_interval(ch, cfg.binterval);
        self.set_eof(ch, cfg.eof);
        self.ep_auto_en(ch, cfg.ep_auto_en);
        for (i, cmd) in cfg.cmds.iter().enumerate() {
            self.set_cmd(ch, i, cmd);
        }
    }
}
