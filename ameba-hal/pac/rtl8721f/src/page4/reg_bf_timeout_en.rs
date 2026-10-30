#[doc = "Register `REG_BF_TIMEOUT_EN` reader"]
pub type R = crate::R<RegBfTimeoutEnSpec>;
#[doc = "Register `REG_BF_TIMEOUT_EN` writer"]
pub type W = crate::W<RegBfTimeoutEnSpec>;
#[doc = "Field `BIT_bf0_timeout_en` reader - 1: when beamforming 0 timer count to 0,then disable beamforming 0"]
pub type BitBf0TimeoutEnR = crate::BitReader;
#[doc = "Field `BIT_bf0_timeout_en` writer - 1: when beamforming 0 timer count to 0,then disable beamforming 0"]
pub type BitBf0TimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_bf1_timeout_en` reader - 1: when beamforming 1 timer count to 0,then disable beamforming 0"]
pub type BitBf1TimeoutEnR = crate::BitReader;
#[doc = "Field `BIT_bf1_timeout_en` writer - 1: when beamforming 1 timer count to 0,then disable beamforming 0"]
pub type BitBf1TimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_HT_LDPC` reader - 1: enable ldpc in ht rate mode"]
pub type BitEnHtLdpcR = crate::BitReader;
#[doc = "Field `BIT_EN_HT_LDPC` writer - 1: enable ldpc in ht rate mode"]
pub type BitEnHtLdpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_VHT_LDPC` reader - 1: enable ldpc in vht rate mode"]
pub type BitEnVhtLdpcR = crate::BitReader;
#[doc = "Field `BIT_EN_VHT_LDPC` writer - 1: enable ldpc in vht rate mode"]
pub type BitEnVhtLdpcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: when beamforming 0 timer count to 0,then disable beamforming 0"]
    #[inline(always)]
    pub fn bit_bf0_timeout_en(&self) -> BitBf0TimeoutEnR {
        BitBf0TimeoutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: when beamforming 1 timer count to 0,then disable beamforming 0"]
    #[inline(always)]
    pub fn bit_bf1_timeout_en(&self) -> BitBf1TimeoutEnR {
        BitBf1TimeoutEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: enable ldpc in ht rate mode"]
    #[inline(always)]
    pub fn bit_en_ht_ldpc(&self) -> BitEnHtLdpcR {
        BitEnHtLdpcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: enable ldpc in vht rate mode"]
    #[inline(always)]
    pub fn bit_en_vht_ldpc(&self) -> BitEnVhtLdpcR {
        BitEnVhtLdpcR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: when beamforming 0 timer count to 0,then disable beamforming 0"]
    #[inline(always)]
    pub fn bit_bf0_timeout_en(&mut self) -> BitBf0TimeoutEnW<'_, RegBfTimeoutEnSpec> {
        BitBf0TimeoutEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: when beamforming 1 timer count to 0,then disable beamforming 0"]
    #[inline(always)]
    pub fn bit_bf1_timeout_en(&mut self) -> BitBf1TimeoutEnW<'_, RegBfTimeoutEnSpec> {
        BitBf1TimeoutEnW::new(self, 1)
    }
    #[doc = "Bit 8 - 1: enable ldpc in ht rate mode"]
    #[inline(always)]
    pub fn bit_en_ht_ldpc(&mut self) -> BitEnHtLdpcW<'_, RegBfTimeoutEnSpec> {
        BitEnHtLdpcW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: enable ldpc in vht rate mode"]
    #[inline(always)]
    pub fn bit_en_vht_ldpc(&mut self) -> BitEnVhtLdpcW<'_, RegBfTimeoutEnSpec> {
        BitEnVhtLdpcW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bf_timeout_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bf_timeout_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBfTimeoutEnSpec;
impl crate::RegisterSpec for RegBfTimeoutEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bf_timeout_en::R`](R) reader structure"]
impl crate::Readable for RegBfTimeoutEnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bf_timeout_en::W`](W) writer structure"]
impl crate::Writable for RegBfTimeoutEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BF_TIMEOUT_EN to value 0x0303"]
impl crate::Resettable for RegBfTimeoutEnSpec {
    const RESET_VALUE: u32 = 0x0303;
}
