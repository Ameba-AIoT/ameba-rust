#[doc = "Register `SEC_END_0` reader"]
pub type R = crate::R<SecEnd0Spec>;
#[doc = "Register `SEC_END_0` writer"]
pub type W = crate::W<SecEnd0Spec>;
#[doc = "Field `SEC_R0_EN` reader - Set to enable SEC region 0 to scramble write data and descramble read data."]
pub type SecR0EnR = crate::BitReader;
#[doc = "Field `SEC_R0_EN` writer - Set to enable SEC region 0 to scramble write data and descramble read data."]
pub type SecR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_R0_MODE` reader - SEC region 0 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR0ModeR = crate::FieldReader;
#[doc = "Field `SEC_R0_MODE` writer - SEC region 0 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_END0` reader - SEC region 0 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd0R = crate::FieldReader<u32>;
#[doc = "Field `SEC_END0` writer - SEC region 0 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Set to enable SEC region 0 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r0_en(&self) -> SecR0EnR {
        SecR0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - SEC region 0 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r0_mode(&self) -> SecR0ModeR {
        SecR0ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:31 - SEC region 0 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end0(&self) -> SecEnd0R {
        SecEnd0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SEC region 0 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r0_en(&mut self) -> SecR0EnW<'_, SecEnd0Spec> {
        SecR0EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SEC region 0 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r0_mode(&mut self) -> SecR0ModeW<'_, SecEnd0Spec> {
        SecR0ModeW::new(self, 4)
    }
    #[doc = "Bits 12:31 - SEC region 0 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end0(&mut self) -> SecEnd0W<'_, SecEnd0Spec> {
        SecEnd0W::new(self, 12)
    }
}
#[doc = "SEC region 0 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_end_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_end_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnd0Spec;
impl crate::RegisterSpec for SecEnd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_end_0::R`](R) reader structure"]
impl crate::Readable for SecEnd0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_end_0::W`](W) writer structure"]
impl crate::Writable for SecEnd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_END_0 to value 0"]
impl crate::Resettable for SecEnd0Spec {}
