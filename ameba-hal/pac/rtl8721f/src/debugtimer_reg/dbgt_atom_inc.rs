#[doc = "Register `DBGT_ATOM_INC` reader"]
pub type R = crate::R<DbgtAtomIncSpec>;
#[doc = "Field `dbgt_atom_inc` reader - Each read will increase 1 and return to the increased value to bus."]
pub type DbgtAtomIncR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Each read will increase 1 and return to the increased value to bus."]
    #[inline(always)]
    pub fn dbgt_atom_inc(&self) -> DbgtAtomIncR {
        DbgtAtomIncR::new(self.bits)
    }
}
#[doc = "debugtimer atom read increase\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgt_atom_inc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgtAtomIncSpec;
impl crate::RegisterSpec for DbgtAtomIncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgt_atom_inc::R`](R) reader structure"]
impl crate::Readable for DbgtAtomIncSpec {}
#[doc = "`reset()` method sets DBGT_ATOM_INC to value 0"]
impl crate::Resettable for DbgtAtomIncSpec {}
