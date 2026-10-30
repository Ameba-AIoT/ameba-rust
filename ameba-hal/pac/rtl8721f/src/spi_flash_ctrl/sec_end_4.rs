#[doc = "Register `SEC_END_4` reader"]
pub type R = crate::R<SecEnd4Spec>;
#[doc = "Register `SEC_END_4` writer"]
pub type W = crate::W<SecEnd4Spec>;
#[doc = "Field `SEC_R4_EN` reader - Set to enable SEC region 4 to scramble write data and descramble read data."]
pub type SecR4EnR = crate::BitReader;
#[doc = "Field `SEC_R4_EN` writer - Set to enable SEC region 4 to scramble write data and descramble read data."]
pub type SecR4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_R4_MODE` reader - SEC region 4 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR4ModeR = crate::FieldReader;
#[doc = "Field `SEC_R4_MODE` writer - SEC region 4 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR4ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_END4` reader - SEC region 4 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd4R = crate::FieldReader<u32>;
#[doc = "Field `SEC_END4` writer - SEC region 4 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd4W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Set to enable SEC region 4 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r4_en(&self) -> SecR4EnR {
        SecR4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - SEC region 4 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r4_mode(&self) -> SecR4ModeR {
        SecR4ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:31 - SEC region 4 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end4(&self) -> SecEnd4R {
        SecEnd4R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SEC region 4 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r4_en(&mut self) -> SecR4EnW<'_, SecEnd4Spec> {
        SecR4EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SEC region 4 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r4_mode(&mut self) -> SecR4ModeW<'_, SecEnd4Spec> {
        SecR4ModeW::new(self, 4)
    }
    #[doc = "Bits 12:31 - SEC region 4 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end4(&mut self) -> SecEnd4W<'_, SecEnd4Spec> {
        SecEnd4W::new(self, 12)
    }
}
#[doc = "SEC region 4 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_end_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_end_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnd4Spec;
impl crate::RegisterSpec for SecEnd4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_end_4::R`](R) reader structure"]
impl crate::Readable for SecEnd4Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_end_4::W`](W) writer structure"]
impl crate::Writable for SecEnd4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_END_4 to value 0"]
impl crate::Resettable for SecEnd4Spec {}
