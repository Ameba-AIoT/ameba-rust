#[doc = "Register `TM_ADC_RESULT` reader"]
pub type R = crate::R<TmAdcResultSpec>;
#[doc = "Field `tm_adc_out` reader - This bit defines thermal meter ADC output. The result refreshes in real time. * ADC output is a positive decimal number, real output=tm_out/2^22"]
pub type TmAdcOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - This bit defines thermal meter ADC output. The result refreshes in real time. * ADC output is a positive decimal number, real output=tm_out/2^22"]
    #[inline(always)]
    pub fn tm_adc_out(&self) -> TmAdcOutR {
        TmAdcOutR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Thermal Meter ADC Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_adc_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmAdcResultSpec;
impl crate::RegisterSpec for TmAdcResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_adc_result::R`](R) reader structure"]
impl crate::Readable for TmAdcResultSpec {}
#[doc = "`reset()` method sets TM_ADC_RESULT to value 0"]
impl crate::Resettable for TmAdcResultSpec {}
