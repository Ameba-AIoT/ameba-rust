#[doc = "Register `REG_ADC_PWR_ON_CNT` reader"]
pub type R = crate::R<RegAdcPwrOnCntSpec>;
#[doc = "Register `REG_ADC_PWR_ON_CNT` writer"]
pub type W = crate::W<RegAdcPwrOnCntSpec>;
#[doc = "Field `adc_pwron_cnt` reader - power ad delay control. Base on 4M cycle default 150us"]
pub type AdcPwronCntR = crate::FieldReader<u16>;
#[doc = "Field `adc_pwron_cnt` writer - power ad delay control. Base on 4M cycle default 150us"]
pub type AdcPwronCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - power ad delay control. Base on 4M cycle default 150us"]
    #[inline(always)]
    pub fn adc_pwron_cnt(&self) -> AdcPwronCntR {
        AdcPwronCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - power ad delay control. Base on 4M cycle default 150us"]
    #[inline(always)]
    pub fn adc_pwron_cnt(&mut self) -> AdcPwronCntW<'_, RegAdcPwrOnCntSpec> {
        AdcPwronCntW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_pwr_on_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_pwr_on_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcPwrOnCntSpec;
impl crate::RegisterSpec for RegAdcPwrOnCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_pwr_on_cnt::R`](R) reader structure"]
impl crate::Readable for RegAdcPwrOnCntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_pwr_on_cnt::W`](W) writer structure"]
impl crate::Writable for RegAdcPwrOnCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_PWR_ON_CNT to value 0x0258"]
impl crate::Resettable for RegAdcPwrOnCntSpec {
    const RESET_VALUE: u32 = 0x0258;
}
