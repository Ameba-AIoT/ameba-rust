#[doc = "Register `REG_BSSID3` reader"]
pub type R = crate::R<RegBssid3Spec>;
#[doc = "Register `REG_BSSID3` writer"]
pub type W = crate::W<RegBssid3Spec>;
#[doc = "Field `BIT_BSSID3_V1` reader - Basic Station Set Identification 3"]
pub type BitBssid3V1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_BSSID3_V1` writer - Basic Station Set Identification 3"]
pub type BitBssid3V1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Basic Station Set Identification 3"]
    #[inline(always)]
    pub fn bit_bssid3_v1(&self) -> BitBssid3V1R {
        BitBssid3V1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Basic Station Set Identification 3"]
    #[inline(always)]
    pub fn bit_bssid3_v1(&mut self) -> BitBssid3V1W<'_, RegBssid3Spec> {
        BitBssid3V1W::new(self, 0)
    }
}
#[doc = "BSSID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bssid3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bssid3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBssid3Spec;
impl crate::RegisterSpec for RegBssid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bssid3::R`](R) reader structure"]
impl crate::Readable for RegBssid3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bssid3::W`](W) writer structure"]
impl crate::Writable for RegBssid3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSID3 to value 0"]
impl crate::Resettable for RegBssid3Spec {}
