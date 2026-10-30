#[doc = "Register `REG_APMC_VEC` reader"]
pub type R = crate::R<RegApmcVecSpec>;
#[doc = "Field `cur_vec` reader - regu pmc all intr vector"]
pub type CurVecR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - regu pmc all intr vector"]
    #[inline(always)]
    pub fn cur_vec(&self) -> CurVecR {
        CurVecR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_apmc_vec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegApmcVecSpec;
impl crate::RegisterSpec for RegApmcVecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_apmc_vec::R`](R) reader structure"]
impl crate::Readable for RegApmcVecSpec {}
#[doc = "`reset()` method sets REG_APMC_VEC to value 0"]
impl crate::Resettable for RegApmcVecSpec {}
