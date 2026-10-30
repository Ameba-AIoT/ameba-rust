#[doc = "Register `REG_BSSID1` reader"]
pub type R = crate::R<RegBssid1Spec>;
#[doc = "Register `REG_BSSID1` writer"]
pub type W = crate::W<RegBssid1Spec>;
#[doc = "Field `BIT_BSSID1_0` reader - Basic Station Set Identification for port 1."]
pub type BitBssid1_0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_BSSID1_0` writer - Basic Station Set Identification for port 1."]
pub type BitBssid1_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Basic Station Set Identification for port 1."]
    #[inline(always)]
    pub fn bit_bssid1_0(&self) -> BitBssid1_0R {
        BitBssid1_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Basic Station Set Identification for port 1."]
    #[inline(always)]
    pub fn bit_bssid1_0(&mut self) -> BitBssid1_0W<'_, RegBssid1Spec> {
        BitBssid1_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bssid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bssid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBssid1Spec;
impl crate::RegisterSpec for RegBssid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bssid1::R`](R) reader structure"]
impl crate::Readable for RegBssid1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bssid1::W`](W) writer structure"]
impl crate::Writable for RegBssid1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSID1 to value 0"]
impl crate::Resettable for RegBssid1Spec {}
