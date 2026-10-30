#[doc = "Register `REG_BSSID2_H` reader"]
pub type R = crate::R<RegBssid2HSpec>;
#[doc = "Register `REG_BSSID2_H` writer"]
pub type W = crate::W<RegBssid2HSpec>;
#[doc = "Field `BIT_BSSID2_H_V1` reader - Basic Station Set Identification 2"]
pub type BitBssid2HV1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BSSID2_H_V1` writer - Basic Station Set Identification 2"]
pub type BitBssid2HV1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Basic Station Set Identification 2"]
    #[inline(always)]
    pub fn bit_bssid2_h_v1(&self) -> BitBssid2HV1R {
        BitBssid2HV1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Basic Station Set Identification 2"]
    #[inline(always)]
    pub fn bit_bssid2_h_v1(&mut self) -> BitBssid2HV1W<'_, RegBssid2HSpec> {
        BitBssid2HV1W::new(self, 0)
    }
}
#[doc = "BSSID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bssid2_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bssid2_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBssid2HSpec;
impl crate::RegisterSpec for RegBssid2HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bssid2_h::R`](R) reader structure"]
impl crate::Readable for RegBssid2HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bssid2_h::W`](W) writer structure"]
impl crate::Writable for RegBssid2HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSID2_H to value 0"]
impl crate::Resettable for RegBssid2HSpec {}
