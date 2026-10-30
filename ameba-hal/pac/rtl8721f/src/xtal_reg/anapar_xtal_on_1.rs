#[doc = "Register `ANAPAR_XTAL_ON_1` reader"]
pub type R = crate::R<AnaparXtalOn1Spec>;
#[doc = "Register `ANAPAR_XTAL_ON_1` writer"]
pub type W = crate::W<AnaparXtalOn1Spec>;
#[doc = "Field `XTAL_SC_LPS` reader - 0: reduce cap, 1: increase cap. (160fF per unit) \\[efuse for BT only \\] adjust further only if there is a frequency offset"]
pub type XtalScLpsR = crate::FieldReader;
#[doc = "Field `XTAL_SC_LPS` writer - 0: reduce cap, 1: increase cap. (160fF per unit) \\[efuse for BT only \\] adjust further only if there is a frequency offset"]
pub type XtalScLpsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN_XTAL_DRV_RF1` reader - RF1 enable clock source control, 1:enable, 0: gated (for WL SYN)"]
pub type EnXtalDrvRf1R = crate::BitReader;
#[doc = "Field `EN_XTAL_DRV_RF1` writer - RF1 enable clock source control, 1:enable, 0: gated (for WL SYN)"]
pub type EnXtalDrvRf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_DRV_RF2` reader - RF2 enable clock source control, 1:enable, 0: gated (for BT)"]
pub type EnXtalDrvRf2R = crate::BitReader;
#[doc = "Field `EN_XTAL_DRV_RF2` writer - RF2 enable clock source control, 1:enable, 0: gated (for BT)"]
pub type EnXtalDrvRf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_DRV_AFE` reader - AFE enable clock source control, 1:enable, 0: gated (for WL AFE PLL)"]
pub type EnXtalDrvAfeR = crate::BitReader;
#[doc = "Field `EN_XTAL_DRV_AFE` writer - AFE enable clock source control, 1:enable, 0: gated (for WL AFE PLL)"]
pub type EnXtalDrvAfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_DRV_USB` reader - USB enable clock source control, 1:enable, 0: gated"]
pub type EnXtalDrvUsbR = crate::BitReader;
#[doc = "Field `EN_XTAL_DRV_USB` writer - USB enable clock source control, 1:enable, 0: gated"]
pub type EnXtalDrvUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_DRV_DIGI` reader - DIGI enable clock source control, 1:enable, 0: gated"]
pub type EnXtalDrvDigiR = crate::BitReader;
#[doc = "Field `EN_XTAL_DRV_DIGI` writer - DIGI enable clock source control, 1:enable, 0: gated"]
pub type EnXtalDrvDigiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_DRV_LPS` reader - LPS enable clock source control, 1:enable, 0: gated Same 312.5KHz enable as anapar_xtal_on\\[51\\]"]
pub type EnXtalDrvLpsR = crate::BitReader;
#[doc = "Field `EN_XTAL_DRV_LPS` writer - LPS enable clock source control, 1:enable, 0: gated Same 312.5KHz enable as anapar_xtal_on\\[51\\]"]
pub type EnXtalDrvLpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_SEL_TOK` reader - XTAL start-up process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.025m / 0.05 001: 2^11 / 0.05m / 0.1m 010: 2^12 / 0.1m / 0.2m 011: 2^13 / 0.2m / 0.3m 100: 2^14 / 0.4m / 0.6m 101: 2^15 / 0.8m / 1.2m 110: 2^16 / 1.6m / 2.5m 111: 2^17 / 3.2m / 4.9m XTAL mode switch process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.0125m / 0.025 001: 2^11 / 0.025m / 0.05m 010: 2^12 / 0.05m / 0.1m 011: 2^13 / 0.1m / 0.2m 100: 2^14 / 0.2m / 0.3m 101: 2^15 / 0.4m / 0.6m 110: 2^16 / 0.8m / 1.25m 111: 2^17 / 1.6m / 2.45m"]
pub type XtalSelTokR = crate::FieldReader;
#[doc = "Field `XTAL_SEL_TOK` writer - XTAL start-up process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.025m / 0.05 001: 2^11 / 0.05m / 0.1m 010: 2^12 / 0.1m / 0.2m 011: 2^13 / 0.2m / 0.3m 100: 2^14 / 0.4m / 0.6m 101: 2^15 / 0.8m / 1.2m 110: 2^16 / 1.6m / 2.5m 111: 2^17 / 3.2m / 4.9m XTAL mode switch process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.0125m / 0.025 001: 2^11 / 0.025m / 0.05m 010: 2^12 / 0.05m / 0.1m 011: 2^13 / 0.1m / 0.2m 100: 2^14 / 0.2m / 0.3m 101: 2^15 / 0.4m / 0.6m 110: 2^16 / 0.8m / 1.25m 111: 2^17 / 1.6m / 2.45m"]
pub type XtalSelTokW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XTAL_LDO_VREF` reader - (before start-up) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
pub type XtalLdoVrefR = crate::FieldReader;
#[doc = "Field `XTAL_LDO_VREF` writer - (before start-up) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
pub type XtalLdoVrefW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EN_XTAL_SCHMITT` reader - 1: enable XTAL SCHMITT"]
pub type EnXtalSchmittR = crate::BitReader;
#[doc = "Field `EN_XTAL_SCHMITT` writer - 1: enable XTAL SCHMITT"]
pub type EnXtalSchmittW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_LPS_CLK` reader - 1: enable XTAL LPS clock Same 312.5KHz enable as anapar_xtal_on\\[43\\]"]
pub type EnXtalLpsClkR = crate::BitReader;
#[doc = "Field `EN_XTAL_LPS_CLK` writer - 1: enable XTAL LPS clock Same 312.5KHz enable as anapar_xtal_on\\[43\\]"]
pub type EnXtalLpsClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_CKDIGI_SEL` reader - 1: DIGI CLK from self-bias 0: DIGI CLK from Schmitt"]
pub type XtalCkdigiSelR = crate::BitReader;
#[doc = "Field `XTAL_CKDIGI_SEL` writer - 1: DIGI CLK from self-bias 0: DIGI CLK from Schmitt"]
pub type XtalCkdigiSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_LPS_DIVISOR` reader - XTAL LPS clock divider 0: XTAL/128(40M), 1: XTAL/64"]
pub type XtalLpsDivisorR = crate::BitReader;
#[doc = "Field `XTAL_LPS_DIVISOR` writer - XTAL LPS clock divider 0: XTAL/128(40M), 1: XTAL/64"]
pub type XtalLpsDivisorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_VREF_SEL` reader - Analog AAC VREF, 390mV~700mV, 10mV/step 11000: Vpp 500mV"]
pub type XtalVrefSelR = crate::FieldReader;
#[doc = "Field `XTAL_VREF_SEL` writer - Analog AAC VREF, 390mV~700mV, 10mV/step 11000: Vpp 500mV"]
pub type XtalVrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `XTAL_AAC_OPCUR` reader - 00: 200nA , 200n/step For AAC robust 11"]
pub type XtalAacOpcurR = crate::FieldReader;
#[doc = "Field `XTAL_AAC_OPCUR` writer - 00: 200nA , 200n/step For AAC robust 11"]
pub type XtalAacOpcurW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `xtal_dummy2` reader - Reserved for future use."]
pub type XtalDummy2R = crate::FieldReader;
#[doc = "Field `xtal_dummy2` writer - Reserved for future use."]
pub type XtalDummy2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - 0: reduce cap, 1: increase cap. (160fF per unit) \\[efuse for BT only \\] adjust further only if there is a frequency offset"]
    #[inline(always)]
    pub fn xtal_sc_lps(&self) -> XtalScLpsR {
        XtalScLpsR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RF1 enable clock source control, 1:enable, 0: gated (for WL SYN)"]
    #[inline(always)]
    pub fn en_xtal_drv_rf1(&self) -> EnXtalDrvRf1R {
        EnXtalDrvRf1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RF2 enable clock source control, 1:enable, 0: gated (for BT)"]
    #[inline(always)]
    pub fn en_xtal_drv_rf2(&self) -> EnXtalDrvRf2R {
        EnXtalDrvRf2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AFE enable clock source control, 1:enable, 0: gated (for WL AFE PLL)"]
    #[inline(always)]
    pub fn en_xtal_drv_afe(&self) -> EnXtalDrvAfeR {
        EnXtalDrvAfeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB enable clock source control, 1:enable, 0: gated"]
    #[inline(always)]
    pub fn en_xtal_drv_usb(&self) -> EnXtalDrvUsbR {
        EnXtalDrvUsbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIGI enable clock source control, 1:enable, 0: gated"]
    #[inline(always)]
    pub fn en_xtal_drv_digi(&self) -> EnXtalDrvDigiR {
        EnXtalDrvDigiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPS enable clock source control, 1:enable, 0: gated Same 312.5KHz enable as anapar_xtal_on\\[51\\]"]
    #[inline(always)]
    pub fn en_xtal_drv_lps(&self) -> EnXtalDrvLpsR {
        EnXtalDrvLpsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - XTAL start-up process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.025m / 0.05 001: 2^11 / 0.05m / 0.1m 010: 2^12 / 0.1m / 0.2m 011: 2^13 / 0.2m / 0.3m 100: 2^14 / 0.4m / 0.6m 101: 2^15 / 0.8m / 1.2m 110: 2^16 / 1.6m / 2.5m 111: 2^17 / 3.2m / 4.9m XTAL mode switch process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.0125m / 0.025 001: 2^11 / 0.025m / 0.05m 010: 2^12 / 0.05m / 0.1m 011: 2^13 / 0.1m / 0.2m 100: 2^14 / 0.2m / 0.3m 101: 2^15 / 0.4m / 0.6m 110: 2^16 / 0.8m / 1.25m 111: 2^17 / 1.6m / 2.45m"]
    #[inline(always)]
    pub fn xtal_sel_tok(&self) -> XtalSelTokR {
        XtalSelTokR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - (before start-up) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
    #[inline(always)]
    pub fn xtal_ldo_vref(&self) -> XtalLdoVrefR {
        XtalLdoVrefR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 1: enable XTAL SCHMITT"]
    #[inline(always)]
    pub fn en_xtal_schmitt(&self) -> EnXtalSchmittR {
        EnXtalSchmittR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: enable XTAL LPS clock Same 312.5KHz enable as anapar_xtal_on\\[43\\]"]
    #[inline(always)]
    pub fn en_xtal_lps_clk(&self) -> EnXtalLpsClkR {
        EnXtalLpsClkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: DIGI CLK from self-bias 0: DIGI CLK from Schmitt"]
    #[inline(always)]
    pub fn xtal_ckdigi_sel(&self) -> XtalCkdigiSelR {
        XtalCkdigiSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XTAL LPS clock divider 0: XTAL/128(40M), 1: XTAL/64"]
    #[inline(always)]
    pub fn xtal_lps_divisor(&self) -> XtalLpsDivisorR {
        XtalLpsDivisorR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - Analog AAC VREF, 390mV~700mV, 10mV/step 11000: Vpp 500mV"]
    #[inline(always)]
    pub fn xtal_vref_sel(&self) -> XtalVrefSelR {
        XtalVrefSelR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - 00: 200nA , 200n/step For AAC robust 11"]
    #[inline(always)]
    pub fn xtal_aac_opcur(&self) -> XtalAacOpcurR {
        XtalAacOpcurR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Reserved for future use."]
    #[inline(always)]
    pub fn xtal_dummy2(&self) -> XtalDummy2R {
        XtalDummy2R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 0: reduce cap, 1: increase cap. (160fF per unit) \\[efuse for BT only \\] adjust further only if there is a frequency offset"]
    #[inline(always)]
    pub fn xtal_sc_lps(&mut self) -> XtalScLpsW<'_, AnaparXtalOn1Spec> {
        XtalScLpsW::new(self, 0)
    }
    #[doc = "Bit 7 - RF1 enable clock source control, 1:enable, 0: gated (for WL SYN)"]
    #[inline(always)]
    pub fn en_xtal_drv_rf1(&mut self) -> EnXtalDrvRf1W<'_, AnaparXtalOn1Spec> {
        EnXtalDrvRf1W::new(self, 7)
    }
    #[doc = "Bit 8 - RF2 enable clock source control, 1:enable, 0: gated (for BT)"]
    #[inline(always)]
    pub fn en_xtal_drv_rf2(&mut self) -> EnXtalDrvRf2W<'_, AnaparXtalOn1Spec> {
        EnXtalDrvRf2W::new(self, 8)
    }
    #[doc = "Bit 9 - AFE enable clock source control, 1:enable, 0: gated (for WL AFE PLL)"]
    #[inline(always)]
    pub fn en_xtal_drv_afe(&mut self) -> EnXtalDrvAfeW<'_, AnaparXtalOn1Spec> {
        EnXtalDrvAfeW::new(self, 9)
    }
    #[doc = "Bit 10 - USB enable clock source control, 1:enable, 0: gated"]
    #[inline(always)]
    pub fn en_xtal_drv_usb(&mut self) -> EnXtalDrvUsbW<'_, AnaparXtalOn1Spec> {
        EnXtalDrvUsbW::new(self, 10)
    }
    #[doc = "Bit 11 - DIGI enable clock source control, 1:enable, 0: gated"]
    #[inline(always)]
    pub fn en_xtal_drv_digi(&mut self) -> EnXtalDrvDigiW<'_, AnaparXtalOn1Spec> {
        EnXtalDrvDigiW::new(self, 11)
    }
    #[doc = "Bit 12 - LPS enable clock source control, 1:enable, 0: gated Same 312.5KHz enable as anapar_xtal_on\\[51\\]"]
    #[inline(always)]
    pub fn en_xtal_drv_lps(&mut self) -> EnXtalDrvLpsW<'_, AnaparXtalOn1Spec> {
        EnXtalDrvLpsW::new(self, 12)
    }
    #[doc = "Bits 13:15 - XTAL start-up process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.025m / 0.05 001: 2^11 / 0.05m / 0.1m 010: 2^12 / 0.1m / 0.2m 011: 2^13 / 0.2m / 0.3m 100: 2^14 / 0.4m / 0.6m 101: 2^15 / 0.8m / 1.2m 110: 2^16 / 1.6m / 2.5m 111: 2^17 / 3.2m / 4.9m XTAL mode switch process reg value: cycles / 40MHz / 26MHz 000: 2^10 / 0.0125m / 0.025 001: 2^11 / 0.025m / 0.05m 010: 2^12 / 0.05m / 0.1m 011: 2^13 / 0.1m / 0.2m 100: 2^14 / 0.2m / 0.3m 101: 2^15 / 0.4m / 0.6m 110: 2^16 / 0.8m / 1.25m 111: 2^17 / 1.6m / 2.45m"]
    #[inline(always)]
    pub fn xtal_sel_tok(&mut self) -> XtalSelTokW<'_, AnaparXtalOn1Spec> {
        XtalSelTokW::new(self, 13)
    }
    #[doc = "Bits 16:18 - (before start-up) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
    #[inline(always)]
    pub fn xtal_ldo_vref(&mut self) -> XtalLdoVrefW<'_, AnaparXtalOn1Spec> {
        XtalLdoVrefW::new(self, 16)
    }
    #[doc = "Bit 19 - 1: enable XTAL SCHMITT"]
    #[inline(always)]
    pub fn en_xtal_schmitt(&mut self) -> EnXtalSchmittW<'_, AnaparXtalOn1Spec> {
        EnXtalSchmittW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: enable XTAL LPS clock Same 312.5KHz enable as anapar_xtal_on\\[43\\]"]
    #[inline(always)]
    pub fn en_xtal_lps_clk(&mut self) -> EnXtalLpsClkW<'_, AnaparXtalOn1Spec> {
        EnXtalLpsClkW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: DIGI CLK from self-bias 0: DIGI CLK from Schmitt"]
    #[inline(always)]
    pub fn xtal_ckdigi_sel(&mut self) -> XtalCkdigiSelW<'_, AnaparXtalOn1Spec> {
        XtalCkdigiSelW::new(self, 21)
    }
    #[doc = "Bit 22 - XTAL LPS clock divider 0: XTAL/128(40M), 1: XTAL/64"]
    #[inline(always)]
    pub fn xtal_lps_divisor(&mut self) -> XtalLpsDivisorW<'_, AnaparXtalOn1Spec> {
        XtalLpsDivisorW::new(self, 22)
    }
    #[doc = "Bits 23:27 - Analog AAC VREF, 390mV~700mV, 10mV/step 11000: Vpp 500mV"]
    #[inline(always)]
    pub fn xtal_vref_sel(&mut self) -> XtalVrefSelW<'_, AnaparXtalOn1Spec> {
        XtalVrefSelW::new(self, 23)
    }
    #[doc = "Bits 28:29 - 00: 200nA , 200n/step For AAC robust 11"]
    #[inline(always)]
    pub fn xtal_aac_opcur(&mut self) -> XtalAacOpcurW<'_, AnaparXtalOn1Spec> {
        XtalAacOpcurW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Reserved for future use."]
    #[inline(always)]
    pub fn xtal_dummy2(&mut self) -> XtalDummy2W<'_, AnaparXtalOn1Spec> {
        XtalDummy2W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_on_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_on_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalOn1Spec;
impl crate::RegisterSpec for AnaparXtalOn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_on_1::R`](R) reader structure"]
impl crate::Readable for AnaparXtalOn1Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_on_1::W`](W) writer structure"]
impl crate::Writable for AnaparXtalOn1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_ON_1 to value 0xfc39_9c80"]
impl crate::Resettable for AnaparXtalOn1Spec {
    const RESET_VALUE: u32 = 0xfc39_9c80;
}
