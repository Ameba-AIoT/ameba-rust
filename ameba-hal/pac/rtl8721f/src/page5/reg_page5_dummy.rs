#[doc = "Register `REG_PAGE5_DUMMY` reader"]
pub type R = crate::R<RegPage5DummySpec>;
#[doc = "Register `REG_PAGE5_DUMMY` writer"]
pub type W = crate::W<RegPage5DummySpec>;
#[doc = "Field `BIT_DUMMY` reader - Dummy"]
pub type BitDummyR = crate::FieldReader<u32>;
#[doc = "Field `BIT_DUMMY` writer - Dummy"]
pub type BitDummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dummy"]
    #[inline(always)]
    pub fn bit_dummy(&self) -> BitDummyR {
        BitDummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dummy"]
    #[inline(always)]
    pub fn bit_dummy(&mut self) -> BitDummyW<'_, RegPage5DummySpec> {
        BitDummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_page5_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_page5_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPage5DummySpec;
impl crate::RegisterSpec for RegPage5DummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_page5_dummy::R`](R) reader structure"]
impl crate::Readable for RegPage5DummySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_page5_dummy::W`](W) writer structure"]
impl crate::Writable for RegPage5DummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PAGE5_DUMMY to value 0xffff_0000"]
impl crate::Resettable for RegPage5DummySpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
