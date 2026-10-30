#[doc = "Register `REG_BCN_PSR_RPT2` reader"]
pub type R = crate::R<RegBcnPsrRpt2Spec>;
#[doc = "Register `REG_BCN_PSR_RPT2` writer"]
pub type W = crate::W<RegBcnPsrRpt2Spec>;
#[doc = "Field `BIT_PS_AID_2` reader - AID0 for MACID"]
pub type BitPsAid2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_PS_AID_2` writer - AID0 for MACID"]
pub type BitPsAid2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `BIT_RPT_VALID2` reader - when set to 1, indicate the tim report is valid"]
pub type BitRptValid2R = crate::BitReader;
#[doc = "Field `BIT_HAS_MYAID2` reader - Indicate has my unicast packet"]
pub type BitHasMyaid2R = crate::BitReader;
#[doc = "Field `BIT_HAS_MY_BSSID2` reader - Indicate BC/MC packets are buffered in nontransmitted BSSID, SW need check crespponding dtim count field in multiple bssid index field"]
pub type BitHasMyBssid2R = crate::BitReader;
#[doc = "Field `BIT_HAS_MY_BC2` reader - Indicate BC/MC packets are buffered in transmitted BSSID or AP"]
pub type BitHasMyBc2R = crate::BitReader;
#[doc = "Field `BIT_DTIM_PERIOD2` reader - The DTIM period field of the TIM element of Rx beacon"]
pub type BitDtimPeriod2R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_CNT2` reader - The DTIM count field of the TIM element of Rx beacon"]
pub type BitDtimCnt2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - AID0 for MACID"]
    #[inline(always)]
    pub fn bit_ps_aid_2(&self) -> BitPsAid2R {
        BitPsAid2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - when set to 1, indicate the tim report is valid"]
    #[inline(always)]
    pub fn bit_rpt_valid2(&self) -> BitRptValid2R {
        BitRptValid2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicate has my unicast packet"]
    #[inline(always)]
    pub fn bit_has_myaid2(&self) -> BitHasMyaid2R {
        BitHasMyaid2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicate BC/MC packets are buffered in nontransmitted BSSID, SW need check crespponding dtim count field in multiple bssid index field"]
    #[inline(always)]
    pub fn bit_has_my_bssid2(&self) -> BitHasMyBssid2R {
        BitHasMyBssid2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicate BC/MC packets are buffered in transmitted BSSID or AP"]
    #[inline(always)]
    pub fn bit_has_my_bc2(&self) -> BitHasMyBc2R {
        BitHasMyBc2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - The DTIM period field of the TIM element of Rx beacon"]
    #[inline(always)]
    pub fn bit_dtim_period2(&self) -> BitDtimPeriod2R {
        BitDtimPeriod2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The DTIM count field of the TIM element of Rx beacon"]
    #[inline(always)]
    pub fn bit_dtim_cnt2(&self) -> BitDtimCnt2R {
        BitDtimCnt2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - AID0 for MACID"]
    #[inline(always)]
    pub fn bit_ps_aid_2(&mut self) -> BitPsAid2W<'_, RegBcnPsrRpt2Spec> {
        BitPsAid2W::new(self, 0)
    }
}
#[doc = "Beacon Parser Report Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_psr_rpt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_psr_rpt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnPsrRpt2Spec;
impl crate::RegisterSpec for RegBcnPsrRpt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_psr_rpt2::R`](R) reader structure"]
impl crate::Readable for RegBcnPsrRpt2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_psr_rpt2::W`](W) writer structure"]
impl crate::Writable for RegBcnPsrRpt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_PSR_RPT2 to value 0"]
impl crate::Resettable for RegBcnPsrRpt2Spec {}
