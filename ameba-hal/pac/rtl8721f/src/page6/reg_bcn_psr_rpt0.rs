#[doc = "Register `REG_BCN_PSR_RPT0` reader"]
pub type R = crate::R<RegBcnPsrRpt0Spec>;
#[doc = "Register `REG_BCN_PSR_RPT0` writer"]
pub type W = crate::W<RegBcnPsrRpt0Spec>;
#[doc = "Field `BIT_PS_AID_0` reader - AID0 for MACID"]
pub type BitPsAid0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_PS_AID_0` writer - AID0 for MACID"]
pub type BitPsAid0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `BIT_RPT_VALID0` reader - when set to 1, indicate the tim report is valid"]
pub type BitRptValid0R = crate::BitReader;
#[doc = "Field `BIT_HAS_MYAID0` reader - Indicate has my unicast packet"]
pub type BitHasMyaid0R = crate::BitReader;
#[doc = "Field `BIT_HAS_MY_BSSID0` reader - Indicate BC/MC packets are buffered in nontransmitted BSSID, SW need check crespponding dtim count field in multiple bssid index field"]
pub type BitHasMyBssid0R = crate::BitReader;
#[doc = "Field `BIT_HAS_MY_BC0` reader - Indicate BC/MC packets are buffered in transmitted BSSID or AP"]
pub type BitHasMyBc0R = crate::BitReader;
#[doc = "Field `BIT_DTIM_PERIOD0` reader - The DTIM period field of the TIM element of Rx beacon"]
pub type BitDtimPeriod0R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_CNT0` reader - The DTIM count field of the TIM element of Rx beacon"]
pub type BitDtimCnt0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - AID0 for MACID"]
    #[inline(always)]
    pub fn bit_ps_aid_0(&self) -> BitPsAid0R {
        BitPsAid0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - when set to 1, indicate the tim report is valid"]
    #[inline(always)]
    pub fn bit_rpt_valid0(&self) -> BitRptValid0R {
        BitRptValid0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicate has my unicast packet"]
    #[inline(always)]
    pub fn bit_has_myaid0(&self) -> BitHasMyaid0R {
        BitHasMyaid0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicate BC/MC packets are buffered in nontransmitted BSSID, SW need check crespponding dtim count field in multiple bssid index field"]
    #[inline(always)]
    pub fn bit_has_my_bssid0(&self) -> BitHasMyBssid0R {
        BitHasMyBssid0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicate BC/MC packets are buffered in transmitted BSSID or AP"]
    #[inline(always)]
    pub fn bit_has_my_bc0(&self) -> BitHasMyBc0R {
        BitHasMyBc0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - The DTIM period field of the TIM element of Rx beacon"]
    #[inline(always)]
    pub fn bit_dtim_period0(&self) -> BitDtimPeriod0R {
        BitDtimPeriod0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The DTIM count field of the TIM element of Rx beacon"]
    #[inline(always)]
    pub fn bit_dtim_cnt0(&self) -> BitDtimCnt0R {
        BitDtimCnt0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - AID0 for MACID"]
    #[inline(always)]
    pub fn bit_ps_aid_0(&mut self) -> BitPsAid0W<'_, RegBcnPsrRpt0Spec> {
        BitPsAid0W::new(self, 0)
    }
}
#[doc = "Beacon Parser Report Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_psr_rpt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_psr_rpt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnPsrRpt0Spec;
impl crate::RegisterSpec for RegBcnPsrRpt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_psr_rpt0::R`](R) reader structure"]
impl crate::Readable for RegBcnPsrRpt0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_psr_rpt0::W`](W) writer structure"]
impl crate::Writable for RegBcnPsrRpt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_PSR_RPT0 to value 0"]
impl crate::Resettable for RegBcnPsrRpt0Spec {}
