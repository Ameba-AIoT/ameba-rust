#[doc = "Register `REG_ADC_SETTLE` reader"]
pub type R = crate::R<RegAdcSettleSpec>;
#[doc = "Register `REG_ADC_SETTLE` writer"]
pub type W = crate::W<RegAdcSettleSpec>;
#[doc = "Field `bit_settle_ctc_to_adc` reader - settle count for sample data from captouch to ADC, the unit is adc sample clock. 0 means no need to settle"]
pub type BitSettleCtcToAdcR = crate::FieldReader;
#[doc = "Field `bit_settle_ctc_to_adc` writer - settle count for sample data from captouch to ADC, the unit is adc sample clock. 0 means no need to settle"]
pub type BitSettleCtcToAdcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_settle_pow_on` reader - settle count for sample data from power on , the unit is adc sample clock. 0 means no need to settle"]
pub type BitSettlePowOnR = crate::FieldReader;
#[doc = "Field `bit_settle_pow_on` writer - settle count for sample data from power on , the unit is adc sample clock. 0 means no need to settle"]
pub type BitSettlePowOnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - settle count for sample data from captouch to ADC, the unit is adc sample clock. 0 means no need to settle"]
    #[inline(always)]
    pub fn bit_settle_ctc_to_adc(&self) -> BitSettleCtcToAdcR {
        BitSettleCtcToAdcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - settle count for sample data from power on , the unit is adc sample clock. 0 means no need to settle"]
    #[inline(always)]
    pub fn bit_settle_pow_on(&self) -> BitSettlePowOnR {
        BitSettlePowOnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - settle count for sample data from captouch to ADC, the unit is adc sample clock. 0 means no need to settle"]
    #[inline(always)]
    pub fn bit_settle_ctc_to_adc(&mut self) -> BitSettleCtcToAdcW<'_, RegAdcSettleSpec> {
        BitSettleCtcToAdcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - settle count for sample data from power on , the unit is adc sample clock. 0 means no need to settle"]
    #[inline(always)]
    pub fn bit_settle_pow_on(&mut self) -> BitSettlePowOnW<'_, RegAdcSettleSpec> {
        BitSettlePowOnW::new(self, 4)
    }
}
#[doc = "ADC Settle Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_settle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_settle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcSettleSpec;
impl crate::RegisterSpec for RegAdcSettleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_settle::R`](R) reader structure"]
impl crate::Readable for RegAdcSettleSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_settle::W`](W) writer structure"]
impl crate::Writable for RegAdcSettleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_SETTLE to value 0"]
impl crate::Resettable for RegAdcSettleSpec {}
