#[doc = "Register `REG_PLL_STATE` reader"]
pub type R = crate::R<RegPllStateSpec>;
#[doc = "Register `REG_PLL_STATE` writer"]
pub type W = crate::W<RegPllStateSpec>;
#[doc = "Field `ckrdy_pll` reader - PLL clock is ready"]
pub type CkrdyPllR = crate::BitReader;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - PLL clock is ready"]
    #[inline(always)]
    pub fn ckrdy_pll(&self) -> CkrdyPllR {
        CkrdyPllR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegPllStateSpec> {
        DummyW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pll_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pll_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPllStateSpec;
impl crate::RegisterSpec for RegPllStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pll_state::R`](R) reader structure"]
impl crate::Readable for RegPllStateSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pll_state::W`](W) writer structure"]
impl crate::Writable for RegPllStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PLL_STATE to value 0x000f_0000"]
impl crate::Resettable for RegPllStateSpec {
    const RESET_VALUE: u32 = 0x000f_0000;
}
