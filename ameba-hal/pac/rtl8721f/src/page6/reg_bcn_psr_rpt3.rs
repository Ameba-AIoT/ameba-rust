#[doc = "Register `REG_BCN_PSR_RPT3` reader"]
pub type R = crate::R<RegBcnPsrRpt3Spec>;
#[doc = "Register `REG_BCN_PSR_RPT3` writer"]
pub type W = crate::W<RegBcnPsrRpt3Spec>;
#[doc = "Field `BIT_PS_AID_3` reader - AID0 for MACID"]
pub type BitPsAid3R = crate::FieldReader<u16>;
#[doc = "Field `BIT_PS_AID_3` writer - AID0 for MACID"]
pub type BitPsAid3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `BIT_RPT_VALID3` reader - when set to 1, indicate the tim report is valid"]
pub type BitRptValid3R = crate::BitReader;
#[doc = "Field `BIT_HAS_MYAID3` reader - Indicate has my unicast packet"]
pub type BitHasMyaid3R = crate::BitReader;
#[doc = "Field `BIT_HAS_MY_BSSID3` reader - Indicate BC/MC packets are buffered in nontransmitted BSSID, SW need check crespponding dtim count field in multiple bssid index field"]
pub type BitHasMyBssid3R = crate::BitReader;
#[doc = "Field `BIT_HAS_MY_BC3` reader - Indicate BC/MC packets are buffered in transmitted BSSID or AP"]
pub type BitHasMyBc3R = crate::BitReader;
#[doc = "Field `BIT_DTIM_PERIOD3` reader - The DTIM period field of the TIM element of Rx beacon"]
pub type BitDtimPeriod3R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_CNT3` reader - The DTIM count field of the TIM element of Rx beacon"]
pub type BitDtimCnt3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - AID0 for MACID"]
    #[inline(always)]
    pub fn bit_ps_aid_3(&self) -> BitPsAid3R {
        BitPsAid3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - when set to 1, indicate the tim report is valid"]
    #[inline(always)]
    pub fn bit_rpt_valid3(&self) -> BitRptValid3R {
        BitRptValid3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicate has my unicast packet"]
    #[inline(always)]
    pub fn bit_has_myaid3(&self) -> BitHasMyaid3R {
        BitHasMyaid3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicate BC/MC packets are buffered in nontransmitted BSSID, SW need check crespponding dtim count field in multiple bssid index field"]
    #[inline(always)]
    pub fn bit_has_my_bssid3(&self) -> BitHasMyBssid3R {
        BitHasMyBssid3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicate BC/MC packets are buffered in transmitted BSSID or AP"]
    #[inline(always)]
    pub fn bit_has_my_bc3(&self) -> BitHasMyBc3R {
        BitHasMyBc3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - The DTIM period field of the TIM element of Rx beacon"]
    #[inline(always)]
    pub fn bit_dtim_period3(&self) -> BitDtimPeriod3R {
        BitDtimPeriod3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The DTIM count field of the TIM element of Rx beacon"]
    #[inline(always)]
    pub fn bit_dtim_cnt3(&self) -> BitDtimCnt3R {
        BitDtimCnt3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - AID0 for MACID"]
    #[inline(always)]
    pub fn bit_ps_aid_3(&mut self) -> BitPsAid3W<'_, RegBcnPsrRpt3Spec> {
        BitPsAid3W::new(self, 0)
    }
}
#[doc = "Beacon Parser Report Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_psr_rpt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_psr_rpt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnPsrRpt3Spec;
impl crate::RegisterSpec for RegBcnPsrRpt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_psr_rpt3::R`](R) reader structure"]
impl crate::Readable for RegBcnPsrRpt3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_psr_rpt3::W`](W) writer structure"]
impl crate::Writable for RegBcnPsrRpt3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_PSR_RPT3 to value 0"]
impl crate::Resettable for RegBcnPsrRpt3Spec {}
