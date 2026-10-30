#[doc = "Register `TM_RESULT` reader"]
pub type R = crate::R<TmResultSpec>;
#[doc = "Field `tm_out` reader - This bit defines real temperature. The result refreshes in real time. * Bit18: Signed bit * Bit\\[17:10\\]: Integer part * Bit\\[9:0\\]: Decimal part"]
pub type TmOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:18 - This bit defines real temperature. The result refreshes in real time. * Bit18: Signed bit * Bit\\[17:10\\]: Integer part * Bit\\[9:0\\]: Decimal part"]
    #[inline(always)]
    pub fn tm_out(&self) -> TmOutR {
        TmOutR::new(self.bits & 0x0007_ffff)
    }
}
#[doc = "Thermal Meter Temperature Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmResultSpec;
impl crate::RegisterSpec for TmResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_result::R`](R) reader structure"]
impl crate::Readable for TmResultSpec {}
#[doc = "`reset()` method sets TM_RESULT to value 0"]
impl crate::Resettable for TmResultSpec {}
