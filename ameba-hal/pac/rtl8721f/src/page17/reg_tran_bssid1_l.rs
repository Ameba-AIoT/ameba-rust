#[doc = "Register `REG_TRAN_BSSID1_L` reader"]
pub type R = crate::R<RegTranBssid1LSpec>;
#[doc = "Register `REG_TRAN_BSSID1_L` writer"]
pub type W = crate::W<RegTranBssid1LSpec>;
#[doc = "Field `r_wmac_transbssid1_l` reader - lower 32 bits of transmitted bssid1"]
pub type RWmacTransbssid1LR = crate::FieldReader<u32>;
#[doc = "Field `r_wmac_transbssid1_l` writer - lower 32 bits of transmitted bssid1"]
pub type RWmacTransbssid1LW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - lower 32 bits of transmitted bssid1"]
    #[inline(always)]
    pub fn r_wmac_transbssid1_l(&self) -> RWmacTransbssid1LR {
        RWmacTransbssid1LR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - lower 32 bits of transmitted bssid1"]
    #[inline(always)]
    pub fn r_wmac_transbssid1_l(&mut self) -> RWmacTransbssid1LW<'_, RegTranBssid1LSpec> {
        RWmacTransbssid1LW::new(self, 0)
    }
}
#[doc = "trasmitted bssid register1_l\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tran_bssid1_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tran_bssid1_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTranBssid1LSpec;
impl crate::RegisterSpec for RegTranBssid1LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tran_bssid1_l::R`](R) reader structure"]
impl crate::Readable for RegTranBssid1LSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tran_bssid1_l::W`](W) writer structure"]
impl crate::Writable for RegTranBssid1LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRAN_BSSID1_L to value 0"]
impl crate::Resettable for RegTranBssid1LSpec {}
