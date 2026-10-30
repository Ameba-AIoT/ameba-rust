#[doc = "Register `REG_AESIV_SETTING` reader"]
pub type R = crate::R<RegAesivSettingSpec>;
#[doc = "Register `REG_AESIV_SETTING` writer"]
pub type W = crate::W<RegAesivSettingSpec>;
#[doc = "Field `BIT_AESIV_OFFSET` reader - aesiv offset in txrpt buffer (Unit:128Bytes)."]
pub type BitAesivOffsetR = crate::FieldReader<u16>;
#[doc = "Field `BIT_AESIV_OFFSET` writer - aesiv offset in txrpt buffer (Unit:128Bytes)."]
pub type BitAesivOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - aesiv offset in txrpt buffer (Unit:128Bytes)."]
    #[inline(always)]
    pub fn bit_aesiv_offset(&self) -> BitAesivOffsetR {
        BitAesivOffsetR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - aesiv offset in txrpt buffer (Unit:128Bytes)."]
    #[inline(always)]
    pub fn bit_aesiv_offset(&mut self) -> BitAesivOffsetW<'_, RegAesivSettingSpec> {
        BitAesivOffsetW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aesiv_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aesiv_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAesivSettingSpec;
impl crate::RegisterSpec for RegAesivSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aesiv_setting::R`](R) reader structure"]
impl crate::Readable for RegAesivSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_aesiv_setting::W`](W) writer structure"]
impl crate::Writable for RegAesivSettingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AESIV_SETTING to value 0x03"]
impl crate::Resettable for RegAesivSettingSpec {
    const RESET_VALUE: u32 = 0x03;
}
