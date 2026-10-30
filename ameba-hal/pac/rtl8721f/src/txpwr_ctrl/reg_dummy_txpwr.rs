#[doc = "Register `REG_DUMMY_TXPWR` reader"]
pub type R = crate::R<RegDummyTxpwrSpec>;
#[doc = "Register `REG_DUMMY_TXPWR` writer"]
pub type W = crate::W<RegDummyTxpwrSpec>;
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
    pub fn dummy(&mut self) -> DummyW<'_, RegDummyTxpwrSpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy_txpwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy_txpwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummyTxpwrSpec;
impl crate::RegisterSpec for RegDummyTxpwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy_txpwr::R`](R) reader structure"]
impl crate::Readable for RegDummyTxpwrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy_txpwr::W`](W) writer structure"]
impl crate::Writable for RegDummyTxpwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY_TXPWR to value 0"]
impl crate::Resettable for RegDummyTxpwrSpec {}
