#[doc = "Register `REG_ASSOCIATED_BFMEE_SEL` reader"]
pub type R = crate::R<RegAssociatedBfmeeSelSpec>;
#[doc = "Register `REG_ASSOCIATED_BFMEE_SEL` writer"]
pub type W = crate::W<RegAssociatedBfmeeSelSpec>;
#[doc = "Field `BIT_BFMEE0_AID` reader - P_AID of the target beamformee0"]
pub type BitBfmee0AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BFMEE0_AID` writer - P_AID of the target beamformee0"]
pub type BitBfmee0AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_BFMEE0_EN` reader - If enabled, NDPA would be sent to beamformee0."]
pub type BitBfmee0EnR = crate::BitReader;
#[doc = "Field `BIT_BFMEE0_EN` writer - If enabled, NDPA would be sent to beamformee0."]
pub type BitBfmee0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BFMEE1_AID` reader - P_AID of the target beamformee1"]
pub type BitBfmee1AidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BFMEE1_AID` writer - P_AID of the target beamformee1"]
pub type BitBfmee1AidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_BFMEE1_EN` reader - If enabled, NDPA would be sent to beamformee1."]
pub type BitBfmee1EnR = crate::BitReader;
#[doc = "Field `BIT_BFMEE1_EN` writer - If enabled, NDPA would be sent to beamformee1."]
pub type BitBfmee1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - P_AID of the target beamformee0"]
    #[inline(always)]
    pub fn bit_bfmee0_aid(&self) -> BitBfmee0AidR {
        BitBfmee0AidR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - If enabled, NDPA would be sent to beamformee0."]
    #[inline(always)]
    pub fn bit_bfmee0_en(&self) -> BitBfmee0EnR {
        BitBfmee0EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:24 - P_AID of the target beamformee1"]
    #[inline(always)]
    pub fn bit_bfmee1_aid(&self) -> BitBfmee1AidR {
        BitBfmee1AidR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - If enabled, NDPA would be sent to beamformee1."]
    #[inline(always)]
    pub fn bit_bfmee1_en(&self) -> BitBfmee1EnR {
        BitBfmee1EnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - P_AID of the target beamformee0"]
    #[inline(always)]
    pub fn bit_bfmee0_aid(&mut self) -> BitBfmee0AidW<'_, RegAssociatedBfmeeSelSpec> {
        BitBfmee0AidW::new(self, 0)
    }
    #[doc = "Bit 9 - If enabled, NDPA would be sent to beamformee0."]
    #[inline(always)]
    pub fn bit_bfmee0_en(&mut self) -> BitBfmee0EnW<'_, RegAssociatedBfmeeSelSpec> {
        BitBfmee0EnW::new(self, 9)
    }
    #[doc = "Bits 16:24 - P_AID of the target beamformee1"]
    #[inline(always)]
    pub fn bit_bfmee1_aid(&mut self) -> BitBfmee1AidW<'_, RegAssociatedBfmeeSelSpec> {
        BitBfmee1AidW::new(self, 16)
    }
    #[doc = "Bit 25 - If enabled, NDPA would be sent to beamformee1."]
    #[inline(always)]
    pub fn bit_bfmee1_en(&mut self) -> BitBfmee1EnW<'_, RegAssociatedBfmeeSelSpec> {
        BitBfmee1EnW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_associated_bfmee_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_associated_bfmee_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAssociatedBfmeeSelSpec;
impl crate::RegisterSpec for RegAssociatedBfmeeSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_associated_bfmee_sel::R`](R) reader structure"]
impl crate::Readable for RegAssociatedBfmeeSelSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_associated_bfmee_sel::W`](W) writer structure"]
impl crate::Writable for RegAssociatedBfmeeSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ASSOCIATED_BFMEE_SEL to value 0"]
impl crate::Resettable for RegAssociatedBfmeeSelSpec {}
