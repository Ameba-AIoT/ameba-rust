#[doc = "Register `SEC_BASE_4` reader"]
pub type R = crate::R<SecBase4Spec>;
#[doc = "Register `SEC_BASE_4` writer"]
pub type W = crate::W<SecBase4Spec>;
#[doc = "Field `SEC_BASE4` reader - SEC region 4 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase4R = crate::FieldReader<u32>;
#[doc = "Field `SEC_BASE4` writer - SEC region 4 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase4W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - SEC region 4 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base4(&self) -> SecBase4R {
        SecBase4R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - SEC region 4 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base4(&mut self) -> SecBase4W<'_, SecBase4Spec> {
        SecBase4W::new(self, 12)
    }
}
#[doc = "SEC region 4 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_base_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_base_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecBase4Spec;
impl crate::RegisterSpec for SecBase4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_base_4::R`](R) reader structure"]
impl crate::Readable for SecBase4Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_base_4::W`](W) writer structure"]
impl crate::Writable for SecBase4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_BASE_4 to value 0"]
impl crate::Resettable for SecBase4Spec {}
