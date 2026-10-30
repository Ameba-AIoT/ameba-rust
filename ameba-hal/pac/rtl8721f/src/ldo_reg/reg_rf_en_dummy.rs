#[doc = "Register `REG_RF_EN_dummy` reader"]
pub type R = crate::R<RegRfEnDummySpec>;
#[doc = "Register `REG_RF_EN_dummy` writer"]
pub type W = crate::W<RegRfEnDummySpec>;
#[doc = "Field `REG_POW_4MOSC` reader - // \\[RF\\]\\[4MOSC\\] dummy"]
pub type RegPow4moscR = crate::BitReader;
#[doc = "Field `REG_POW_4MOSC` writer - // \\[RF\\]\\[4MOSC\\] dummy"]
pub type RegPow4moscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - // \\[RF\\]\\[4MOSC\\] dummy"]
    #[inline(always)]
    pub fn reg_pow_4mosc(&self) -> RegPow4moscR {
        RegPow4moscR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - // \\[RF\\]\\[4MOSC\\] dummy"]
    #[inline(always)]
    pub fn reg_pow_4mosc(&mut self) -> RegPow4moscW<'_, RegRfEnDummySpec> {
        RegPow4moscW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rf_en_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rf_en_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRfEnDummySpec;
impl crate::RegisterSpec for RegRfEnDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rf_en_dummy::R`](R) reader structure"]
impl crate::Readable for RegRfEnDummySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rf_en_dummy::W`](W) writer structure"]
impl crate::Writable for RegRfEnDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RF_EN_dummy to value 0"]
impl crate::Resettable for RegRfEnDummySpec {}
