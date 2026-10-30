#[doc = "Register `REG_DBGTIM_DUMMY` reader"]
pub type R = crate::R<RegDbgtimDummySpec>;
#[doc = "Register `REG_DBGTIM_DUMMY` writer"]
pub type W = crate::W<RegDbgtimDummySpec>;
#[doc = "Field `dummy` reader - dummy register"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - dummy register"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dummy register"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dummy register"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegDbgtimDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "Debug TimerDummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dbgtim_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dbgtim_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDbgtimDummySpec;
impl crate::RegisterSpec for RegDbgtimDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dbgtim_dummy::R`](R) reader structure"]
impl crate::Readable for RegDbgtimDummySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dbgtim_dummy::W`](W) writer structure"]
impl crate::Writable for RegDbgtimDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DBGTIM_DUMMY to value 0xffff"]
impl crate::Resettable for RegDbgtimDummySpec {
    const RESET_VALUE: u32 = 0xffff;
}
