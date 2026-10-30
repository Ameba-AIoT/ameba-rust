#[doc = "Register `REG_WMAC_ASSOCIATED_MU_BFMEE2` reader"]
pub type R = crate::R<RegWmacAssociatedMuBfmee2Spec>;
#[doc = "Register `REG_WMAC_ASSOCIATED_MU_BFMEE2` writer"]
pub type W = crate::W<RegWmacAssociatedMuBfmee2Spec>;
#[doc = "Field `BIT_WMAC_MU_BFEE2_AID` reader - P_AID of the target beamformee2"]
pub type BitWmacMuBfee2AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE2_AID` writer - P_AID of the target beamformee2"]
pub type BitWmacMuBfee2AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE2_EN` reader - If enabled, NDPA/BF-rpt-poll would be sent to beamformee2, and CSI rpt of beamformee2 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 2"]
pub type BitWmacMuBfee2EnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_MU_BFEE2_EN` writer - If enabled, NDPA/BF-rpt-poll would be sent to beamformee2, and CSI rpt of beamformee2 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 2"]
pub type BitWmacMuBfee2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_status_bfee2` reader - beamforming report status of beamformee2 0: fail, 1: success"]
pub type BitStatusBfee2R = crate::BitReader;
#[doc = "Field `BIT_WMAC_MU_BFEE3_AID` reader - P_AID of the target beamformee3"]
pub type BitWmacMuBfee3AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE3_AID` writer - P_AID of the target beamformee3"]
pub type BitWmacMuBfee3AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE3_EN` reader - If enabled, NDPA/BF-rpt-poll would be sent to beamformee3, and CSI rpt of beamformee3 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 3"]
pub type BitWmacMuBfee3EnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_MU_BFEE3_EN` writer - If enabled, NDPA/BF-rpt-poll would be sent to beamformee3, and CSI rpt of beamformee3 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 3"]
pub type BitWmacMuBfee3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_status_bfee3` reader - beamforming report status of beamformee3 0: fail, 1: success"]
pub type BitStatusBfee3R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - P_AID of the target beamformee2"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee2_aid(&self) -> BitWmacMuBfee2AidR {
        BitWmacMuBfee2AidR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee2, and CSI rpt of beamformee2 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 2"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee2_en(&self) -> BitWmacMuBfee2EnR {
        BitWmacMuBfee2EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - beamforming report status of beamformee2 0: fail, 1: success"]
    #[inline(always)]
    pub fn bit_status_bfee2(&self) -> BitStatusBfee2R {
        BitStatusBfee2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:24 - P_AID of the target beamformee3"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee3_aid(&self) -> BitWmacMuBfee3AidR {
        BitWmacMuBfee3AidR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee3, and CSI rpt of beamformee3 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 3"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee3_en(&self) -> BitWmacMuBfee3EnR {
        BitWmacMuBfee3EnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - beamforming report status of beamformee3 0: fail, 1: success"]
    #[inline(always)]
    pub fn bit_status_bfee3(&self) -> BitStatusBfee3R {
        BitStatusBfee3R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - P_AID of the target beamformee2"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee2_aid(
        &mut self,
    ) -> BitWmacMuBfee2AidW<'_, RegWmacAssociatedMuBfmee2Spec> {
        BitWmacMuBfee2AidW::new(self, 0)
    }
    #[doc = "Bit 9 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee2, and CSI rpt of beamformee2 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 2"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee2_en(&mut self) -> BitWmacMuBfee2EnW<'_, RegWmacAssociatedMuBfmee2Spec> {
        BitWmacMuBfee2EnW::new(self, 9)
    }
    #[doc = "Bits 16:24 - P_AID of the target beamformee3"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee3_aid(
        &mut self,
    ) -> BitWmacMuBfee3AidW<'_, RegWmacAssociatedMuBfmee2Spec> {
        BitWmacMuBfee3AidW::new(self, 16)
    }
    #[doc = "Bit 25 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee3, and CSI rpt of beamformee3 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 3"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee3_en(&mut self) -> BitWmacMuBfee3EnW<'_, RegWmacAssociatedMuBfmee2Spec> {
        BitWmacMuBfee3EnW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_associated_mu_bfmee2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_associated_mu_bfmee2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacAssociatedMuBfmee2Spec;
impl crate::RegisterSpec for RegWmacAssociatedMuBfmee2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_associated_mu_bfmee2::R`](R) reader structure"]
impl crate::Readable for RegWmacAssociatedMuBfmee2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_associated_mu_bfmee2::W`](W) writer structure"]
impl crate::Writable for RegWmacAssociatedMuBfmee2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_ASSOCIATED_MU_BFMEE2 to value 0"]
impl crate::Resettable for RegWmacAssociatedMuBfmee2Spec {}
