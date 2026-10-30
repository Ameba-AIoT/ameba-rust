#[doc = "Register `SEC_END_1` reader"]
pub type R = crate::R<SecEnd1Spec>;
#[doc = "Register `SEC_END_1` writer"]
pub type W = crate::W<SecEnd1Spec>;
#[doc = "Field `SEC_R1_EN` reader - Set to enable SEC region 1 to scramble write data and descramble read data."]
pub type SecR1EnR = crate::BitReader;
#[doc = "Field `SEC_R1_EN` writer - Set to enable SEC region 1 to scramble write data and descramble read data."]
pub type SecR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_R1_MODE` reader - SEC region 1 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR1ModeR = crate::FieldReader;
#[doc = "Field `SEC_R1_MODE` writer - SEC region 1 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
pub type SecR1ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_END1` reader - SEC region 1 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd1R = crate::FieldReader<u32>;
#[doc = "Field `SEC_END1` writer - SEC region 1 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecEnd1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - Set to enable SEC region 1 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r1_en(&self) -> SecR1EnR {
        SecR1EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - SEC region 1 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r1_mode(&self) -> SecR1ModeR {
        SecR1ModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:31 - SEC region 1 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end1(&self) -> SecEnd1R {
        SecEnd1R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SEC region 1 to scramble write data and descramble read data."]
    #[inline(always)]
    pub fn sec_r1_en(&mut self) -> SecR1EnW<'_, SecEnd1Spec> {
        SecR1EnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SEC region 1 - function mode/seed selection. Select one of mode/seed from SEC_NONCE0~SEC_NONCE7"]
    #[inline(always)]
    pub fn sec_r1_mode(&mut self) -> SecR1ModeW<'_, SecEnd1Spec> {
        SecR1ModeW::new(self, 4)
    }
    #[doc = "Bits 12:31 - SEC region 1 end address\\[31:12\\]. This is 4KB aligned address. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_end1(&mut self) -> SecEnd1W<'_, SecEnd1Spec> {
        SecEnd1W::new(self, 12)
    }
}
#[doc = "SEC region 1 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_end_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_end_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnd1Spec;
impl crate::RegisterSpec for SecEnd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_end_1::R`](R) reader structure"]
impl crate::Readable for SecEnd1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_end_1::W`](W) writer structure"]
impl crate::Writable for SecEnd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_END_1 to value 0"]
impl crate::Resettable for SecEnd1Spec {}
