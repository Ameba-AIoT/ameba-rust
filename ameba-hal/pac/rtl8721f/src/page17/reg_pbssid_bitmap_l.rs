#[doc = "Register `REG_PBSSID_BITMAP_L` reader"]
pub type R = crate::R<RegPbssidBitmapLSpec>;
#[doc = "Register `REG_PBSSID_BITMAP_L` writer"]
pub type W = crate::W<RegPbssidBitmapLSpec>;
#[doc = "Field `r_partial_bssid_bitmap_l` reader - low 32bits partial bssid bitmap"]
pub type RPartialBssidBitmapLR = crate::FieldReader<u32>;
#[doc = "Field `r_partial_bssid_bitmap_l` writer - low 32bits partial bssid bitmap"]
pub type RPartialBssidBitmapLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bits partial bssid bitmap"]
    #[inline(always)]
    pub fn r_partial_bssid_bitmap_l(&self) -> RPartialBssidBitmapLR {
        RPartialBssidBitmapLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32bits partial bssid bitmap"]
    #[inline(always)]
    pub fn r_partial_bssid_bitmap_l(&mut self) -> RPartialBssidBitmapLW<'_, RegPbssidBitmapLSpec> {
        RPartialBssidBitmapLW::new(self, 0)
    }
}
#[doc = "partial bssid bitmap register_l\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pbssid_bitmap_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pbssid_bitmap_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPbssidBitmapLSpec;
impl crate::RegisterSpec for RegPbssidBitmapLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pbssid_bitmap_l::R`](R) reader structure"]
impl crate::Readable for RegPbssidBitmapLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pbssid_bitmap_l::W`](W) writer structure"]
impl crate::Writable for RegPbssidBitmapLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PBSSID_BITMAP_L to value 0"]
impl crate::Resettable for RegPbssidBitmapLSpec {}
