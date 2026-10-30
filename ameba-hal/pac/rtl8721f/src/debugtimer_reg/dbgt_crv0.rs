#[doc = "Register `DBGT_CRV0` reader"]
pub type R = crate::R<DbgtCrv0Spec>;
#[doc = "Register `DBGT_CRV0` writer"]
pub type W = crate::W<DbgtCrv0Spec>;
#[doc = "Field `dbgt_crv0` reader - dbgtcnt\\[31:0\\].Continuing to count from 0 once it's reached 32'Hffffffff"]
pub type DbgtCrv0R = crate::FieldReader<u32>;
#[doc = "Field `dbgt_crv0` writer - dbgtcnt\\[31:0\\].Continuing to count from 0 once it's reached 32'Hffffffff"]
pub type DbgtCrv0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dbgtcnt\\[31:0\\].Continuing to count from 0 once it's reached 32'Hffffffff"]
    #[inline(always)]
    pub fn dbgt_crv0(&self) -> DbgtCrv0R {
        DbgtCrv0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dbgtcnt\\[31:0\\].Continuing to count from 0 once it's reached 32'Hffffffff"]
    #[inline(always)]
    pub fn dbgt_crv0(&mut self) -> DbgtCrv0W<'_, DbgtCrv0Spec> {
        DbgtCrv0W::new(self, 0)
    }
}
#[doc = "debugtimer count register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgt_crv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgt_crv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgtCrv0Spec;
impl crate::RegisterSpec for DbgtCrv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgt_crv0::R`](R) reader structure"]
impl crate::Readable for DbgtCrv0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgt_crv0::W`](W) writer structure"]
impl crate::Writable for DbgtCrv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGT_CRV0 to value 0"]
impl crate::Resettable for DbgtCrv0Spec {}
