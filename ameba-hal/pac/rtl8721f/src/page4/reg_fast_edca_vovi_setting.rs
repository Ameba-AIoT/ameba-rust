#[doc = "Register `REG_FAST_EDCA_VOVI_SETTING` reader"]
pub type R = crate::R<RegFastEdcaVoviSettingSpec>;
#[doc = "Register `REG_FAST_EDCA_VOVI_SETTING` writer"]
pub type W = crate::W<RegFastEdcaVoviSettingSpec>;
#[doc = "Field `BIT_VO_FAST_EDCA_PKT_TH` reader - vo queue fast edca pkt number threshold"]
pub type BitVoFastEdcaPktThR = crate::FieldReader;
#[doc = "Field `BIT_VO_FAST_EDCA_PKT_TH` writer - vo queue fast edca pkt number threshold"]
pub type BitVoFastEdcaPktThW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_VO_THRESHOLD_SEL` reader - 1: use all vo queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
pub type BitVoThresholdSelR = crate::BitReader;
#[doc = "Field `BIT_VO_THRESHOLD_SEL` writer - 1: use all vo queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
pub type BitVoThresholdSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VO_FAST_EDCA_TO` reader - vo queue fast edca timeout value"]
pub type BitVoFastEdcaToR = crate::FieldReader;
#[doc = "Field `BIT_VO_FAST_EDCA_TO` writer - vo queue fast edca timeout value"]
pub type BitVoFastEdcaToW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_VI_FAST_EDCA_PKT_TH` reader - vi queue fast edca pkt number threshold"]
pub type BitViFastEdcaPktThR = crate::FieldReader;
#[doc = "Field `BIT_VI_FAST_EDCA_PKT_TH` writer - vi queue fast edca pkt number threshold"]
pub type BitViFastEdcaPktThW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_VI_THRESHOLD_SEL` reader - 1: use all vi queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
pub type BitViThresholdSelR = crate::BitReader;
#[doc = "Field `BIT_VI_THRESHOLD_SEL` writer - 1: use all vi queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
pub type BitViThresholdSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VI_FAST_EDCA_TO` reader - vi queue fast edca timeout value"]
pub type BitViFastEdcaToR = crate::FieldReader;
#[doc = "Field `BIT_VI_FAST_EDCA_TO` writer - vi queue fast edca timeout value"]
pub type BitViFastEdcaToW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - vo queue fast edca pkt number threshold"]
    #[inline(always)]
    pub fn bit_vo_fast_edca_pkt_th(&self) -> BitVoFastEdcaPktThR {
        BitVoFastEdcaPktThR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 1: use all vo queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_vo_threshold_sel(&self) -> BitVoThresholdSelR {
        BitVoThresholdSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - vo queue fast edca timeout value"]
    #[inline(always)]
    pub fn bit_vo_fast_edca_to(&self) -> BitVoFastEdcaToR {
        BitVoFastEdcaToR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - vi queue fast edca pkt number threshold"]
    #[inline(always)]
    pub fn bit_vi_fast_edca_pkt_th(&self) -> BitViFastEdcaPktThR {
        BitViFastEdcaPktThR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 1: use all vi queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_vi_threshold_sel(&self) -> BitViThresholdSelR {
        BitViThresholdSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - vi queue fast edca timeout value"]
    #[inline(always)]
    pub fn bit_vi_fast_edca_to(&self) -> BitViFastEdcaToR {
        BitViFastEdcaToR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - vo queue fast edca pkt number threshold"]
    #[inline(always)]
    pub fn bit_vo_fast_edca_pkt_th(
        &mut self,
    ) -> BitVoFastEdcaPktThW<'_, RegFastEdcaVoviSettingSpec> {
        BitVoFastEdcaPktThW::new(self, 0)
    }
    #[doc = "Bit 7 - 1: use all vo queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_vo_threshold_sel(&mut self) -> BitVoThresholdSelW<'_, RegFastEdcaVoviSettingSpec> {
        BitVoThresholdSelW::new(self, 7)
    }
    #[doc = "Bits 8:15 - vo queue fast edca timeout value"]
    #[inline(always)]
    pub fn bit_vo_fast_edca_to(&mut self) -> BitVoFastEdcaToW<'_, RegFastEdcaVoviSettingSpec> {
        BitVoFastEdcaToW::new(self, 8)
    }
    #[doc = "Bits 16:22 - vi queue fast edca pkt number threshold"]
    #[inline(always)]
    pub fn bit_vi_fast_edca_pkt_th(
        &mut self,
    ) -> BitViFastEdcaPktThW<'_, RegFastEdcaVoviSettingSpec> {
        BitViFastEdcaPktThW::new(self, 16)
    }
    #[doc = "Bit 23 - 1: use all vi queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_vi_threshold_sel(&mut self) -> BitViThresholdSelW<'_, RegFastEdcaVoviSettingSpec> {
        BitViThresholdSelW::new(self, 23)
    }
    #[doc = "Bits 24:31 - vi queue fast edca timeout value"]
    #[inline(always)]
    pub fn bit_vi_fast_edca_to(&mut self) -> BitViFastEdcaToW<'_, RegFastEdcaVoviSettingSpec> {
        BitViFastEdcaToW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fast_edca_vovi_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fast_edca_vovi_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFastEdcaVoviSettingSpec;
impl crate::RegisterSpec for RegFastEdcaVoviSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fast_edca_vovi_setting::R`](R) reader structure"]
impl crate::Readable for RegFastEdcaVoviSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fast_edca_vovi_setting::W`](W) writer structure"]
impl crate::Writable for RegFastEdcaVoviSettingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FAST_EDCA_VOVI_SETTING to value 0x0804_0804"]
impl crate::Resettable for RegFastEdcaVoviSettingSpec {
    const RESET_VALUE: u32 = 0x0804_0804;
}
