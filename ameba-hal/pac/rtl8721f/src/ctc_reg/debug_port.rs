#[doc = "Register `DEBUG_PORT` reader"]
pub type R = crate::R<DebugPortSpec>;
#[doc = "Field `dbg_port` reader - Debug port output value,depend on dbg_sel value"]
pub type DbgPortR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Debug port output value,depend on dbg_sel value"]
    #[inline(always)]
    pub fn dbg_port(&self) -> DbgPortR {
        DbgPortR::new(self.bits)
    }
}
#[doc = "Debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_port::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugPortSpec;
impl crate::RegisterSpec for DebugPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_port::R`](R) reader structure"]
impl crate::Readable for DebugPortSpec {}
#[doc = "`reset()` method sets DEBUG_PORT to value 0"]
impl crate::Resettable for DebugPortSpec {}
