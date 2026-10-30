#[doc = "Register `adc_0_lpf_rd` reader"]
pub type R = crate::R<Adc0LpfRdSpec>;
#[doc = "Field `adc_0_lpf_rd` reader - CH 0 ADC LPF out values"]
pub type Adc0LpfRdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:18 - CH 0 ADC LPF out values"]
    #[inline(always)]
    pub fn adc_0_lpf_rd(&self) -> Adc0LpfRdR {
        Adc0LpfRdR::new(self.bits & 0x0007_ffff)
    }
}
#[doc = "ADC Channel 0 LPF RD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_0_lpf_rd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0LpfRdSpec;
impl crate::RegisterSpec for Adc0LpfRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_0_lpf_rd::R`](R) reader structure"]
impl crate::Readable for Adc0LpfRdSpec {}
#[doc = "`reset()` method sets adc_0_lpf_rd to value 0"]
impl crate::Resettable for Adc0LpfRdSpec {}
