#[doc = "Register `SEC_END_2` reader"]
pub type R = crate::R<SecEnd2Spec>;
#[doc = "Register `SEC_END_2` writer"]
pub type W = crate::W<SecEnd2Spec>;
#[doc = "Field `SEC_R2_EN` reader - Set to enable SEC region 2 to scramble write data and descramble read data."]
pub type SecR2EnR = crate::BitReader;
#[doc = "Field `SEC_R2_EN` writer - Set to enable SEC region 2 to scramble write data and descramble read data."]
pub type SecR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_R2_MODE` reader - SEC region 2 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR2ModeR = crate::FieldReader;
#[doc = "Field `SEC_R2_MODE` writer - SEC region 2 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR2ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_END2` reader - SEC region 2 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd2R = crate::FieldReader<u32>;
#[doc = "Field `SEC_END2` writer - SEC region 2 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Set to enable SEC region 2 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r2_en(&self) -> SecR2EnR {
        SecR2EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - SEC region 2 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r2_mode(&self) -> SecR2ModeR {
        SecR2ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:31 - SEC region 2 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end2(&self) -> SecEnd2R {
        SecEnd2R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SEC region 2 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r2_en(&mut self) -> SecR2EnW<'_, SecEnd2Spec> {
        SecR2EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SEC region 2 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r2_mode(&mut self) -> SecR2ModeW<'_, SecEnd2Spec> {
        SecR2ModeW::new(self, 4)
    }
    #[doc = "Bits 12:31 - SEC region 2 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end2(&mut self) -> SecEnd2W<'_, SecEnd2Spec> {
        SecEnd2W::new(self, 12)
    }
}
#[doc = "SEC region 2 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_end_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_end_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnd2Spec;
impl crate::RegisterSpec for SecEnd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_end_2::R`](R) reader structure"]
impl crate::Readable for SecEnd2Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_end_2::W`](W) writer structure"]
impl crate::Writable for SecEnd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_END_2 to value 0"]
impl crate::Resettable for SecEnd2Spec {}
