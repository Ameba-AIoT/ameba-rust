#[doc = "Register `REG_BCNERR_TSF_CTRL` reader"]
pub type R = crate::R<RegBcnerrTsfCtrlSpec>;
#[doc = "Register `REG_BCNERR_TSF_CTRL` writer"]
pub type W = crate::W<RegBcnerrTsfCtrlSpec>;
#[doc = "Field `BIT_RXTRIG_DELAY` reader - add compensation of RX trigger's TSFT. Unit: us"]
pub type BitRxtrigDelayR = crate::FieldReader;
#[doc = "Field `BIT_RXTRIG_DELAY` writer - add compensation of RX trigger's TSFT. Unit: us"]
pub type BitRxtrigDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCN_MAX_ERR` reader - When beacon TX error counter goes over this threshold, Beacon will transmit within PIFS."]
pub type BitBcnMaxErrR = crate::FieldReader;
#[doc = "Field `BIT_BCN_MAX_ERR` writer - When beacon TX error counter goes over this threshold, Beacon will transmit within PIFS."]
pub type BitBcnMaxErrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_CCK_RXTSF_OFFSET` reader - The Datapath of CCK RX_tsf"]
pub type BitCckRxtsfOffsetR = crate::FieldReader;
#[doc = "Field `BIT_CCK_RXTSF_OFFSET` writer - The Datapath of CCK RX_tsf"]
pub type BitCckRxtsfOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_OFDM_RXTSF_OFFSET` reader - The Datapath of OFDM RX_tsf"]
pub type BitOfdmRxtsfOffsetR = crate::FieldReader;
#[doc = "Field `BIT_OFDM_RXTSF_OFFSET` writer - The Datapath of OFDM RX_tsf"]
pub type BitOfdmRxtsfOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - add compensation of RX trigger's TSFT. Unit: us"]
    #[inline(always)]
    pub fn bit_rxtrig_delay(&self) -> BitRxtrigDelayR {
        BitRxtrigDelayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - When beacon TX error counter goes over this threshold, Beacon will transmit within PIFS."]
    #[inline(always)]
    pub fn bit_bcn_max_err(&self) -> BitBcnMaxErrR {
        BitBcnMaxErrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The Datapath of CCK RX_tsf"]
    #[inline(always)]
    pub fn bit_cck_rxtsf_offset(&self) -> BitCckRxtsfOffsetR {
        BitCckRxtsfOffsetR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The Datapath of OFDM RX_tsf"]
    #[inline(always)]
    pub fn bit_ofdm_rxtsf_offset(&self) -> BitOfdmRxtsfOffsetR {
        BitOfdmRxtsfOffsetR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - add compensation of RX trigger's TSFT. Unit: us"]
    #[inline(always)]
    pub fn bit_rxtrig_delay(&mut self) -> BitRxtrigDelayW<'_, RegBcnerrTsfCtrlSpec> {
        BitRxtrigDelayW::new(self, 0)
    }
    #[doc = "Bits 8:15 - When beacon TX error counter goes over this threshold, Beacon will transmit within PIFS."]
    #[inline(always)]
    pub fn bit_bcn_max_err(&mut self) -> BitBcnMaxErrW<'_, RegBcnerrTsfCtrlSpec> {
        BitBcnMaxErrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - The Datapath of CCK RX_tsf"]
    #[inline(always)]
    pub fn bit_cck_rxtsf_offset(&mut self) -> BitCckRxtsfOffsetW<'_, RegBcnerrTsfCtrlSpec> {
        BitCckRxtsfOffsetW::new(self, 16)
    }
    #[doc = "Bits 24:31 - The Datapath of OFDM RX_tsf"]
    #[inline(always)]
    pub fn bit_ofdm_rxtsf_offset(&mut self) -> BitOfdmRxtsfOffsetW<'_, RegBcnerrTsfCtrlSpec> {
        BitOfdmRxtsfOffsetW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcnerr_tsf_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcnerr_tsf_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnerrTsfCtrlSpec;
impl crate::RegisterSpec for RegBcnerrTsfCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcnerr_tsf_ctrl::R`](R) reader structure"]
impl crate::Readable for RegBcnerrTsfCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcnerr_tsf_ctrl::W`](W) writer structure"]
impl crate::Writable for RegBcnerrTsfCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCNERR_TSF_CTRL to value 0x0f0f_1000"]
impl crate::Resettable for RegBcnerrTsfCtrlSpec {
    const RESET_VALUE: u32 = 0x0f0f_1000;
}
