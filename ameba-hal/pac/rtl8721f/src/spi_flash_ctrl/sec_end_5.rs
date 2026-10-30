#[doc = "Register `SEC_END_5` reader"]
pub type R = crate::R<SecEnd5Spec>;
#[doc = "Register `SEC_END_5` writer"]
pub type W = crate::W<SecEnd5Spec>;
#[doc = "Field `SEC_R5_EN` reader - Set to enable SEC region 5 to scramble write data and descramble read data."]
pub type SecR5EnR = crate::BitReader;
#[doc = "Field `SEC_R5_EN` writer - Set to enable SEC region 5 to scramble write data and descramble read data."]
pub type SecR5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_R5_MODE` reader - SEC region 5 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR5ModeR = crate::FieldReader;
#[doc = "Field `SEC_R5_MODE` writer - SEC region 5 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR5ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_END5` reader - SEC region 5 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd5R = crate::FieldReader<u32>;
#[doc = "Field `SEC_END5` writer - SEC region 5 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd5W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Set to enable SEC region 5 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r5_en(&self) -> SecR5EnR {
        SecR5EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - SEC region 5 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r5_mode(&self) -> SecR5ModeR {
        SecR5ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:31 - SEC region 5 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end5(&self) -> SecEnd5R {
        SecEnd5R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SEC region 5 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r5_en(&mut self) -> SecR5EnW<'_, SecEnd5Spec> {
        SecR5EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SEC region 5 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r5_mode(&mut self) -> SecR5ModeW<'_, SecEnd5Spec> {
        SecR5ModeW::new(self, 4)
    }
    #[doc = "Bits 12:31 - SEC region 5 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end5(&mut self) -> SecEnd5W<'_, SecEnd5Spec> {
        SecEnd5W::new(self, 12)
    }
}
#[doc = "SEC region 5 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_end_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_end_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnd5Spec;
impl crate::RegisterSpec for SecEnd5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_end_5::R`](R) reader structure"]
impl crate::Readable for SecEnd5Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_end_5::W`](W) writer structure"]
impl crate::Writable for SecEnd5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_END_5 to value 0"]
impl crate::Resettable for SecEnd5Spec {}
