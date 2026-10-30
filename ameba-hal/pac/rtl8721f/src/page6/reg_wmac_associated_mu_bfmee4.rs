#[doc = "Register `REG_WMAC_ASSOCIATED_MU_BFMEE4` reader"]
pub type R = crate::R<RegWmacAssociatedMuBfmee4Spec>;
#[doc = "Register `REG_WMAC_ASSOCIATED_MU_BFMEE4` writer"]
pub type W = crate::W<RegWmacAssociatedMuBfmee4Spec>;
#[doc = "Field `BIT_WMAC_MU_BFEE4_AID` reader - P_AID of the target beamformee4"]
pub type BitWmacMuBfee4AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE4_AID` writer - P_AID of the target beamformee4"]
pub type BitWmacMuBfee4AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE4_EN` reader - If enabled, NDPA/BF-rpt-poll would be sent to beamformee4, and CSI rpt of beamformee4 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 4"]
pub type BitWmacMuBfee4EnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_MU_BFEE4_EN` writer - If enabled, NDPA/BF-rpt-poll would be sent to beamformee4, and CSI rpt of beamformee4 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 4"]
pub type BitWmacMuBfee4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_MU_BFEE5_AID` reader - P_AID of the target beamformee5"]
pub type BitWmacMuBfee5AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE5_AID` writer - P_AID of the target beamformee5"]
pub type BitWmacMuBfee5AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_WMAC_MU_BFEE5_EN` reader - If enabled, NDPA/BF-rpt-poll would be sent to beamformee5, and CSI rpt of beamformee5 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 5"]
pub type BitWmacMuBfee5EnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_MU_BFEE5_EN` writer - If enabled, NDPA/BF-rpt-poll would be sent to beamformee5, and CSI rpt of beamformee5 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 5"]
pub type BitWmacMuBfee5EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - P_AID of the target beamformee4"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee4_aid(&self) -> BitWmacMuBfee4AidR {
        BitWmacMuBfee4AidR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee4, and CSI rpt of beamformee4 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 4"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee4_en(&self) -> BitWmacMuBfee4EnR {
        BitWmacMuBfee4EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:24 - P_AID of the target beamformee5"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee5_aid(&self) -> BitWmacMuBfee5AidR {
        BitWmacMuBfee5AidR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee5, and CSI rpt of beamformee5 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 5"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee5_en(&self) -> BitWmacMuBfee5EnR {
        BitWmacMuBfee5EnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - P_AID of the target beamformee4"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee4_aid(
        &mut self,
    ) -> BitWmacMuBfee4AidW<'_, RegWmacAssociatedMuBfmee4Spec> {
        BitWmacMuBfee4AidW::new(self, 0)
    }
    #[doc = "Bit 9 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee4, and CSI rpt of beamformee4 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 4"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee4_en(&mut self) -> BitWmacMuBfee4EnW<'_, RegWmacAssociatedMuBfmee4Spec> {
        BitWmacMuBfee4EnW::new(self, 9)
    }
    #[doc = "Bits 16:24 - P_AID of the target beamformee5"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee5_aid(
        &mut self,
    ) -> BitWmacMuBfee5AidW<'_, RegWmacAssociatedMuBfmee4Spec> {
        BitWmacMuBfee5AidW::new(self, 16)
    }
    #[doc = "Bit 25 - If enabled, NDPA/BF-rpt-poll would be sent to beamformee5, and CSI rpt of beamformee5 can be read from 0x1682\\[11:0\\] when 0x1682\\[14:12\\] is 5"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfee5_en(&mut self) -> BitWmacMuBfee5EnW<'_, RegWmacAssociatedMuBfmee4Spec> {
        BitWmacMuBfee5EnW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_associated_mu_bfmee4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_associated_mu_bfmee4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacAssociatedMuBfmee4Spec;
impl crate::RegisterSpec for RegWmacAssociatedMuBfmee4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_associated_mu_bfmee4::R`](R) reader structure"]
impl crate::Readable for RegWmacAssociatedMuBfmee4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_associated_mu_bfmee4::W`](W) writer structure"]
impl crate::Writable for RegWmacAssociatedMuBfmee4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_ASSOCIATED_MU_BFMEE4 to value 0"]
impl crate::Resettable for RegWmacAssociatedMuBfmee4Spec {}
