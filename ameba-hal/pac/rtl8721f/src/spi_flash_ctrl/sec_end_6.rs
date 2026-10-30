#[doc = "Register `SEC_END_6` reader"]
pub type R = crate::R<SecEnd6Spec>;
#[doc = "Register `SEC_END_6` writer"]
pub type W = crate::W<SecEnd6Spec>;
#[doc = "Field `SEC_R6_EN` reader - Set to enable SEC region 6 to scramble write data and descramble read data."]
pub type SecR6EnR = crate::BitReader;
#[doc = "Field `SEC_R6_EN` writer - Set to enable SEC region 6 to scramble write data and descramble read data."]
pub type SecR6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_R6_MODE` reader - SEC region 6 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR6ModeR = crate::FieldReader;
#[doc = "Field `SEC_R6_MODE` writer - SEC region 6 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR6ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_END6` reader - SEC region 6 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd6R = crate::FieldReader<u32>;
#[doc = "Field `SEC_END6` writer - SEC region 6 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd6W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Set to enable SEC region 6 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r6_en(&self) -> SecR6EnR {
        SecR6EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - SEC region 6 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r6_mode(&self) -> SecR6ModeR {
        SecR6ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:31 - SEC region 6 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end6(&self) -> SecEnd6R {
        SecEnd6R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SEC region 6 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r6_en(&mut self) -> SecR6EnW<'_, SecEnd6Spec> {
        SecR6EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SEC region 6 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r6_mode(&mut self) -> SecR6ModeW<'_, SecEnd6Spec> {
        SecR6ModeW::new(self, 4)
    }
    #[doc = "Bits 12:31 - SEC region 6 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end6(&mut self) -> SecEnd6W<'_, SecEnd6Spec> {
        SecEnd6W::new(self, 12)
    }
}
#[doc = "SEC region 6 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_end_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_end_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnd6Spec;
impl crate::RegisterSpec for SecEnd6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_end_6::R`](R) reader structure"]
impl crate::Readable for SecEnd6Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_end_6::W`](W) writer structure"]
impl crate::Writable for SecEnd6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_END_6 to value 0"]
impl crate::Resettable for SecEnd6Spec {}
