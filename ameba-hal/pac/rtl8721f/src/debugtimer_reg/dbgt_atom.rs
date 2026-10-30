#[doc = "Register `DBGT_ATOM` reader"]
pub type R = crate::R<DbgtAtomSpec>;
#[doc = "Register `DBGT_ATOM` writer"]
pub type W = crate::W<DbgtAtomSpec>;
#[doc = "Field `dbgt_atom` reader - Return the last increased value of atom counter"]
pub type DbgtAtomR = crate::FieldReader<u32>;
#[doc = "Field `dbgt_atom` writer - Return the last increased value of atom counter"]
pub type DbgtAtomW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Return the last increased value of atom counter"]
    #[inline(always)]
    pub fn dbgt_atom(&self) -> DbgtAtomR {
        DbgtAtomR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Return the last increased value of atom counter"]
    #[inline(always)]
    pub fn dbgt_atom(&mut self) -> DbgtAtomW<'_, DbgtAtomSpec> {
        DbgtAtomW::new(self, 0)
    }
}
#[doc = "debugtimer atom\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgt_atom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgt_atom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgtAtomSpec;
impl crate::RegisterSpec for DbgtAtomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgt_atom::R`](R) reader structure"]
impl crate::Readable for DbgtAtomSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgt_atom::W`](W) writer structure"]
impl crate::Writable for DbgtAtomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGT_ATOM to value 0"]
impl crate::Resettable for DbgtAtomSpec {}
