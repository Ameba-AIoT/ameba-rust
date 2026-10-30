#[doc = "Register `SEC_BASE_7` reader"]
pub type R = crate::R<SecBase7Spec>;
#[doc = "Register `SEC_BASE_7` writer"]
pub type W = crate::W<SecBase7Spec>;
#[doc = "Field `SEC_BASE7` reader - SEC region 7 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase7R = crate::FieldReader<u32>;
#[doc = "Field `SEC_BASE7` writer - SEC region 7 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase7W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - SEC region 7 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base7(&self) -> SecBase7R {
        SecBase7R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - SEC region 7 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base7(&mut self) -> SecBase7W<'_, SecBase7Spec> {
        SecBase7W::new(self, 12)
    }
}
#[doc = "SEC region 7 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_base_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_base_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecBase7Spec;
impl crate::RegisterSpec for SecBase7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_base_7::R`](R) reader structure"]
impl crate::Readable for SecBase7Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_base_7::W`](W) writer structure"]
impl crate::Writable for SecBase7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_BASE_7 to value 0"]
impl crate::Resettable for SecBase7Spec {}
