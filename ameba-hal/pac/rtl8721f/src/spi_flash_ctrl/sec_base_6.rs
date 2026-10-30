#[doc = "Register `SEC_BASE_6` reader"]
pub type R = crate::R<SecBase6Spec>;
#[doc = "Register `SEC_BASE_6` writer"]
pub type W = crate::W<SecBase6Spec>;
#[doc = "Field `SEC_BASE6` reader - SEC region 6 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase6R = crate::FieldReader<u32>;
#[doc = "Field `SEC_BASE6` writer - SEC region 6 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase6W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - SEC region 6 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base6(&self) -> SecBase6R {
        SecBase6R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - SEC region 6 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base6(&mut self) -> SecBase6W<'_, SecBase6Spec> {
        SecBase6W::new(self, 12)
    }
}
#[doc = "SEC region 6 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_base_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_base_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecBase6Spec;
impl crate::RegisterSpec for SecBase6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_base_6::R`](R) reader structure"]
impl crate::Readable for SecBase6Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_base_6::W`](W) writer structure"]
impl crate::Writable for SecBase6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_BASE_6 to value 0"]
impl crate::Resettable for SecBase6Spec {}
