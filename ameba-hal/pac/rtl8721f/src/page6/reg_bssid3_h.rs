#[doc = "Register `REG_BSSID3_H` reader"]
pub type R = crate::R<RegBssid3HSpec>;
#[doc = "Register `REG_BSSID3_H` writer"]
pub type W = crate::W<RegBssid3HSpec>;
#[doc = "Field `BIT_BSSID3_H_V1` reader - Basic Station Set Identification 3"]
pub type BitBssid3HV1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BSSID3_H_V1` writer - Basic Station Set Identification 3"]
pub type BitBssid3HV1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Basic Station Set Identification 3"]
    #[inline(always)]
    pub fn bit_bssid3_h_v1(&self) -> BitBssid3HV1R {
        BitBssid3HV1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Basic Station Set Identification 3"]
    #[inline(always)]
    pub fn bit_bssid3_h_v1(&mut self) -> BitBssid3HV1W<'_, RegBssid3HSpec> {
        BitBssid3HV1W::new(self, 0)
    }
}
#[doc = "BSSID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bssid3_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bssid3_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBssid3HSpec;
impl crate::RegisterSpec for RegBssid3HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bssid3_h::R`](R) reader structure"]
impl crate::Readable for RegBssid3HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bssid3_h::W`](W) writer structure"]
impl crate::Writable for RegBssid3HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSID3_H to value 0"]
impl crate::Resettable for RegBssid3HSpec {}
