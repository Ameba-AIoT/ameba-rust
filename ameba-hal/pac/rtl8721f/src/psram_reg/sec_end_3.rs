#[doc = "Register `SEC_END_3` reader"]
pub type R = crate::R<SecEnd3Spec>;
#[doc = "Register `SEC_END_3` writer"]
pub type W = crate::W<SecEnd3Spec>;
#[doc = "Field `SEC_R3_EN` reader - Set to enable SEC region 3 to scramble write data and descramble read data."]
pub type SecR3EnR = crate::BitReader;
#[doc = "Field `SEC_R3_EN` writer - Set to enable SEC region 3 to scramble write data and descramble read data."]
pub type SecR3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_R3_MODE` reader - SEC region 3 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR3ModeR = crate::FieldReader;
#[doc = "Field `SEC_R3_MODE` writer - SEC region 3 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR3ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_END3` reader - SEC region 3 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd3R = crate::FieldReader<u32>;
#[doc = "Field `SEC_END3` writer - SEC region 3 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd3W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Set to enable SEC region 3 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r3_en(&self) -> SecR3EnR {
        SecR3EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - SEC region 3 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r3_mode(&self) -> SecR3ModeR {
        SecR3ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:31 - SEC region 3 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end3(&self) -> SecEnd3R {
        SecEnd3R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SEC region 3 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r3_en(&mut self) -> SecR3EnW<'_, SecEnd3Spec> {
        SecR3EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SEC region 3 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r3_mode(&mut self) -> SecR3ModeW<'_, SecEnd3Spec> {
        SecR3ModeW::new(self, 4)
    }
    #[doc = "Bits 12:31 - SEC region 3 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end3(&mut self) -> SecEnd3W<'_, SecEnd3Spec> {
        SecEnd3W::new(self, 12)
    }
}
#[doc = "SEC region 3 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_end_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_end_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnd3Spec;
impl crate::RegisterSpec for SecEnd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_end_3::R`](R) reader structure"]
impl crate::Readable for SecEnd3Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_end_3::W`](W) writer structure"]
impl crate::Writable for SecEnd3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_END_3 to value 0"]
impl crate::Resettable for SecEnd3Spec {}
