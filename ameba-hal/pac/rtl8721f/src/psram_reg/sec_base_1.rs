#[doc = "Register `SEC_BASE_1` reader"]
pub type R = crate::R<SecBase1Spec>;
#[doc = "Register `SEC_BASE_1` writer"]
pub type W = crate::W<SecBase1Spec>;
#[doc = "Field `SEC_BASE1` reader - SEC region 1 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase1R = crate::FieldReader<u32>;
#[doc = "Field `SEC_BASE1` writer - SEC region 1 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - SEC region 1 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base1(&self) -> SecBase1R {
        SecBase1R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - SEC region 1 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base1(&mut self) -> SecBase1W<'_, SecBase1Spec> {
        SecBase1W::new(self, 12)
    }
}
#[doc = "SEC region 1 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_base_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_base_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecBase1Spec;
impl crate::RegisterSpec for SecBase1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_base_1::R`](R) reader structure"]
impl crate::Readable for SecBase1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_base_1::W`](W) writer structure"]
impl crate::Writable for SecBase1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_BASE_1 to value 0"]
impl crate::Resettable for SecBase1Spec {}
