#[doc = "Register `REG_BCN_IEPSR0` reader"]
pub type R = crate::R<RegBcnIepsr0Spec>;
#[doc = "Register `REG_BCN_IEPSR0` writer"]
pub type W = crate::W<RegBcnIepsr0Spec>;
#[doc = "Field `r_WMAC_BCN_IE_ID0` reader - specified Element ID, set corresponding interrupt when it is parsed"]
pub type RWmacBcnIeId0R = crate::FieldReader;
#[doc = "Field `r_WMAC_BCN_IE_ID0` writer - specified Element ID, set corresponding interrupt when it is parsed"]
pub type RWmacBcnIeId0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_WMAC_BCN_ID0_EXT` reader - indicate current ie is the extend element which need to parser extend id field"]
pub type RWmacBcnId0ExtR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_ID0_EXT` writer - indicate current ie is the extend element which need to parser extend id field"]
pub type RWmacBcnId0ExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_BCN_IE_ID1` reader - specified Element ID, set corresponding interrupt when it is parsed"]
pub type RWmacBcnIeId1R = crate::FieldReader;
#[doc = "Field `r_WMAC_BCN_IE_ID1` writer - specified Element ID, set corresponding interrupt when it is parsed"]
pub type RWmacBcnIeId1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_WMAC_BCN_ID1_EXT` reader - indicate current ie is the extend element which need to parser extend id field"]
pub type RWmacBcnId1ExtR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_ID1_EXT` writer - indicate current ie is the extend element which need to parser extend id field"]
pub type RWmacBcnId1ExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_BCN_MBSSID_N` reader - SW write the value of MaxBSSID Indicator field"]
pub type RWmacBcnMbssidNR = crate::FieldReader;
#[doc = "Field `r_WMAC_BCN_MBSSID_N` writer - SW write the value of MaxBSSID Indicator field"]
pub type RWmacBcnMbssidNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_WMAC_BCN_SW_MBSSID` reader - enable the MaxBSSID indicator value from SW"]
pub type RWmacBcnSwMbssidR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_SW_MBSSID` writer - enable the MaxBSSID indicator value from SW"]
pub type RWmacBcnSwMbssidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_BCN_MBSSID_PSR` reader - enable parser multiple BSSID element"]
pub type RWmacBcnMbssidPsrR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_MBSSID_PSR` writer - enable parser multiple BSSID element"]
pub type RWmacBcnMbssidPsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_BCN_IE0_PSR` reader - enable parser specified ie0 element"]
pub type RWmacBcnIe0PsrR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_IE0_PSR` writer - enable parser specified ie0 element"]
pub type RWmacBcnIe0PsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_BCN_IE1_PSR` reader - enable parser specified ie 1 element"]
pub type RWmacBcnIe1PsrR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_IE1_PSR` writer - enable parser specified ie 1 element"]
pub type RWmacBcnIe1PsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_BCN_CLR_N` reader - enable clear macbssid indicator when no multiple BSSID element is found."]
pub type RWmacBcnClrNR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_CLR_N` writer - enable clear macbssid indicator when no multiple BSSID element is found."]
pub type RWmacBcnClrNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_WMAC_BCN_MBSSID_MASK` reader - enable tim no hit don't check whether BC/MC packets are buffered at nontransmitted BSSID"]
pub type RWmacBcnMbssidMaskR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_MBSSID_MASK` writer - enable tim no hit don't check whether BC/MC packets are buffered at nontransmitted BSSID"]
pub type RWmacBcnMbssidMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - specified Element ID, set corresponding interrupt when it is parsed"]
    #[inline(always)]
    pub fn r_wmac_bcn_ie_id0(&self) -> RWmacBcnIeId0R {
        RWmacBcnIeId0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - indicate current ie is the extend element which need to parser extend id field"]
    #[inline(always)]
    pub fn r_wmac_bcn_id0_ext(&self) -> RWmacBcnId0ExtR {
        RWmacBcnId0ExtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - specified Element ID, set corresponding interrupt when it is parsed"]
    #[inline(always)]
    pub fn r_wmac_bcn_ie_id1(&self) -> RWmacBcnIeId1R {
        RWmacBcnIeId1R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - indicate current ie is the extend element which need to parser extend id field"]
    #[inline(always)]
    pub fn r_wmac_bcn_id1_ext(&self) -> RWmacBcnId1ExtR {
        RWmacBcnId1ExtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:25 - SW write the value of MaxBSSID Indicator field"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_n(&self) -> RWmacBcnMbssidNR {
        RWmacBcnMbssidNR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bit 26 - enable the MaxBSSID indicator value from SW"]
    #[inline(always)]
    pub fn r_wmac_bcn_sw_mbssid(&self) -> RWmacBcnSwMbssidR {
        RWmacBcnSwMbssidR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable parser multiple BSSID element"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_psr(&self) -> RWmacBcnMbssidPsrR {
        RWmacBcnMbssidPsrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable parser specified ie0 element"]
    #[inline(always)]
    pub fn r_wmac_bcn_ie0_psr(&self) -> RWmacBcnIe0PsrR {
        RWmacBcnIe0PsrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable parser specified ie 1 element"]
    #[inline(always)]
    pub fn r_wmac_bcn_ie1_psr(&self) -> RWmacBcnIe1PsrR {
        RWmacBcnIe1PsrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable clear macbssid indicator when no multiple BSSID element is found."]
    #[inline(always)]
    pub fn r_wmac_bcn_clr_n(&self) -> RWmacBcnClrNR {
        RWmacBcnClrNR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable tim no hit don't check whether BC/MC packets are buffered at nontransmitted BSSID"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_mask(&self) -> RWmacBcnMbssidMaskR {
        RWmacBcnMbssidMaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - specified Element ID, set corresponding interrupt when it is parsed"]
    #[inline(always)]
    pub fn r_wmac_bcn_ie_id0(&mut self) -> RWmacBcnIeId0W<'_, RegBcnIepsr0Spec> {
        RWmacBcnIeId0W::new(self, 0)
    }
    #[doc = "Bit 8 - indicate current ie is the extend element which need to parser extend id field"]
    #[inline(always)]
    pub fn r_wmac_bcn_id0_ext(&mut self) -> RWmacBcnId0ExtW<'_, RegBcnIepsr0Spec> {
        RWmacBcnId0ExtW::new(self, 8)
    }
    #[doc = "Bits 9:16 - specified Element ID, set corresponding interrupt when it is parsed"]
    #[inline(always)]
    pub fn r_wmac_bcn_ie_id1(&mut self) -> RWmacBcnIeId1W<'_, RegBcnIepsr0Spec> {
        RWmacBcnIeId1W::new(self, 9)
    }
    #[doc = "Bit 17 - indicate current ie is the extend element which need to parser extend id field"]
    #[inline(always)]
    pub fn r_wmac_bcn_id1_ext(&mut self) -> RWmacBcnId1ExtW<'_, RegBcnIepsr0Spec> {
        RWmacBcnId1ExtW::new(self, 17)
    }
    #[doc = "Bits 18:25 - SW write the value of MaxBSSID Indicator field"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_n(&mut self) -> RWmacBcnMbssidNW<'_, RegBcnIepsr0Spec> {
        RWmacBcnMbssidNW::new(self, 18)
    }
    #[doc = "Bit 26 - enable the MaxBSSID indicator value from SW"]
    #[inline(always)]
    pub fn r_wmac_bcn_sw_mbssid(&mut self) -> RWmacBcnSwMbssidW<'_, RegBcnIepsr0Spec> {
        RWmacBcnSwMbssidW::new(self, 26)
    }
    #[doc = "Bit 27 - enable parser multiple BSSID element"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_psr(&mut self) -> RWmacBcnMbssidPsrW<'_, RegBcnIepsr0Spec> {
        RWmacBcnMbssidPsrW::new(self, 27)
    }
    #[doc = "Bit 28 - enable parser specified ie0 element"]
    #[inline(always)]
    pub fn r_wmac_bcn_ie0_psr(&mut self) -> RWmacBcnIe0PsrW<'_, RegBcnIepsr0Spec> {
        RWmacBcnIe0PsrW::new(self, 28)
    }
    #[doc = "Bit 29 - enable parser specified ie 1 element"]
    #[inline(always)]
    pub fn r_wmac_bcn_ie1_psr(&mut self) -> RWmacBcnIe1PsrW<'_, RegBcnIepsr0Spec> {
        RWmacBcnIe1PsrW::new(self, 29)
    }
    #[doc = "Bit 30 - enable clear macbssid indicator when no multiple BSSID element is found."]
    #[inline(always)]
    pub fn r_wmac_bcn_clr_n(&mut self) -> RWmacBcnClrNW<'_, RegBcnIepsr0Spec> {
        RWmacBcnClrNW::new(self, 30)
    }
    #[doc = "Bit 31 - enable tim no hit don't check whether BC/MC packets are buffered at nontransmitted BSSID"]
    #[inline(always)]
    pub fn r_wmac_bcn_mbssid_mask(&mut self) -> RWmacBcnMbssidMaskW<'_, RegBcnIepsr0Spec> {
        RWmacBcnMbssidMaskW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_iepsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_iepsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnIepsr0Spec;
impl crate::RegisterSpec for RegBcnIepsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_iepsr0::R`](R) reader structure"]
impl crate::Readable for RegBcnIepsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_iepsr0::W`](W) writer structure"]
impl crate::Writable for RegBcnIepsr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_IEPSR0 to value 0"]
impl crate::Resettable for RegBcnIepsr0Spec {}
