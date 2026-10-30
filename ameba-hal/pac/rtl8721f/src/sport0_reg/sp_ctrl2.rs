#[doc = "Register `SP_CTRL2` reader"]
pub type R = crate::R<SpCtrl2Spec>;
#[doc = "Register `SP_CTRL2` writer"]
pub type W = crate::W<SpCtrl2Spec>;
#[doc = "Field `lrc_stop_thresh` reader - either tx or rx excat phase counter > this value, generate lrc_stop_intr"]
pub type LrcStopThreshR = crate::FieldReader<u16>;
#[doc = "Field `lrc_stop_thresh` writer - either tx or rx excat phase counter > this value, generate lrc_stop_intr"]
pub type LrcStopThreshW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ex_phase_cnt_en` reader - enable t/rx exact phase counter, counted by sport_clk"]
pub type ExPhaseCntEnR = crate::BitReader;
#[doc = "Field `ex_phase_cnt_en` writer - enable t/rx exact phase counter, counted by sport_clk"]
pub type ExPhaseCntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lrc_start_intr_mask` reader - "]
pub type LrcStartIntrMaskR = crate::BitReader;
#[doc = "Field `lrc_start_intr_mask` writer - "]
pub type LrcStartIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lrc_stop_intr_mask` reader - "]
pub type LrcStopIntrMaskR = crate::BitReader;
#[doc = "Field `lrc_stop_intr_mask` writer - "]
pub type LrcStopIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_dma_single_low` reader - disable tx_dma_single"]
pub type TxDmaSingleLowR = crate::BitReader;
#[doc = "Field `tx_dma_single_low` writer - disable tx_dma_single"]
pub type TxDmaSingleLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_dma_single_low` reader - disable rx_dma_single"]
pub type RxDmaSingleLowR = crate::BitReader;
#[doc = "Field `rx_dma_single_low` writer - disable rx_dma_single"]
pub type RxDmaSingleLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lrc_start_pol` reader - 1: lrclk rising edge as lrc start 0: lrclk falling edge as lrc start"]
pub type LrcStartPolR = crate::BitReader;
#[doc = "Field `lrc_start_pol` writer - 1: lrclk rising edge as lrc start 0: lrclk falling edge as lrc start"]
pub type LrcStartPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:15 - either tx or rx excat phase counter > this value, generate lrc_stop_intr"]
    #[inline(always)]
    pub fn lrc_stop_thresh(&self) -> LrcStopThreshR {
        LrcStopThreshR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - enable t/rx exact phase counter, counted by sport_clk"]
    #[inline(always)]
    pub fn ex_phase_cnt_en(&self) -> ExPhaseCntEnR {
        ExPhaseCntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lrc_start_intr_mask(&self) -> LrcStartIntrMaskR {
        LrcStartIntrMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lrc_stop_intr_mask(&self) -> LrcStopIntrMaskR {
        LrcStopIntrMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - disable tx_dma_single"]
    #[inline(always)]
    pub fn tx_dma_single_low(&self) -> TxDmaSingleLowR {
        TxDmaSingleLowR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - disable rx_dma_single"]
    #[inline(always)]
    pub fn rx_dma_single_low(&self) -> RxDmaSingleLowR {
        RxDmaSingleLowR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: lrclk rising edge as lrc start 0: lrclk falling edge as lrc start"]
    #[inline(always)]
    pub fn lrc_start_pol(&self) -> LrcStartPolR {
        LrcStartPolR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - either tx or rx excat phase counter > this value, generate lrc_stop_intr"]
    #[inline(always)]
    pub fn lrc_stop_thresh(&mut self) -> LrcStopThreshW<'_, SpCtrl2Spec> {
        LrcStopThreshW::new(self, 0)
    }
    #[doc = "Bit 16 - enable t/rx exact phase counter, counted by sport_clk"]
    #[inline(always)]
    pub fn ex_phase_cnt_en(&mut self) -> ExPhaseCntEnW<'_, SpCtrl2Spec> {
        ExPhaseCntEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lrc_start_intr_mask(&mut self) -> LrcStartIntrMaskW<'_, SpCtrl2Spec> {
        LrcStartIntrMaskW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lrc_stop_intr_mask(&mut self) -> LrcStopIntrMaskW<'_, SpCtrl2Spec> {
        LrcStopIntrMaskW::new(self, 18)
    }
    #[doc = "Bit 24 - disable tx_dma_single"]
    #[inline(always)]
    pub fn tx_dma_single_low(&mut self) -> TxDmaSingleLowW<'_, SpCtrl2Spec> {
        TxDmaSingleLowW::new(self, 24)
    }
    #[doc = "Bit 25 - disable rx_dma_single"]
    #[inline(always)]
    pub fn rx_dma_single_low(&mut self) -> RxDmaSingleLowW<'_, SpCtrl2Spec> {
        RxDmaSingleLowW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: lrclk rising edge as lrc start 0: lrclk falling edge as lrc start"]
    #[inline(always)]
    pub fn lrc_start_pol(&mut self) -> LrcStartPolW<'_, SpCtrl2Spec> {
        LrcStartPolW::new(self, 26)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, SpCtrl2Spec> {
        DummyW::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpCtrl2Spec;
impl crate::RegisterSpec for SpCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_ctrl2::R`](R) reader structure"]
impl crate::Readable for SpCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_ctrl2::W`](W) writer structure"]
impl crate::Writable for SpCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_CTRL2 to value 0"]
impl crate::Resettable for SpCtrl2Spec {}
