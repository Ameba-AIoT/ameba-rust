#[doc = "Register `REG_CHINFO_CONTENT` reader"]
pub type R = crate::R<RegChinfoContentSpec>;
#[doc = "Register `REG_CHINFO_CONTENT` writer"]
pub type W = crate::W<RegChinfoContentSpec>;
#[doc = "Field `r_chinfo_content` reader - the packet text vaule need hw to compare for channel info rx normal mode. When the content check function is enabled, channel info will be triggered only when the text match the cfg vaule."]
pub type RChinfoContentR = crate::FieldReader<u32>;
#[doc = "Field `r_chinfo_content` writer - the packet text vaule need hw to compare for channel info rx normal mode. When the content check function is enabled, channel info will be triggered only when the text match the cfg vaule."]
pub type RChinfoContentW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the packet text vaule need hw to compare for channel info rx normal mode. When the content check function is enabled, channel info will be triggered only when the text match the cfg vaule."]
    #[inline(always)]
    pub fn r_chinfo_content(&self) -> RChinfoContentR {
        RChinfoContentR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the packet text vaule need hw to compare for channel info rx normal mode. When the content check function is enabled, channel info will be triggered only when the text match the cfg vaule."]
    #[inline(always)]
    pub fn r_chinfo_content(&mut self) -> RChinfoContentW<'_, RegChinfoContentSpec> {
        RChinfoContentW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_chinfo_content::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_chinfo_content::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChinfoContentSpec;
impl crate::RegisterSpec for RegChinfoContentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_chinfo_content::R`](R) reader structure"]
impl crate::Readable for RegChinfoContentSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_chinfo_content::W`](W) writer structure"]
impl crate::Writable for RegChinfoContentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CHINFO_CONTENT to value 0"]
impl crate::Resettable for RegChinfoContentSpec {}
