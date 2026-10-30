#[doc = "Register `REG_PBSSID_BITMAP_H` reader"]
pub type R = crate::R<RegPbssidBitmapHSpec>;
#[doc = "Register `REG_PBSSID_BITMAP_H` writer"]
pub type W = crate::W<RegPbssidBitmapHSpec>;
#[doc = "Field `r_partial_bssid_bitmap_h` reader - high 32bits partial bssid bitmap"]
pub type RPartialBssidBitmapHR = crate::FieldReader<u32>;
#[doc = "Field `r_partial_bssid_bitmap_h` writer - high 32bits partial bssid bitmap"]
pub type RPartialBssidBitmapHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high 32bits partial bssid bitmap"]
    #[inline(always)]
    pub fn r_partial_bssid_bitmap_h(&self) -> RPartialBssidBitmapHR {
        RPartialBssidBitmapHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high 32bits partial bssid bitmap"]
    #[inline(always)]
    pub fn r_partial_bssid_bitmap_h(&mut self) -> RPartialBssidBitmapHW<'_, RegPbssidBitmapHSpec> {
        RPartialBssidBitmapHW::new(self, 0)
    }
}
#[doc = "partial bssid bitmap register_h\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pbssid_bitmap_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pbssid_bitmap_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPbssidBitmapHSpec;
impl crate::RegisterSpec for RegPbssidBitmapHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pbssid_bitmap_h::R`](R) reader structure"]
impl crate::Readable for RegPbssidBitmapHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pbssid_bitmap_h::W`](W) writer structure"]
impl crate::Writable for RegPbssidBitmapHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PBSSID_BITMAP_H to value 0"]
impl crate::Resettable for RegPbssidBitmapHSpec {}
