#[doc = "Register `adc_1_lpf_rd` reader"]
pub type R = crate::R<Adc1LpfRdSpec>;
#[doc = "Field `adc_1_lpf_rd` reader - CH 1 ADC LPF out values"]
pub type Adc1LpfRdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:18 - CH 1 ADC LPF out values"]
    #[inline(always)]
    pub fn adc_1_lpf_rd(&self) -> Adc1LpfRdR {
        Adc1LpfRdR::new(self.bits & 0x0007_ffff)
    }
}
#[doc = "ADC Channel 1 LPF RD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1_lpf_rd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1LpfRdSpec;
impl crate::RegisterSpec for Adc1LpfRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_1_lpf_rd::R`](R) reader structure"]
impl crate::Readable for Adc1LpfRdSpec {}
#[doc = "`reset()` method sets adc_1_lpf_rd to value 0"]
impl crate::Resettable for Adc1LpfRdSpec {}
