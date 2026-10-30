#[doc = "Register `REG_MACCLKFRQ` reader"]
pub type R = crate::R<RegMacclkfrqSpec>;
#[doc = "Register `REG_MACCLKFRQ` writer"]
pub type W = crate::W<RegMacclkfrqSpec>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegMacclkfrqSpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "MAC clk frequency value register for 32k circuit\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macclkfrq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macclkfrq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacclkfrqSpec;
impl crate::RegisterSpec for RegMacclkfrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macclkfrq::R`](R) reader structure"]
impl crate::Readable for RegMacclkfrqSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_macclkfrq::W`](W) writer structure"]
impl crate::Writable for RegMacclkfrqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACCLKFRQ to value 0"]
impl crate::Resettable for RegMacclkfrqSpec {}
