#[doc = "Register `REG_BSSID1_1` reader"]
pub type R = crate::R<RegBssid1_1Spec>;
#[doc = "Register `REG_BSSID1_1` writer"]
pub type W = crate::W<RegBssid1_1Spec>;
#[doc = "Field `BIT_BSSID1_1` reader - Basic Station Set Identification for port 1."]
pub type BitBssid1_1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BSSID1_1` writer - Basic Station Set Identification for port 1."]
pub type BitBssid1_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Basic Station Set Identification for port 1."]
    #[inline(always)]
    pub fn bit_bssid1_1(&self) -> BitBssid1_1R {
        BitBssid1_1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Basic Station Set Identification for port 1."]
    #[inline(always)]
    pub fn bit_bssid1_1(&mut self) -> BitBssid1_1W<'_, RegBssid1_1Spec> {
        BitBssid1_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bssid1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bssid1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBssid1_1Spec;
impl crate::RegisterSpec for RegBssid1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bssid1_1::R`](R) reader structure"]
impl crate::Readable for RegBssid1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bssid1_1::W`](W) writer structure"]
impl crate::Writable for RegBssid1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSID1_1 to value 0"]
impl crate::Resettable for RegBssid1_1Spec {}
