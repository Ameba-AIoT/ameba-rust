#[doc = "Register `REG_TRAN_BSSID0_L` reader"]
pub type R = crate::R<RegTranBssid0LSpec>;
#[doc = "Register `REG_TRAN_BSSID0_L` writer"]
pub type W = crate::W<RegTranBssid0LSpec>;
#[doc = "Field `r_wmac_transbssid0_l` reader - lower 32 bits of transmitted bssid0"]
pub type RWmacTransbssid0LR = crate::FieldReader<u32>;
#[doc = "Field `r_wmac_transbssid0_l` writer - lower 32 bits of transmitted bssid0"]
pub type RWmacTransbssid0LW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - lower 32 bits of transmitted bssid0"]
    #[inline(always)]
    pub fn r_wmac_transbssid0_l(&self) -> RWmacTransbssid0LR {
        RWmacTransbssid0LR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - lower 32 bits of transmitted bssid0"]
    #[inline(always)]
    pub fn r_wmac_transbssid0_l(&mut self) -> RWmacTransbssid0LW<'_, RegTranBssid0LSpec> {
        RWmacTransbssid0LW::new(self, 0)
    }
}
#[doc = "trasmitted bssid register0_l\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tran_bssid0_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tran_bssid0_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTranBssid0LSpec;
impl crate::RegisterSpec for RegTranBssid0LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tran_bssid0_l::R`](R) reader structure"]
impl crate::Readable for RegTranBssid0LSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tran_bssid0_l::W`](W) writer structure"]
impl crate::Writable for RegTranBssid0LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRAN_BSSID0_L to value 0"]
impl crate::Resettable for RegTranBssid0LSpec {}
