#[doc = "Register `REG_RSVD_FOR_SW1` reader"]
pub type R = crate::R<RegRsvdForSw1Spec>;
#[doc = "Register `REG_RSVD_FOR_SW1` writer"]
pub type W = crate::W<RegRsvdForSw1Spec>;
#[doc = "Field `AON_RSVD_SW1` reader - "]
pub type AonRsvdSw1R = crate::FieldReader<u32>;
#[doc = "Field `AON_RSVD_SW1` writer - "]
pub type AonRsvdSw1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `WIFI_INIC_NP_READY` reader - Wi-Fi NP initialization ready flag"]
pub type WifiInicNpReadyR = crate::BitReader;
#[doc = "Field `WIFI_INIC_NP_READY` writer - Wi-Fi NP initialization ready flag"]
pub type WifiInicNpReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn aon_rsvd_sw1(&self) -> AonRsvdSw1R {
        AonRsvdSw1R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Wi-Fi NP initialization ready flag"]
    #[inline(always)]
    pub fn wifi_inic_np_ready(&self) -> WifiInicNpReadyR {
        WifiInicNpReadyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn aon_rsvd_sw1(&mut self) -> AonRsvdSw1W<'_, RegRsvdForSw1Spec> {
        AonRsvdSw1W::new(self, 0)
    }
    #[doc = "Bit 31 - Wi-Fi NP initialization ready flag"]
    #[inline(always)]
    pub fn wifi_inic_np_ready(&mut self) -> WifiInicNpReadyW<'_, RegRsvdForSw1Spec> {
        WifiInicNpReadyW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsvd_for_sw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsvd_for_sw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRsvdForSw1Spec;
impl crate::RegisterSpec for RegRsvdForSw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rsvd_for_sw1::R`](R) reader structure"]
impl crate::Readable for RegRsvdForSw1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rsvd_for_sw1::W`](W) writer structure"]
impl crate::Writable for RegRsvdForSw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RSVD_FOR_SW1 to value 0"]
impl crate::Resettable for RegRsvdForSw1Spec {}
