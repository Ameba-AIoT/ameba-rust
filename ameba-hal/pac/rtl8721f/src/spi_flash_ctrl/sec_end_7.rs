#[doc = "Register `SEC_END_7` reader"]
pub type R = crate::R<SecEnd7Spec>;
#[doc = "Register `SEC_END_7` writer"]
pub type W = crate::W<SecEnd7Spec>;
#[doc = "Field `SEC_R7_EN` reader - Set to enable SEC region 7 to scramble write data and descramble read data."]
pub type SecR7EnR = crate::BitReader;
#[doc = "Field `SEC_R7_EN` writer - Set to enable SEC region 7 to scramble write data and descramble read data."]
pub type SecR7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_R7_MODE` reader - SEC region 7 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR7ModeR = crate::FieldReader;
#[doc = "Field `SEC_R7_MODE` writer - SEC region 7 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR7ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_END7` reader - SEC region 7 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd7R = crate::FieldReader<u32>;
#[doc = "Field `SEC_END7` writer - SEC region 7 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd7W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Set to enable SEC region 7 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r7_en(&self) -> SecR7EnR {
        SecR7EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - SEC region 7 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r7_mode(&self) -> SecR7ModeR {
        SecR7ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:31 - SEC region 7 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end7(&self) -> SecEnd7R {
        SecEnd7R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SEC region 7 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r7_en(&mut self) -> SecR7EnW<'_, SecEnd7Spec> {
        SecR7EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SEC region 7 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r7_mode(&mut self) -> SecR7ModeW<'_, SecEnd7Spec> {
        SecR7ModeW::new(self, 4)
    }
    #[doc = "Bits 12:31 - SEC region 7 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end7(&mut self) -> SecEnd7W<'_, SecEnd7Spec> {
        SecEnd7W::new(self, 12)
    }
}
#[doc = "SEC region 7 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_end_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_end_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnd7Spec;
impl crate::RegisterSpec for SecEnd7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_end_7::R`](R) reader structure"]
impl crate::Readable for SecEnd7Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_end_7::W`](W) writer structure"]
impl crate::Writable for SecEnd7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_END_7 to value 0"]
impl crate::Resettable for SecEnd7Spec {}
