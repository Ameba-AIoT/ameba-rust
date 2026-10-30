#[doc = "Register `REG_BSSID` reader"]
pub type R = crate::R<RegBssidSpec>;
#[doc = "Register `REG_BSSID` writer"]
pub type W = crate::W<RegBssidSpec>;
#[doc = "Field `BIT_BSSID_V1` reader - Basic Station Set Identification BIT\\[31:0\\]"]
pub type BitBssidV1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_BSSID_V1` writer - Basic Station Set Identification BIT\\[31:0\\]"]
pub type BitBssidV1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Basic Station Set Identification BIT\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_bssid_v1(&self) -> BitBssidV1R {
        BitBssidV1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Basic Station Set Identification BIT\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_bssid_v1(&mut self) -> BitBssidV1W<'_, RegBssidSpec> {
        BitBssidV1W::new(self, 0)
    }
}
#[doc = "BSSID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bssid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bssid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBssidSpec;
impl crate::RegisterSpec for RegBssidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bssid::R`](R) reader structure"]
impl crate::Readable for RegBssidSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bssid::W`](W) writer structure"]
impl crate::Writable for RegBssidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSID to value 0"]
impl crate::Resettable for RegBssidSpec {}
