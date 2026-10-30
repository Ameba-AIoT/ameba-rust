#[doc = "Register `HOST_DMABUF_CTL` reader"]
pub type R = crate::R<HostDmabufCtlSpec>;
#[doc = "Register `HOST_DMABUF_CTL` writer"]
pub type W = crate::W<HostDmabufCtlSpec>;
#[doc = "Field `auto_cmd23_ctl` reader - Auto Cmd23 error control 0: auto cmd23 response error stop next command transfer 1:auto cmd23 response error doesn't stop next command transfer"]
pub type AutoCmd23CtlR = crate::FieldReader;
#[doc = "Field `auto_cmd23_ctl` writer - Auto Cmd23 error control 0: auto cmd23 response error stop next command transfer 1:auto cmd23 response error doesn't stop next command transfer"]
pub type AutoCmd23CtlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `buf_room_vld_ctl` reader - buf_room_vld_ctl (2020/09/01) 0: byte-level buf_room_vld control 1: Qword-level(8-byte) buf_room_vld control to fix the PCIE_RING_BUF bug when ADMA length not equal to the multiple of 8 (to fix the same sram address read-write collision)"]
pub type BufRoomVldCtlR = crate::BitReader;
#[doc = "Field `buf_room_vld_ctl` writer - buf_room_vld_ctl (2020/09/01) 0: byte-level buf_room_vld control 1: Qword-level(8-byte) buf_room_vld control to fix the PCIE_RING_BUF bug when ADMA length not equal to the multiple of 8 (to fix the same sram address read-write collision)"]
pub type BufRoomVldCtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `buf_flush_ctl` reader - buf_flush_ctl (2020/09/04) 0: don't flush PCIE_RING_BUF and reset rd_ptr/wr_ptr at DMA mode 1: auto-flush PCIE_RING_BUF and reset rd_ptr/wr_ptr after each SDIO data transfer (to fix the same sram address read-write collision) at DMA Mode"]
pub type BufFlushCtlR = crate::BitReader;
#[doc = "Field `buf_flush_ctl` writer - buf_flush_ctl (2020/09/04) 0: don't flush PCIE_RING_BUF and reset rd_ptr/wr_ptr at DMA mode 1: auto-flush PCIE_RING_BUF and reset rd_ptr/wr_ptr after each SDIO data transfer (to fix the same sram address read-write collision) at DMA Mode"]
pub type BufFlushCtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `buf_flush_ctl_pio` reader - buf_flush_ctl_pio (2020/10/12) 0: don't flush PCIE_RING_BUF and reset rd_ptr/wr_ptr at PIO mode xxxxxx 1: auto-flush PCIE_RING_BUF and reset rd_prt/wr_ptr after each block transfer complete at PIO Mode"]
pub type BufFlushCtlPioR = crate::BitReader;
#[doc = "Field `buf_flush_ctl_pio` writer - buf_flush_ctl_pio (2020/10/12) 0: don't flush PCIE_RING_BUF and reset rd_ptr/wr_ptr at PIO mode xxxxxx 1: auto-flush PCIE_RING_BUF and reset rd_prt/wr_ptr after each block transfer complete at PIO Mode"]
pub type BufFlushCtlPioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Auto Cmd23 error control 0: auto cmd23 response error stop next command transfer 1:auto cmd23 response error doesn't stop next command transfer"]
    #[inline(always)]
    pub fn auto_cmd23_ctl(&self) -> AutoCmd23CtlR {
        AutoCmd23CtlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - buf_room_vld_ctl (2020/09/01) 0: byte-level buf_room_vld control 1: Qword-level(8-byte) buf_room_vld control to fix the PCIE_RING_BUF bug when ADMA length not equal to the multiple of 8 (to fix the same sram address read-write collision)"]
    #[inline(always)]
    pub fn buf_room_vld_ctl(&self) -> BufRoomVldCtlR {
        BufRoomVldCtlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - buf_flush_ctl (2020/09/04) 0: don't flush PCIE_RING_BUF and reset rd_ptr/wr_ptr at DMA mode 1: auto-flush PCIE_RING_BUF and reset rd_ptr/wr_ptr after each SDIO data transfer (to fix the same sram address read-write collision) at DMA Mode"]
    #[inline(always)]
    pub fn buf_flush_ctl(&self) -> BufFlushCtlR {
        BufFlushCtlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - buf_flush_ctl_pio (2020/10/12) 0: don't flush PCIE_RING_BUF and reset rd_ptr/wr_ptr at PIO mode xxxxxx 1: auto-flush PCIE_RING_BUF and reset rd_prt/wr_ptr after each block transfer complete at PIO Mode"]
    #[inline(always)]
    pub fn buf_flush_ctl_pio(&self) -> BufFlushCtlPioR {
        BufFlushCtlPioR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Auto Cmd23 error control 0: auto cmd23 response error stop next command transfer 1:auto cmd23 response error doesn't stop next command transfer"]
    #[inline(always)]
    pub fn auto_cmd23_ctl(&mut self) -> AutoCmd23CtlW<'_, HostDmabufCtlSpec> {
        AutoCmd23CtlW::new(self, 0)
    }
    #[doc = "Bit 4 - buf_room_vld_ctl (2020/09/01) 0: byte-level buf_room_vld control 1: Qword-level(8-byte) buf_room_vld control to fix the PCIE_RING_BUF bug when ADMA length not equal to the multiple of 8 (to fix the same sram address read-write collision)"]
    #[inline(always)]
    pub fn buf_room_vld_ctl(&mut self) -> BufRoomVldCtlW<'_, HostDmabufCtlSpec> {
        BufRoomVldCtlW::new(self, 4)
    }
    #[doc = "Bit 5 - buf_flush_ctl (2020/09/04) 0: don't flush PCIE_RING_BUF and reset rd_ptr/wr_ptr at DMA mode 1: auto-flush PCIE_RING_BUF and reset rd_ptr/wr_ptr after each SDIO data transfer (to fix the same sram address read-write collision) at DMA Mode"]
    #[inline(always)]
    pub fn buf_flush_ctl(&mut self) -> BufFlushCtlW<'_, HostDmabufCtlSpec> {
        BufFlushCtlW::new(self, 5)
    }
    #[doc = "Bit 6 - buf_flush_ctl_pio (2020/10/12) 0: don't flush PCIE_RING_BUF and reset rd_ptr/wr_ptr at PIO mode xxxxxx 1: auto-flush PCIE_RING_BUF and reset rd_prt/wr_ptr after each block transfer complete at PIO Mode"]
    #[inline(always)]
    pub fn buf_flush_ctl_pio(&mut self) -> BufFlushCtlPioW<'_, HostDmabufCtlSpec> {
        BufFlushCtlPioW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_dmabuf_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_dmabuf_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostDmabufCtlSpec;
impl crate::RegisterSpec for HostDmabufCtlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`host_dmabuf_ctl::R`](R) reader structure"]
impl crate::Readable for HostDmabufCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`host_dmabuf_ctl::W`](W) writer structure"]
impl crate::Writable for HostDmabufCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_DMABUF_CTL to value 0x70"]
impl crate::Resettable for HostDmabufCtlSpec {
    const RESET_VALUE: u16 = 0x70;
}
