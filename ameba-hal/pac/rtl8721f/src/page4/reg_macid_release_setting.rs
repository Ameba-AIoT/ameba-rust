#[doc = "Register `REG_MACID_RELEASE_SETTING` reader"]
pub type R = crate::R<RegMacidReleaseSettingSpec>;
#[doc = "Register `REG_MACID_RELEASE_SETTING` writer"]
pub type W = crate::W<RegMacidReleaseSettingSpec>;
#[doc = "Field `BIT_macid_offset` reader - write this field to set REG_macid_release\\[BIT_macid_offset\\] = BIT_macid_value"]
pub type BitMacidOffsetR = crate::FieldReader;
#[doc = "Field `BIT_macid_offset` writer - write this field to set REG_macid_release\\[BIT_macid_offset\\] = BIT_macid_value"]
pub type BitMacidOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_macid_value` reader - write this field to set REG_macid_release\\[BIT_macid_offset\\] = BIT_macid_value"]
pub type BitMacidValueR = crate::BitReader;
#[doc = "Field `BIT_macid_value` writer - write this field to set REG_macid_release\\[BIT_macid_offset\\] = BIT_macid_value"]
pub type BitMacidValueW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - write this field to set REG_macid_release\\[BIT_macid_offset\\] = BIT_macid_value"]
    #[inline(always)]
    pub fn bit_macid_offset(&self) -> BitMacidOffsetR {
        BitMacidOffsetR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - write this field to set REG_macid_release\\[BIT_macid_offset\\] = BIT_macid_value"]
    #[inline(always)]
    pub fn bit_macid_value(&self) -> BitMacidValueR {
        BitMacidValueR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - write this field to set REG_macid_release\\[BIT_macid_offset\\] = BIT_macid_value"]
    #[inline(always)]
    pub fn bit_macid_offset(&mut self) -> BitMacidOffsetW<'_, RegMacidReleaseSettingSpec> {
        BitMacidOffsetW::new(self, 0)
    }
    #[doc = "Bit 7 - write this field to set REG_macid_release\\[BIT_macid_offset\\] = BIT_macid_value"]
    #[inline(always)]
    pub fn bit_macid_value(&mut self) -> BitMacidValueW<'_, RegMacidReleaseSettingSpec> {
        BitMacidValueW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_release_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_release_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidReleaseSettingSpec;
impl crate::RegisterSpec for RegMacidReleaseSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_release_setting::R`](R) reader structure"]
impl crate::Readable for RegMacidReleaseSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_release_setting::W`](W) writer structure"]
impl crate::Writable for RegMacidReleaseSettingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_RELEASE_SETTING to value 0"]
impl crate::Resettable for RegMacidReleaseSettingSpec {}
