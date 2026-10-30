#[doc = "Register `REG_FAST_EDCA_BEBK_SETTING` reader"]
pub type R = crate::R<RegFastEdcaBebkSettingSpec>;
#[doc = "Register `REG_FAST_EDCA_BEBK_SETTING` writer"]
pub type W = crate::W<RegFastEdcaBebkSettingSpec>;
#[doc = "Field `BIT_BE_FAST_EDCA_PKT_TH` reader - be queue fast edca pkt number threshold"]
pub type BitBeFastEdcaPktThR = crate::FieldReader;
#[doc = "Field `BIT_BE_FAST_EDCA_PKT_TH` writer - be queue fast edca pkt number threshold"]
pub type BitBeFastEdcaPktThW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_BE_THRESHOLD_SEL` reader - 1: use all be queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
pub type BitBeThresholdSelR = crate::BitReader;
#[doc = "Field `BIT_BE_THRESHOLD_SEL` writer - 1: use all be queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
pub type BitBeThresholdSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BE_FAST_EDCA_TO` reader - be queue fast edca timeout value"]
pub type BitBeFastEdcaToR = crate::FieldReader;
#[doc = "Field `BIT_BE_FAST_EDCA_TO` writer - be queue fast edca timeout value"]
pub type BitBeFastEdcaToW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BK_FAST_EDCA_PKT_TH` reader - bk queue fast edca pkt number threshold"]
pub type BitBkFastEdcaPktThR = crate::FieldReader;
#[doc = "Field `BIT_BK_FAST_EDCA_PKT_TH` writer - bk queue fast edca pkt number threshold"]
pub type BitBkFastEdcaPktThW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_BK_THRESHOLD_SEL` reader - 1: use all bk queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
pub type BitBkThresholdSelR = crate::BitReader;
#[doc = "Field `BIT_BK_THRESHOLD_SEL` writer - 1: use all bk queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
pub type BitBkThresholdSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BK_FAST_EDCA_TO` reader - bk queue fast edca timeout value"]
pub type BitBkFastEdcaToR = crate::FieldReader;
#[doc = "Field `BIT_BK_FAST_EDCA_TO` writer - bk queue fast edca timeout value"]
pub type BitBkFastEdcaToW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - be queue fast edca pkt number threshold"]
    #[inline(always)]
    pub fn bit_be_fast_edca_pkt_th(&self) -> BitBeFastEdcaPktThR {
        BitBeFastEdcaPktThR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 1: use all be queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_be_threshold_sel(&self) -> BitBeThresholdSelR {
        BitBeThresholdSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - be queue fast edca timeout value"]
    #[inline(always)]
    pub fn bit_be_fast_edca_to(&self) -> BitBeFastEdcaToR {
        BitBeFastEdcaToR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - bk queue fast edca pkt number threshold"]
    #[inline(always)]
    pub fn bit_bk_fast_edca_pkt_th(&self) -> BitBkFastEdcaPktThR {
        BitBkFastEdcaPktThR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - 1: use all bk queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_bk_threshold_sel(&self) -> BitBkThresholdSelR {
        BitBkThresholdSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - bk queue fast edca timeout value"]
    #[inline(always)]
    pub fn bit_bk_fast_edca_to(&self) -> BitBkFastEdcaToR {
        BitBkFastEdcaToR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - be queue fast edca pkt number threshold"]
    #[inline(always)]
    pub fn bit_be_fast_edca_pkt_th(
        &mut self,
    ) -> BitBeFastEdcaPktThW<'_, RegFastEdcaBebkSettingSpec> {
        BitBeFastEdcaPktThW::new(self, 0)
    }
    #[doc = "Bit 7 - 1: use all be queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_be_threshold_sel(&mut self) -> BitBeThresholdSelW<'_, RegFastEdcaBebkSettingSpec> {
        BitBeThresholdSelW::new(self, 7)
    }
    #[doc = "Bits 8:15 - be queue fast edca timeout value"]
    #[inline(always)]
    pub fn bit_be_fast_edca_to(&mut self) -> BitBeFastEdcaToW<'_, RegFastEdcaBebkSettingSpec> {
        BitBeFastEdcaToW::new(self, 8)
    }
    #[doc = "Bits 16:22 - bk queue fast edca pkt number threshold"]
    #[inline(always)]
    pub fn bit_bk_fast_edca_pkt_th(
        &mut self,
    ) -> BitBkFastEdcaPktThW<'_, RegFastEdcaBebkSettingSpec> {
        BitBkFastEdcaPktThW::new(self, 16)
    }
    #[doc = "Bit 23 - 1: use all bk queue pktnum compare to pktnum_threshold; 0: use single macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_bk_threshold_sel(&mut self) -> BitBkThresholdSelW<'_, RegFastEdcaBebkSettingSpec> {
        BitBkThresholdSelW::new(self, 23)
    }
    #[doc = "Bits 24:31 - bk queue fast edca timeout value"]
    #[inline(always)]
    pub fn bit_bk_fast_edca_to(&mut self) -> BitBkFastEdcaToW<'_, RegFastEdcaBebkSettingSpec> {
        BitBkFastEdcaToW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fast_edca_bebk_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fast_edca_bebk_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFastEdcaBebkSettingSpec;
impl crate::RegisterSpec for RegFastEdcaBebkSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fast_edca_bebk_setting::R`](R) reader structure"]
impl crate::Readable for RegFastEdcaBebkSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fast_edca_bebk_setting::W`](W) writer structure"]
impl crate::Writable for RegFastEdcaBebkSettingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FAST_EDCA_BEBK_SETTING to value 0x0804_0804"]
impl crate::Resettable for RegFastEdcaBebkSettingSpec {
    const RESET_VALUE: u32 = 0x0804_0804;
}
