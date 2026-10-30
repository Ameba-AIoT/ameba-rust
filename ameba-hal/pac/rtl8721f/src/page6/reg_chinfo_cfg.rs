#[doc = "Register `REG_CHINFO_CFG` reader"]
pub type R = crate::R<RegChinfoCfgSpec>;
#[doc = "Register `REG_CHINFO_CFG` writer"]
pub type W = crate::W<RegChinfoCfgSpec>;
#[doc = "Field `r_CHINFO_TX_ADDR_h` reader - high 16bit tx address for trigger channel info"]
pub type RChinfoTxAddrHR = crate::FieldReader<u16>;
#[doc = "Field `r_CHINFO_TX_ADDR_h` writer - high 16bit tx address for trigger channel info"]
pub type RChinfoTxAddrHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_CHINFO_EN` reader - enable for getting channel info from BB. When set to 0, will not trigger channel info function"]
pub type RChinfoEnR = crate::BitReader;
#[doc = "Field `r_CHINFO_EN` writer - enable for getting channel info from BB. When set to 0, will not trigger channel info function"]
pub type RChinfoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_CHINFO_MODE` reader - channel info mode setting. 0: enable normal rx trigger; 1: enable ndpa trigger; note: repsonse mode decided by tx descriptor. Valid when r_CHINFO_EN is 1."]
pub type RChinfoModeR = crate::FieldReader;
#[doc = "Field `r_CHINFO_MODE` writer - channel info mode setting. 0: enable normal rx trigger; 1: enable ndpa trigger; note: repsonse mode decided by tx descriptor. Valid when r_CHINFO_EN is 1."]
pub type RChinfoModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_chinfo_a1_en` reader - 1: rx address1 must match the self mac address 0: do not care of unicast rx address1"]
pub type RChinfoA1EnR = crate::BitReader;
#[doc = "Field `r_chinfo_a1_en` writer - 1: rx address1 must match the self mac address 0: do not care of unicast rx address1"]
pub type RChinfoA1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_chinfo_txaddr_en` reader - 1: rx address2 must match the tx address for trigger channel info, except ack and cts 0: do not care of rx address2"]
pub type RChinfoTxaddrEnR = crate::BitReader;
#[doc = "Field `r_chinfo_txaddr_en` writer - 1: rx address2 must match the tx address for trigger channel info, except ack and cts 0: do not care of rx address2"]
pub type RChinfoTxaddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_CHINFO_TIMER` reader - Wait timeout value after repuest channe info, unit 4 mac clock cycles"]
pub type RChinfoTimerR = crate::FieldReader;
#[doc = "Field `r_CHINFO_TIMER` writer - Wait timeout value after repuest channe info, unit 4 mac clock cycles"]
pub type RChinfoTimerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_CSI_BUFF_FULL` reader - Indicate rxcsibuff full occur"]
pub type RCsiBuffFullR = crate::BitReader;
#[doc = "Field `r_CHINFO_TIME_OUT` reader - BB send channel info length is less than report length"]
pub type RChinfoTimeOutR = crate::BitReader;
#[doc = "Field `r_CHINFO_WTM_OUT` reader - BB does't send channel info when timeout occur"]
pub type RChinfoWtmOutR = crate::BitReader;
#[doc = "Field `r_chinfo_fcs` reader - when set to 1, will not check channel info fcs ok flag"]
pub type RChinfoFcsR = crate::BitReader;
#[doc = "Field `r_chinfo_fcs` writer - when set to 1, will not check channel info fcs ok flag"]
pub type RChinfoFcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_CHINFO_DROP_INVALID` reader - when set to 1, will not drop channe info if valid bit of bb header is 0"]
pub type RChinfoDropInvalidR = crate::BitReader;
#[doc = "Field `r_CHINFO_DROP_INVALID` writer - when set to 1, will not drop channe info if valid bit of bb header is 0"]
pub type RChinfoDropInvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_chinfo_content_en` reader - used for rx normal mode 1: the payload(excluding mac header/iv/eiv/icv/mic/fcs) of the specified location must match the configured value 0: do not check the payload value"]
pub type RChinfoContentEnR = crate::BitReader;
#[doc = "Field `r_chinfo_content_en` writer - used for rx normal mode 1: the payload(excluding mac header/iv/eiv/icv/mic/fcs) of the specified location must match the configured value 0: do not check the payload value"]
pub type RChinfoContentEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - high 16bit tx address for trigger channel info"]
    #[inline(always)]
    pub fn r_chinfo_tx_addr_h(&self) -> RChinfoTxAddrHR {
        RChinfoTxAddrHR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - enable for getting channel info from BB. When set to 0, will not trigger channel info function"]
    #[inline(always)]
    pub fn r_chinfo_en(&self) -> RChinfoEnR {
        RChinfoEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - channel info mode setting. 0: enable normal rx trigger; 1: enable ndpa trigger; note: repsonse mode decided by tx descriptor. Valid when r_CHINFO_EN is 1."]
    #[inline(always)]
    pub fn r_chinfo_mode(&self) -> RChinfoModeR {
        RChinfoModeR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - 1: rx address1 must match the self mac address 0: do not care of unicast rx address1"]
    #[inline(always)]
    pub fn r_chinfo_a1_en(&self) -> RChinfoA1EnR {
        RChinfoA1EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: rx address2 must match the tx address for trigger channel info, except ack and cts 0: do not care of rx address2"]
    #[inline(always)]
    pub fn r_chinfo_txaddr_en(&self) -> RChinfoTxaddrEnR {
        RChinfoTxaddrEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:24 - Wait timeout value after repuest channe info, unit 4 mac clock cycles"]
    #[inline(always)]
    pub fn r_chinfo_timer(&self) -> RChinfoTimerR {
        RChinfoTimerR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - Indicate rxcsibuff full occur"]
    #[inline(always)]
    pub fn r_csi_buff_full(&self) -> RCsiBuffFullR {
        RCsiBuffFullR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BB send channel info length is less than report length"]
    #[inline(always)]
    pub fn r_chinfo_time_out(&self) -> RChinfoTimeOutR {
        RChinfoTimeOutR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BB does't send channel info when timeout occur"]
    #[inline(always)]
    pub fn r_chinfo_wtm_out(&self) -> RChinfoWtmOutR {
        RChinfoWtmOutR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - when set to 1, will not check channel info fcs ok flag"]
    #[inline(always)]
    pub fn r_chinfo_fcs(&self) -> RChinfoFcsR {
        RChinfoFcsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - when set to 1, will not drop channe info if valid bit of bb header is 0"]
    #[inline(always)]
    pub fn r_chinfo_drop_invalid(&self) -> RChinfoDropInvalidR {
        RChinfoDropInvalidR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - used for rx normal mode 1: the payload(excluding mac header/iv/eiv/icv/mic/fcs) of the specified location must match the configured value 0: do not check the payload value"]
    #[inline(always)]
    pub fn r_chinfo_content_en(&self) -> RChinfoContentEnR {
        RChinfoContentEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - high 16bit tx address for trigger channel info"]
    #[inline(always)]
    pub fn r_chinfo_tx_addr_h(&mut self) -> RChinfoTxAddrHW<'_, RegChinfoCfgSpec> {
        RChinfoTxAddrHW::new(self, 0)
    }
    #[doc = "Bit 16 - enable for getting channel info from BB. When set to 0, will not trigger channel info function"]
    #[inline(always)]
    pub fn r_chinfo_en(&mut self) -> RChinfoEnW<'_, RegChinfoCfgSpec> {
        RChinfoEnW::new(self, 16)
    }
    #[doc = "Bits 17:18 - channel info mode setting. 0: enable normal rx trigger; 1: enable ndpa trigger; note: repsonse mode decided by tx descriptor. Valid when r_CHINFO_EN is 1."]
    #[inline(always)]
    pub fn r_chinfo_mode(&mut self) -> RChinfoModeW<'_, RegChinfoCfgSpec> {
        RChinfoModeW::new(self, 17)
    }
    #[doc = "Bit 20 - 1: rx address1 must match the self mac address 0: do not care of unicast rx address1"]
    #[inline(always)]
    pub fn r_chinfo_a1_en(&mut self) -> RChinfoA1EnW<'_, RegChinfoCfgSpec> {
        RChinfoA1EnW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: rx address2 must match the tx address for trigger channel info, except ack and cts 0: do not care of rx address2"]
    #[inline(always)]
    pub fn r_chinfo_txaddr_en(&mut self) -> RChinfoTxaddrEnW<'_, RegChinfoCfgSpec> {
        RChinfoTxaddrEnW::new(self, 21)
    }
    #[doc = "Bits 22:24 - Wait timeout value after repuest channe info, unit 4 mac clock cycles"]
    #[inline(always)]
    pub fn r_chinfo_timer(&mut self) -> RChinfoTimerW<'_, RegChinfoCfgSpec> {
        RChinfoTimerW::new(self, 22)
    }
    #[doc = "Bit 28 - when set to 1, will not check channel info fcs ok flag"]
    #[inline(always)]
    pub fn r_chinfo_fcs(&mut self) -> RChinfoFcsW<'_, RegChinfoCfgSpec> {
        RChinfoFcsW::new(self, 28)
    }
    #[doc = "Bit 29 - when set to 1, will not drop channe info if valid bit of bb header is 0"]
    #[inline(always)]
    pub fn r_chinfo_drop_invalid(&mut self) -> RChinfoDropInvalidW<'_, RegChinfoCfgSpec> {
        RChinfoDropInvalidW::new(self, 29)
    }
    #[doc = "Bit 30 - used for rx normal mode 1: the payload(excluding mac header/iv/eiv/icv/mic/fcs) of the specified location must match the configured value 0: do not check the payload value"]
    #[inline(always)]
    pub fn r_chinfo_content_en(&mut self) -> RChinfoContentEnW<'_, RegChinfoCfgSpec> {
        RChinfoContentEnW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_chinfo_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_chinfo_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChinfoCfgSpec;
impl crate::RegisterSpec for RegChinfoCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_chinfo_cfg::R`](R) reader structure"]
impl crate::Readable for RegChinfoCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_chinfo_cfg::W`](W) writer structure"]
impl crate::Writable for RegChinfoCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CHINFO_CFG to value 0x0130_0000"]
impl crate::Resettable for RegChinfoCfgSpec {
    const RESET_VALUE: u32 = 0x0130_0000;
}
