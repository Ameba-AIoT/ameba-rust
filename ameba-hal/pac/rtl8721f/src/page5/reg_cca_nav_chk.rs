#[doc = "Register `REG_CCA_NAV_CHK` reader"]
pub type R = crate::R<RegCcaNavChkSpec>;
#[doc = "Register `REG_CCA_NAV_CHK` writer"]
pub type W = crate::W<RegCcaNavChkSpec>;
#[doc = "Field `BIT_CTN_CHK_CCA_P20` reader - When CCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkCcaP20R = crate::BitReader;
#[doc = "Field `BIT_CTN_CHK_CCA_P20` writer - When CCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkCcaP20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CTN_CHK_CCA_S20` reader - When SECONDARY_CCA_20 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
pub type BitCtnChkCcaS20R = crate::BitReader;
#[doc = "Field `BIT_CTN_CHK_CCA_S20` writer - When SECONDARY_CCA_20 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
pub type BitCtnChkCcaS20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CTN_CHK_CCA_S40` reader - When SECONDARY_CCA_40 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
pub type BitCtnChkCcaS40R = crate::BitReader;
#[doc = "Field `BIT_CTN_CHK_CCA_S40` writer - When SECONDARY_CCA_40 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
pub type BitCtnChkCcaS40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CTN_CHK_EDCCA` reader - When EDCCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkEdccaR = crate::BitReader;
#[doc = "Field `BIT_CTN_CHK_EDCCA` writer - When EDCCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkEdccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CTN_CHK_ZGBTCCA` reader - When ~GNT_WL(BT/ZigBee) assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkZgbtccaR = crate::BitReader;
#[doc = "Field `BIT_CTN_CHK_ZGBTCCA` writer - When ~GNT_WL(BT/ZigBee) assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkZgbtccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CTN_CHK_BASIC_NAV` reader - When BASIC NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkBasicNavR = crate::BitReader;
#[doc = "Field `BIT_CTN_CHK_BASIC_NAV` writer - When BASIC NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkBasicNavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CTN_CHK_INTRA_NAV` reader - When INTRA NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkIntraNavR = crate::BitReader;
#[doc = "Field `BIT_CTN_CHK_INTRA_NAV` writer - When INTRA NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitCtnChkIntraNavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SR_CTN_CHK_CCA_S20` reader - When SECONDARY_CCA_20 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
pub type BitSrCtnChkCcaS20R = crate::BitReader;
#[doc = "Field `BIT_SR_CTN_CHK_CCA_S20` writer - When SECONDARY_CCA_20 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
pub type BitSrCtnChkCcaS20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SR_CTN_CHK_CCA_S40` reader - When SECONDARY_CCA_40 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
pub type BitSrCtnChkCcaS40R = crate::BitReader;
#[doc = "Field `BIT_SR_CTN_CHK_CCA_S40` writer - When SECONDARY_CCA_40 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
pub type BitSrCtnChkCcaS40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SR_CTN_CHK_EDCCA` reader - When EDCCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitSrCtnChkEdccaR = crate::BitReader;
#[doc = "Field `BIT_SR_CTN_CHK_EDCCA` writer - When EDCCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitSrCtnChkEdccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SR_CTN_CHK_ZGBTCCA` reader - When ~GNT_WL(BT/ZigBee) assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitSrCtnChkZgbtccaR = crate::BitReader;
#[doc = "Field `BIT_SR_CTN_CHK_ZGBTCCA` writer - When ~GNT_WL(BT/ZigBee) assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitSrCtnChkZgbtccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SR_CTN_CHK__NAV` reader - When BASIC or INTRA NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitSrCtnChk_NavR = crate::BitReader;
#[doc = "Field `BIT_SR_CTN_CHK__NAV` writer - When BASIC or INTRA NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
pub type BitSrCtnChk_NavW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When CCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_cca_p20(&self) -> BitCtnChkCcaP20R {
        BitCtnChkCcaP20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When SECONDARY_CCA_20 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_cca_s20(&self) -> BitCtnChkCcaS20R {
        BitCtnChkCcaS20R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When SECONDARY_CCA_40 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_cca_s40(&self) -> BitCtnChkCcaS40R {
        BitCtnChkCcaS40R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When EDCCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_edcca(&self) -> BitCtnChkEdccaR {
        BitCtnChkEdccaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When ~GNT_WL(BT/ZigBee) assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_zgbtcca(&self) -> BitCtnChkZgbtccaR {
        BitCtnChkZgbtccaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When BASIC NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_basic_nav(&self) -> BitCtnChkBasicNavR {
        BitCtnChkBasicNavR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When INTRA NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_intra_nav(&self) -> BitCtnChkIntraNavR {
        BitCtnChkIntraNavR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - When SECONDARY_CCA_20 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk_cca_s20(&self) -> BitSrCtnChkCcaS20R {
        BitSrCtnChkCcaS20R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When SECONDARY_CCA_40 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk_cca_s40(&self) -> BitSrCtnChkCcaS40R {
        BitSrCtnChkCcaS40R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When EDCCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk_edcca(&self) -> BitSrCtnChkEdccaR {
        BitSrCtnChkEdccaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When ~GNT_WL(BT/ZigBee) assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk_zgbtcca(&self) -> BitSrCtnChkZgbtccaR {
        BitSrCtnChkZgbtccaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When BASIC or INTRA NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk__nav(&self) -> BitSrCtnChk_NavR {
        BitSrCtnChk_NavR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When CCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_cca_p20(&mut self) -> BitCtnChkCcaP20W<'_, RegCcaNavChkSpec> {
        BitCtnChkCcaP20W::new(self, 0)
    }
    #[doc = "Bit 1 - When SECONDARY_CCA_20 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_cca_s20(&mut self) -> BitCtnChkCcaS20W<'_, RegCcaNavChkSpec> {
        BitCtnChkCcaS20W::new(self, 1)
    }
    #[doc = "Bit 2 - When SECONDARY_CCA_40 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_cca_s40(&mut self) -> BitCtnChkCcaS40W<'_, RegCcaNavChkSpec> {
        BitCtnChkCcaS40W::new(self, 2)
    }
    #[doc = "Bit 3 - When EDCCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_edcca(&mut self) -> BitCtnChkEdccaW<'_, RegCcaNavChkSpec> {
        BitCtnChkEdccaW::new(self, 3)
    }
    #[doc = "Bit 4 - When ~GNT_WL(BT/ZigBee) assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_zgbtcca(&mut self) -> BitCtnChkZgbtccaW<'_, RegCcaNavChkSpec> {
        BitCtnChkZgbtccaW::new(self, 4)
    }
    #[doc = "Bit 5 - When BASIC NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_basic_nav(&mut self) -> BitCtnChkBasicNavW<'_, RegCcaNavChkSpec> {
        BitCtnChkBasicNavW::new(self, 5)
    }
    #[doc = "Bit 6 - When INTRA NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_ctn_chk_intra_nav(&mut self) -> BitCtnChkIntraNavW<'_, RegCcaNavChkSpec> {
        BitCtnChkIntraNavW::new(self, 6)
    }
    #[doc = "Bit 8 - When SECONDARY_CCA_20 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk_cca_s20(&mut self) -> BitSrCtnChkCcaS20W<'_, RegCcaNavChkSpec> {
        BitSrCtnChkCcaS20W::new(self, 8)
    }
    #[doc = "Bit 9 - When SECONDARY_CCA_40 assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Can backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk_cca_s40(&mut self) -> BitSrCtnChkCcaS40W<'_, RegCcaNavChkSpec> {
        BitSrCtnChkCcaS40W::new(self, 9)
    }
    #[doc = "Bit 10 - When EDCCA assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk_edcca(&mut self) -> BitSrCtnChkEdccaW<'_, RegCcaNavChkSpec> {
        BitSrCtnChkEdccaW::new(self, 10)
    }
    #[doc = "Bit 11 - When ~GNT_WL(BT/ZigBee) assert in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk_zgbtcca(&mut self) -> BitSrCtnChkZgbtccaW<'_, RegCcaNavChkSpec> {
        BitSrCtnChkZgbtccaW::new(self, 11)
    }
    #[doc = "Bit 12 - When BASIC or INTRA NAV > 0 in the AIFS/Backoff before active TX 0: Can backoff, can TX 1: Cannot backoff, cannot TX"]
    #[inline(always)]
    pub fn bit_sr_ctn_chk__nav(&mut self) -> BitSrCtnChk_NavW<'_, RegCcaNavChkSpec> {
        BitSrCtnChk_NavW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cca_nav_chk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cca_nav_chk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCcaNavChkSpec;
impl crate::RegisterSpec for RegCcaNavChkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cca_nav_chk::R`](R) reader structure"]
impl crate::Readable for RegCcaNavChkSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cca_nav_chk::W`](W) writer structure"]
impl crate::Writable for RegCcaNavChkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CCA_NAV_CHK to value 0x1f7f"]
impl crate::Resettable for RegCcaNavChkSpec {
    const RESET_VALUE: u32 = 0x1f7f;
}
