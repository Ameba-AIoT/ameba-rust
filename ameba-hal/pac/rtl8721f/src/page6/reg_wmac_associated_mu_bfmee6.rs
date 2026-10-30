#[doc = "Register `REG_WMAC_ASSOCIATED_MU_BFMEE6` reader"]
pub type R = crate::R<RegWmacAssociatedMuBfmee6Spec>;
#[doc = "Register `REG_WMAC_ASSOCIATED_MU_BFMEE6` writer"]
pub type W = crate::W<RegWmacAssociatedMuBfmee6Spec>;
#[doc = "Field `BIT_WMAC_MU_BFEE6_AID` reader - P_AID of the target beamformee6"]
pub type BitWmacMuBfee6AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE6_AID` writer - P_AID of the target beamformee6"]
pub type BitWmacMuBfee6AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE6_EN` reader - If enabled, NDPA/BF-rpt-poll would be sent to beamformee6, and CSI rpt of beamformee6 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 6"]
pub type BitWmacMuBfee6EnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_MU_BFEE6_EN` writer - If enabled, NDPA/BF-rpt-poll would be sent to beamformee6, and CSI rpt of beamformee6 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 6"]
pub type BitWmacMuBfee6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_MU_BFEE7_AID` reader - P_AID of the target beamformee7"]
pub type BitWmacMuBfee7AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE7_AID` writer - P_AID of the target beamformee7"]
pub type BitWmacMuBfee7AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE7_EN` reader - If enabled, NDPA/BF-rpt-poll would be sent to beamformee7, and CSI rpt of beamformee7 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 7"]
pub type BitWmacMuBfee7EnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_MU_BFEE7_EN` writer - If enabled, NDPA/BF-rpt-poll would be sent to beamformee7, and CSI rpt of beamformee7 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 7"]
pub type BitWmacMuBfee7EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - P_AID of the target beamformee6"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee6_aid(&self) -> BitWmacMuBfee6AidR {
        BitWmacMuBfee6AidR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee6, and CSI rpt of beamformee6 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 6"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee6_en(&self) -> BitWmacMuBfee6EnR {
        BitWmacMuBfee6EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:24 - P_AID of the target beamformee7"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee7_aid(&self) -> BitWmacMuBfee7AidR {
        BitWmacMuBfee7AidR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee7, and CSI rpt of beamformee7 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 7"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee7_en(&self) -> BitWmacMuBfee7EnR {
        BitWmacMuBfee7EnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - P_AID of the target beamformee6"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee6_aid(
        &mut self,
    ) -> BitWmacMuBfee6AidW<'_, RegWmacAssociatedMuBfmee6Spec> {
        BitWmacMuBfee6AidW::new(self, 0)
    }
    #[doc = "Bit 9 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee6, and CSI rpt of beamformee6 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 6"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee6_en(&mut self) -> BitWmacMuBfee6EnW<'_, RegWmacAssociatedMuBfmee6Spec> {
        BitWmacMuBfee6EnW::new(self, 9)
    }
    #[doc = "Bits 16:24 - P_AID of the target beamformee7"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee7_aid(
        &mut self,
    ) -> BitWmacMuBfee7AidW<'_, RegWmacAssociatedMuBfmee6Spec> {
        BitWmacMuBfee7AidW::new(self, 16)
    }
    #[doc = "Bit 25 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee7, and CSI rpt of beamformee7 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 7"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee7_en(&mut self) -> BitWmacMuBfee7EnW<'_, RegWmacAssociatedMuBfmee6Spec> {
        BitWmacMuBfee7EnW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_associated_mu_bfmee6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_associated_mu_bfmee6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacAssociatedMuBfmee6Spec;
impl crate::RegisterSpec for RegWmacAssociatedMuBfmee6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_associated_mu_bfmee6::R`](R) reader structure"]
impl crate::Readable for RegWmacAssociatedMuBfmee6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_associated_mu_bfmee6::W`](W) writer structure"]
impl crate::Writable for RegWmacAssociatedMuBfmee6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_ASSOCIATED_MU_BFMEE6 to value 0"]
impl crate::Resettable for RegWmacAssociatedMuBfmee6Spec {}
