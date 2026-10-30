#[doc = "Register `REG_BSSID_H` reader"]
pub type R = crate::R<RegBssidHSpec>;
#[doc = "Register `REG_BSSID_H` writer"]
pub type W = crate::W<RegBssidHSpec>;
#[doc = "Field `BIT_BSSID_H` reader - Basic Station Set Identification BIT\\[47:32\\]"]
pub type BitBssidHR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BSSID_H` writer - Basic Station Set Identification BIT\\[47:32\\]"]
pub type BitBssidHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Basic Station Set Identification BIT\\[47:32\\]"]
    #[inline(always)]
    pub fn bit_bssid_h(&self) -> BitBssidHR {
        BitBssidHR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Basic Station Set Identification BIT\\[47:32\\]"]
    #[inline(always)]
    pub fn bit_bssid_h(&mut self) -> BitBssidHW<'_, RegBssidHSpec> {
        BitBssidHW::new(self, 0)
    }
}
#[doc = "BSSID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bssid_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bssid_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBssidHSpec;
impl crate::RegisterSpec for RegBssidHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bssid_h::R`](R) reader structure"]
impl crate::Readable for RegBssidHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bssid_h::W`](W) writer structure"]
impl crate::Writable for RegBssidHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSID_H to value 0"]
impl crate::Resettable for RegBssidHSpec {}
