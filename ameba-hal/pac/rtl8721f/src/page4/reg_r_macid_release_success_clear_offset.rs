#[doc = "Register `REG_r_macid_release_success_clear_offset` reader"]
pub type R = crate::R<RegRMacidReleaseSuccessClearOffsetSpec>;
#[doc = "Register `REG_r_macid_release_success_clear_offset` writer"]
pub type W = crate::W<RegRMacidReleaseSuccessClearOffsetSpec>;
#[doc = "Field `BIT_r_macid_release_success_clear_offset` reader - HW will set 0x2460~0x246C if one packet was released successfully for a given macid. SW use this register to reset the status."]
pub type BitRMacidReleaseSuccessClearOffsetR = crate::FieldReader;
#[doc = "Field `BIT_r_macid_release_success_clear_offset` writer - HW will set 0x2460~0x246C if one packet was released successfully for a given macid. SW use this register to reset the status."]
pub type BitRMacidReleaseSuccessClearOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - HW will set 0x2460~0x246C if one packet was released successfully for a given macid. SW use this register to reset the status."]
    #[inline(always)]
    pub fn bit_r_macid_release_success_clear_offset(&self) -> BitRMacidReleaseSuccessClearOffsetR {
        BitRMacidReleaseSuccessClearOffsetR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - HW will set 0x2460~0x246C if one packet was released successfully for a given macid. SW use this register to reset the status."]
    #[inline(always)]
    pub fn bit_r_macid_release_success_clear_offset(
        &mut self,
    ) -> BitRMacidReleaseSuccessClearOffsetW<'_, RegRMacidReleaseSuccessClearOffsetSpec> {
        BitRMacidReleaseSuccessClearOffsetW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_r_macid_release_success_clear_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_r_macid_release_success_clear_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRMacidReleaseSuccessClearOffsetSpec;
impl crate::RegisterSpec for RegRMacidReleaseSuccessClearOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_r_macid_release_success_clear_offset::R`](R) reader structure"]
impl crate::Readable for RegRMacidReleaseSuccessClearOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_r_macid_release_success_clear_offset::W`](W) writer structure"]
impl crate::Writable for RegRMacidReleaseSuccessClearOffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_r_macid_release_success_clear_offset to value 0"]
impl crate::Resettable for RegRMacidReleaseSuccessClearOffsetSpec {}
