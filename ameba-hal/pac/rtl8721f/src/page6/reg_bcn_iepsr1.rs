#[doc = "Register `REG_BCN_IEPSR1` reader"]
pub type R = crate::R<RegBcnIepsr1Spec>;
#[doc = "Register `REG_BCN_IEPSR1` writer"]
pub type W = crate::W<RegBcnIepsr1Spec>;
#[doc = "Field `r_WMAC_BCN_MBSSID_IDX` reader - the multiple bssid index that current sta connect with"]
pub type RWmacBcnMbssidIdxR = crate::FieldReader;
#[doc = "Field `r_WMAC_BCN_MBSSID_IDX` writer - the multiple bssid index that current sta connect with"]
pub type RWmacBcnMbssidIdxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_WMAC_BCN_BMC_MASK` reader - bcn tim hit bc/mc mask"]
pub type RWmacBcnBmcMaskR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_BMC_MASK` writer - bcn tim hit bc/mc mask"]
pub type RWmacBcnBmcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_TIM_TIMPSR_EN` reader - enable parser tim frame tim element"]
pub type RWmacTimTimpsrEnR = crate::BitReader;
#[doc = "Field `r_WMAC_TIM_TIMPSR_EN` writer - enable parser tim frame tim element"]
pub type RWmacTimTimpsrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_OPS_PSR_EN` reader - enable parser ops frame tim elemnt and ops element"]
pub type RWmacOpsPsrEnR = crate::BitReader;
#[doc = "Field `r_WMAC_OPS_PSR_EN` writer - enable parser ops frame tim elemnt and ops element"]
pub type RWmacOpsPsrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_BCN_MBSSID_VAL` reader - indicate ofs and n0 change bit is valid."]
pub type RWmacBcnMbssidValR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_N0_CHG` reader - indicate parsed maxmbssid indicator field value has changed which leads to the change of N0."]
pub type RWmacBcnN0ChgR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_MBSSID_OFS` reader - the multiple bssid element offset (byte unit) in frame body"]
pub type RWmacBcnMbssidOfsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - the multiple bssid index that current sta connect with"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_idx(&self) -> RWmacBcnMbssidIdxR {
        RWmacBcnMbssidIdxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - bcn tim hit bc/mc mask"]
    #[inline(always)]
    pub fn r_wmac_bcn_bmc_mask(&self) -> RWmacBcnBmcMaskR {
        RWmacBcnBmcMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - enable parser tim frame tim element"]
    #[inline(always)]
    pub fn r_wmac_tim_timpsr_en(&self) -> RWmacTimTimpsrEnR {
        RWmacTimTimpsrEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable parser ops frame tim elemnt and ops element"]
    #[inline(always)]
    pub fn r_wmac_ops_psr_en(&self) -> RWmacOpsPsrEnR {
        RWmacOpsPsrEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - indicate ofs and n0 change bit is valid."]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_val(&self) -> RWmacBcnMbssidValR {
        RWmacBcnMbssidValR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - indicate parsed maxmbssid indicator field value has changed which leads to the change of N0."]
    #[inline(always)]
    pub fn r_wmac_bcn_n0_chg(&self) -> RWmacBcnN0ChgR {
        RWmacBcnN0ChgR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - the multiple bssid element offset (byte unit) in frame body"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_ofs(&self) -> RWmacBcnMbssidOfsR {
        RWmacBcnMbssidOfsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - the multiple bssid index that current sta connect with"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_idx(&mut self) -> RWmacBcnMbssidIdxW<'_, RegBcnIepsr1Spec> {
        RWmacBcnMbssidIdxW::new(self, 0)
    }
    #[doc = "Bit 8 - bcn tim hit bc/mc mask"]
    #[inline(always)]
    pub fn r_wmac_bcn_bmc_mask(&mut self) -> RWmacBcnBmcMaskW<'_, RegBcnIepsr1Spec> {
        RWmacBcnBmcMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - enable parser tim frame tim element"]
    #[inline(always)]
    pub fn r_wmac_tim_timpsr_en(&mut self) -> RWmacTimTimpsrEnW<'_, RegBcnIepsr1Spec> {
        RWmacTimTimpsrEnW::new(self, 9)
    }
    #[doc = "Bit 10 - enable parser ops frame tim elemnt and ops element"]
    #[inline(always)]
    pub fn r_wmac_ops_psr_en(&mut self) -> RWmacOpsPsrEnW<'_, RegBcnIepsr1Spec> {
        RWmacOpsPsrEnW::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_iepsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_iepsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnIepsr1Spec;
impl crate::RegisterSpec for RegBcnIepsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_iepsr1::R`](R) reader structure"]
impl crate::Readable for RegBcnIepsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_iepsr1::W`](W) writer structure"]
impl crate::Writable for RegBcnIepsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_IEPSR1 to value 0"]
impl crate::Resettable for RegBcnIepsr1Spec {}
