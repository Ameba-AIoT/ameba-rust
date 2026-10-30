#[doc = "Register `DBGT_SCRATCH` reader"]
pub type R = crate::R<DbgtScratchSpec>;
#[doc = "Register `DBGT_SCRATCH` writer"]
pub type W = crate::W<DbgtScratchSpec>;
#[doc = "Field `dbgt_scratch` reader - Reserved for user use"]
pub type DbgtScratchR = crate::FieldReader<u32>;
#[doc = "Field `dbgt_scratch` writer - Reserved for user use"]
pub type DbgtScratchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved for user use"]
    #[inline(always)]
    pub fn dbgt_scratch(&self) -> DbgtScratchR {
        DbgtScratchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for user use"]
    #[inline(always)]
    pub fn dbgt_scratch(&mut self) -> DbgtScratchW<'_, DbgtScratchSpec> {
        DbgtScratchW::new(self, 0)
    }
}
#[doc = "debugtimer scratch\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgt_scratch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgt_scratch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgtScratchSpec;
impl crate::RegisterSpec for DbgtScratchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgt_scratch::R`](R) reader structure"]
impl crate::Readable for DbgtScratchSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgt_scratch::W`](W) writer structure"]
impl crate::Writable for DbgtScratchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGT_SCRATCH to value 0"]
impl crate::Resettable for DbgtScratchSpec {}
