#[doc = "Register `REG_BSSID2` reader"]
pub type R = crate::R<RegBssid2Spec>;
#[doc = "Register `REG_BSSID2` writer"]
pub type W = crate::W<RegBssid2Spec>;
#[doc = "Field `BIT_BSSID2_V1` reader - Basic Station Set Identification 2"]
pub type BitBssid2V1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_BSSID2_V1` writer - Basic Station Set Identification 2"]
pub type BitBssid2V1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Basic Station Set Identification 2"]
    #[inline(always)]
    pub fn bit_bssid2_v1(&self) -> BitBssid2V1R {
        BitBssid2V1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Basic Station Set Identification 2"]
    #[inline(always)]
    pub fn bit_bssid2_v1(&mut self) -> BitBssid2V1W<'_, RegBssid2Spec> {
        BitBssid2V1W::new(self, 0)
    }
}
#[doc = "BSSID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bssid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bssid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBssid2Spec;
impl crate::RegisterSpec for RegBssid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bssid2::R`](R) reader structure"]
impl crate::Readable for RegBssid2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bssid2::W`](W) writer structure"]
impl crate::Writable for RegBssid2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSID2 to value 0"]
impl crate::Resettable for RegBssid2Spec {}
