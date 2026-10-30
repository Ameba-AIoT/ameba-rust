#[doc = "Register `REG_BCN_PSR_RPT1` reader"]
pub type R = crate::R<RegBcnPsrRpt1Spec>;
#[doc = "Register `REG_BCN_PSR_RPT1` writer"]
pub type W = crate::W<RegBcnPsrRpt1Spec>;
#[doc = "Field `BIT_PS_AID_1` reader - AID1 for MACID1"]
pub type BitPsAid1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_PS_AID_1` writer - AID1 for MACID1"]
pub type BitPsAid1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Dummy` reader - "]
pub type DummyR = crate::BitReader;
#[doc = "Field `Dummy` writer - "]
pub type DummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TIM_VALID` reader - when set to 1 , indicate tim report valid"]
pub type BitTimValidR = crate::BitReader;
#[doc = "Field `BIT_TIM_HAS_MYAID` reader - my aid in the virtual bitmap is set"]
pub type BitTimHasMyaidR = crate::BitReader;
#[doc = "Field `BIT_TIM_HAS_MY_BSSID` reader - indicate BC/MC packets are buffered at nontransmitted BSSID, sw need to check corresponding dtim count field in multiple bssid index field"]
pub type BitTimHasMyBssidR = crate::BitReader;
#[doc = "Field `BIT_TIM_HAS_AID` reader - indicate BC/MC packets are buffered at the AP or transimited BSSID"]
pub type BitTimHasAidR = crate::BitReader;
#[doc = "Field `BIT_DTIM_PERIOD1` reader - DTIM Period"]
pub type BitDtimPeriod1R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_CNT1` reader - DTIM count"]
pub type BitDtimCnt1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - AID1 for MACID1"]
    #[inline(always)]
    pub fn bit_ps_aid_1(&self) -> BitPsAid1R {
        BitPsAid1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - when set to 1 , indicate tim report valid"]
    #[inline(always)]
    pub fn bit_tim_valid(&self) -> BitTimValidR {
        BitTimValidR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - my aid in the virtual bitmap is set"]
    #[inline(always)]
    pub fn bit_tim_has_myaid(&self) -> BitTimHasMyaidR {
        BitTimHasMyaidR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - indicate BC/MC packets are buffered at nontransmitted BSSID, sw need to check corresponding dtim count field in multiple bssid index field"]
    #[inline(always)]
    pub fn bit_tim_has_my_bssid(&self) -> BitTimHasMyBssidR {
        BitTimHasMyBssidR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - indicate BC/MC packets are buffered at the AP or transimited BSSID"]
    #[inline(always)]
    pub fn bit_tim_has_aid(&self) -> BitTimHasAidR {
        BitTimHasAidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DTIM Period"]
    #[inline(always)]
    pub fn bit_dtim_period1(&self) -> BitDtimPeriod1R {
        BitDtimPeriod1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTIM count"]
    #[inline(always)]
    pub fn bit_dtim_cnt1(&self) -> BitDtimCnt1R {
        BitDtimCnt1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - AID1 for MACID1"]
    #[inline(always)]
    pub fn bit_ps_aid_1(&mut self) -> BitPsAid1W<'_, RegBcnPsrRpt1Spec> {
        BitPsAid1W::new(self, 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegBcnPsrRpt1Spec> {
        DummyW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_psr_rpt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_psr_rpt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnPsrRpt1Spec;
impl crate::RegisterSpec for RegBcnPsrRpt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_psr_rpt1::R`](R) reader structure"]
impl crate::Readable for RegBcnPsrRpt1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_psr_rpt1::W`](W) writer structure"]
impl crate::Writable for RegBcnPsrRpt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_PSR_RPT1 to value 0"]
impl crate::Resettable for RegBcnPsrRpt1Spec {}
