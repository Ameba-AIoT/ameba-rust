#[doc = "Register `DMA_RST` reader"]
pub type R = crate::R<DmaRstSpec>;
#[doc = "Register `DMA_RST` writer"]
pub type W = crate::W<DmaRstSpec>;
#[doc = "Field `dma_rstn` reader - dma soft reset."]
pub type DmaRstnR = crate::BitReader;
#[doc = "Field `dma_rstn` writer - dma soft reset."]
pub type DmaRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_cfg` reader - Fix software Reset bug on the sdio_dbus module 1'b0: don't soft reset pe_tx_leng_re/pe_rx_leng_re 1'b1: soft reset pe_tx_leng_rx/pe_rx_leng_re"]
pub type DmaCfgR = crate::BitReader;
#[doc = "Field `dma_cfg` writer - Fix software Reset bug on the sdio_dbus module 1'b0: don't soft reset pe_tx_leng_re/pe_rx_leng_re 1'b1: soft reset pe_tx_leng_rx/pe_rx_leng_re"]
pub type DmaCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_stop` reader - Set this bit to stop dma, (suggest running dma_stop before doing software reset to avoid DMA hang)"]
pub type DmaStopR = crate::BitReader;
#[doc = "Field `dma_stop` writer - Set this bit to stop dma, (suggest running dma_stop before doing software reset to avoid DMA hang)"]
pub type DmaStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_rst_ack_ctl` reader - 1'b0: dma acknowledge soft reset after dbus_req done 1'b1: dma always acknowledge soft reset regardless of dbus_req"]
pub type DmaRstAckCtlR = crate::BitReader;
#[doc = "Field `dma_rst_ack_ctl` writer - 1'b0: dma acknowledge soft reset after dbus_req done 1'b1: dma always acknowledge soft reset regardless of dbus_req"]
pub type DmaRstAckCtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cur_pe_dma_fsm` reader - {1'b0, cur_pe_dma_fsm} for debug"]
pub type CurPeDmaFsmR = crate::FieldReader;
#[doc = "Field `dmabus_info` reader - {1'b0, dma_state\\[2:0\\], 1'b0, dbus_state\\[2:0\\]} for debug"]
pub type DmabusInfoR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - dma soft reset."]
    #[inline(always)]
    pub fn dma_rstn(&self) -> DmaRstnR {
        DmaRstnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fix software Reset bug on the sdio_dbus module 1'b0: don't soft reset pe_tx_leng_re/pe_rx_leng_re 1'b1: soft reset pe_tx_leng_rx/pe_rx_leng_re"]
    #[inline(always)]
    pub fn dma_cfg(&self) -> DmaCfgR {
        DmaCfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to stop dma, (suggest running dma_stop before doing software reset to avoid DMA hang)"]
    #[inline(always)]
    pub fn dma_stop(&self) -> DmaStopR {
        DmaStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1'b0: dma acknowledge soft reset after dbus_req done 1'b1: dma always acknowledge soft reset regardless of dbus_req"]
    #[inline(always)]
    pub fn dma_rst_ack_ctl(&self) -> DmaRstAckCtlR {
        DmaRstAckCtlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - {1'b0, cur_pe_dma_fsm} for debug"]
    #[inline(always)]
    pub fn cur_pe_dma_fsm(&self) -> CurPeDmaFsmR {
        CurPeDmaFsmR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - {1'b0, dma_state\\[2:0\\], 1'b0, dbus_state\\[2:0\\]} for debug"]
    #[inline(always)]
    pub fn dmabus_info(&self) -> DmabusInfoR {
        DmabusInfoR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - dma soft reset."]
    #[inline(always)]
    pub fn dma_rstn(&mut self) -> DmaRstnW<'_, DmaRstSpec> {
        DmaRstnW::new(self, 0)
    }
    #[doc = "Bit 1 - Fix software Reset bug on the sdio_dbus module 1'b0: don't soft reset pe_tx_leng_re/pe_rx_leng_re 1'b1: soft reset pe_tx_leng_rx/pe_rx_leng_re"]
    #[inline(always)]
    pub fn dma_cfg(&mut self) -> DmaCfgW<'_, DmaRstSpec> {
        DmaCfgW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to stop dma, (suggest running dma_stop before doing software reset to avoid DMA hang)"]
    #[inline(always)]
    pub fn dma_stop(&mut self) -> DmaStopW<'_, DmaRstSpec> {
        DmaStopW::new(self, 2)
    }
    #[doc = "Bit 3 - 1'b0: dma acknowledge soft reset after dbus_req done 1'b1: dma always acknowledge soft reset regardless of dbus_req"]
    #[inline(always)]
    pub fn dma_rst_ack_ctl(&mut self) -> DmaRstAckCtlW<'_, DmaRstSpec> {
        DmaRstAckCtlW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRstSpec;
impl crate::RegisterSpec for DmaRstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rst::R`](R) reader structure"]
impl crate::Readable for DmaRstSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_rst::W`](W) writer structure"]
impl crate::Writable for DmaRstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RST to value 0x0101"]
impl crate::Resettable for DmaRstSpec {
    const RESET_VALUE: u32 = 0x0101;
}
