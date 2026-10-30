#[doc = "Register `DBGT_CRV1` reader"]
pub type R = crate::R<DbgtCrv1Spec>;
#[doc = "Register `DBGT_CRV1` writer"]
pub type W = crate::W<DbgtCrv1Spec>;
#[doc = "Field `dbgt_crv1` reader - dbgtcnt\\[63:32\\].Continuing to count from 0 once it's reached 32'hFFFFFFFF"]
pub type DbgtCrv1R = crate::FieldReader<u32>;
#[doc = "Field `dbgt_crv1` writer - dbgtcnt\\[63:32\\].Continuing to count from 0 once it's reached 32'hFFFFFFFF"]
pub type DbgtCrv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dbgtcnt\\[63:32\\].Continuing to count from 0 once it's reached 32'hFFFFFFFF"]
    #[inline(always)]
    pub fn dbgt_crv1(&self) -> DbgtCrv1R {
        DbgtCrv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dbgtcnt\\[63:32\\].Continuing to count from 0 once it's reached 32'hFFFFFFFF"]
    #[inline(always)]
    pub fn dbgt_crv1(&mut self) -> DbgtCrv1W<'_, DbgtCrv1Spec> {
        DbgtCrv1W::new(self, 0)
    }
}
#[doc = "debugtimer count register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgt_crv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgt_crv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgtCrv1Spec;
impl crate::RegisterSpec for DbgtCrv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgt_crv1::R`](R) reader structure"]
impl crate::Readable for DbgtCrv1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgt_crv1::W`](W) writer structure"]
impl crate::Writable for DbgtCrv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGT_CRV1 to value 0"]
impl crate::Resettable for DbgtCrv1Spec {}
