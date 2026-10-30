#[doc = "Register `SEC_BASE_2` reader"]
pub type R = crate::R<SecBase2Spec>;
#[doc = "Register `SEC_BASE_2` writer"]
pub type W = crate::W<SecBase2Spec>;
#[doc = "Field `SEC_BASE2` reader - SEC region 2 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase2R = crate::FieldReader<u32>;
#[doc = "Field `SEC_BASE2` writer - SEC region 2 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecBase2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - SEC region 2 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base2(&self) -> SecBase2R {
        SecBase2R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - SEC region 2 base address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_base2(&mut self) -> SecBase2W<'_, SecBase2Spec> {
        SecBase2W::new(self, 12)
    }
}
#[doc = "SEC region 2 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_base_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_base_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecBase2Spec;
impl crate::RegisterSpec for SecBase2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_base_2::R`](R) reader structure"]
impl crate::Readable for SecBase2Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_base_2::W`](W) writer structure"]
impl crate::Writable for SecBase2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_BASE_2 to value 0"]
impl crate::Resettable for SecBase2Spec {}
