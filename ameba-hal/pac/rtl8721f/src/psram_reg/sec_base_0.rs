#[doc = "Register `SEC_BASE_0` reader"]
pub type R = crate::R<SecBase0Spec>;
#[doc = "Register `SEC_BASE_0` writer"]
pub type W = crate::W<SecBase0Spec>;
#[doc = "Field `SEC_BASE0` reader - SEC region 0 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase0R = crate::FieldReader<u32>;
#[doc = "Field `SEC_BASE0` writer - SEC region 0 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - SEC region 0 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base0(&self) -> SecBase0R {
        SecBase0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - SEC region 0 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base0(&mut self) -> SecBase0W<'_, SecBase0Spec> {
        SecBase0W::new(self, 12)
    }
}
#[doc = "SEC region 0 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_base_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_base_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecBase0Spec;
impl crate::RegisterSpec for SecBase0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_base_0::R`](R) reader structure"]
impl crate::Readable for SecBase0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_base_0::W`](W) writer structure"]
impl crate::Writable for SecBase0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_BASE_0 to value 0"]
impl crate::Resettable for SecBase0Spec {}
