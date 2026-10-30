#[doc = "Register `GPIO_DUMMY` reader"]
pub type R = crate::R<GpioDummySpec>;
#[doc = "Field `DUMMY` reader - No real reg, only for run_espec.py"]
pub type DummyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - No real reg, only for run_espec.py"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dummy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDummySpec;
impl crate::RegisterSpec for GpioDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_dummy::R`](R) reader structure"]
impl crate::Readable for GpioDummySpec {}
#[doc = "`reset()` method sets GPIO_DUMMY to value 0"]
impl crate::Resettable for GpioDummySpec {}
