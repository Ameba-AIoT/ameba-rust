#[doc = "Register `REG_BCN_PSR_RPT4` reader"]
pub type R = crate::R<RegBcnPsrRpt4Spec>;
#[doc = "Register `REG_BCN_PSR_RPT4` writer"]
pub type W = crate::W<RegBcnPsrRpt4Spec>;
#[doc = "Field `BIT_PS_AID_4` reader - AID0 for MACID"]
pub type BitPsAid4R = crate::FieldReader<u16>;
#[doc = "Field `BIT_PS_AID_4` writer - AID0 for MACID"]
pub type BitPsAid4W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `BIT_RPT_VALID4` reader - when set to 1, indicate the tim report is valid"]
pub type BitRptValid4R = crate::BitReader;
#[doc = "Field `BIT_HAS_MYAID4` reader - Indicate has my unicast packet"]
pub type BitHasMyaid4R = crate::BitReader;
#[doc = "Field `BIT_HAS_MY_BSSID4` reader - Indicate BC/MC packets are buffered in nontransmitted BSSID, SW need check crespponding dtim count field in multiple bssid index field"]
pub type BitHasMyBssid4R = crate::BitReader;
#[doc = "Field `BIT_HAS_MY_BC4` reader - Indicate BC/MC packets are buffered in transmitted BSSID or AP"]
pub type BitHasMyBc4R = crate::BitReader;
#[doc = "Field `BIT_DTIM_PERIOD4` reader - The DTIM period field of the TIM element of Rx beacon"]
pub type BitDtimPeriod4R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_CNT4` reader - The DTIM count field of the TIM element of Rx beacon"]
pub type BitDtimCnt4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - AID0 for MACID"]
    #[inline(always)]
    pub fn bit_ps_aid_4(&self) -> BitPsAid4R {
        BitPsAid4R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - when set to 1, indicate the tim report is valid"]
    #[inline(always)]
    pub fn bit_rpt_valid4(&self) -> BitRptValid4R {
        BitRptValid4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicate has my unicast packet"]
    #[inline(always)]
    pub fn bit_has_myaid4(&self) -> BitHasMyaid4R {
        BitHasMyaid4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicate BC/MC packets are buffered in nontransmitted BSSID, SW need check crespponding dtim count field in multiple bssid index field"]
    #[inline(always)]
    pub fn bit_has_my_bssid4(&self) -> BitHasMyBssid4R {
        BitHasMyBssid4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicate BC/MC packets are buffered in transmitted BSSID or AP"]
    #[inline(always)]
    pub fn bit_has_my_bc4(&self) -> BitHasMyBc4R {
        BitHasMyBc4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - The DTIM period field of the TIM element of Rx beacon"]
    #[inline(always)]
    pub fn bit_dtim_period4(&self) -> BitDtimPeriod4R {
        BitDtimPeriod4R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The DTIM count field of the TIM element of Rx beacon"]
    #[inline(always)]
    pub fn bit_dtim_cnt4(&self) -> BitDtimCnt4R {
        BitDtimCnt4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - AID0 for MACID"]
    #[inline(always)]
    pub fn bit_ps_aid_4(&mut self) -> BitPsAid4W<'_, RegBcnPsrRpt4Spec> {
        BitPsAid4W::new(self, 0)
    }
}
#[doc = "Beacon Parser Report Register4\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_psr_rpt4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_psr_rpt4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnPsrRpt4Spec;
impl crate::RegisterSpec for RegBcnPsrRpt4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_psr_rpt4::R`](R) reader structure"]
impl crate::Readable for RegBcnPsrRpt4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_psr_rpt4::W`](W) writer structure"]
impl crate::Writable for RegBcnPsrRpt4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_PSR_RPT4 to value 0"]
impl crate::Resettable for RegBcnPsrRpt4Spec {}
