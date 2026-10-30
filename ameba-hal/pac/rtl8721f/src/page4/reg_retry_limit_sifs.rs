#[doc = "Register `REG_RETRY_LIMIT_SIFS` reader"]
pub type R = crate::R<RegRetryLimitSifsSpec>;
#[doc = "Register `REG_RETRY_LIMIT_SIFS` writer"]
pub type W = crate::W<RegRetryLimitSifsSpec>;
#[doc = "Field `BIT_SPEC_SIFS_CCK_PTCL` reader - The spec SIFS value for duration calculation"]
pub type BitSpecSifsCckPtclR = crate::FieldReader;
#[doc = "Field `BIT_SPEC_SIFS_CCK_PTCL` writer - The spec SIFS value for duration calculation"]
pub type BitSpecSifsCckPtclW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_SPEC_SIFS_OFDM_PTCL` reader - The spec SIFS value for duration calculation"]
pub type BitSpecSifsOfdmPtclR = crate::FieldReader;
#[doc = "Field `BIT_SPEC_SIFS_OFDM_PTCL` writer - The spec SIFS value for duration calculation"]
pub type BitSpecSifsOfdmPtclW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_LRL` reader - Long Retry Limit"]
pub type BitLrlR = crate::FieldReader;
#[doc = "Field `BIT_LRL` writer - Long Retry Limit"]
pub type BitLrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_SRL` reader - Short Retry Limit"]
pub type BitSrlR = crate::FieldReader;
#[doc = "Field `BIT_SRL` writer - Short Retry Limit"]
pub type BitSrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - The spec SIFS value for duration calculation"]
    #[inline(always)]
    pub fn bit_spec_sifs_cck_ptcl(&self) -> BitSpecSifsCckPtclR {
        BitSpecSifsCckPtclR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The spec SIFS value for duration calculation"]
    #[inline(always)]
    pub fn bit_spec_sifs_ofdm_ptcl(&self) -> BitSpecSifsOfdmPtclR {
        BitSpecSifsOfdmPtclR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - Long Retry Limit"]
    #[inline(always)]
    pub fn bit_lrl(&self) -> BitLrlR {
        BitLrlR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Short Retry Limit"]
    #[inline(always)]
    pub fn bit_srl(&self) -> BitSrlR {
        BitSrlR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The spec SIFS value for duration calculation"]
    #[inline(always)]
    pub fn bit_spec_sifs_cck_ptcl(&mut self) -> BitSpecSifsCckPtclW<'_, RegRetryLimitSifsSpec> {
        BitSpecSifsCckPtclW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The spec SIFS value for duration calculation"]
    #[inline(always)]
    pub fn bit_spec_sifs_ofdm_ptcl(&mut self) -> BitSpecSifsOfdmPtclW<'_, RegRetryLimitSifsSpec> {
        BitSpecSifsOfdmPtclW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Long Retry Limit"]
    #[inline(always)]
    pub fn bit_lrl(&mut self) -> BitLrlW<'_, RegRetryLimitSifsSpec> {
        BitLrlW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Short Retry Limit"]
    #[inline(always)]
    pub fn bit_srl(&mut self) -> BitSrlW<'_, RegRetryLimitSifsSpec> {
        BitSrlW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_retry_limit_sifs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_retry_limit_sifs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRetryLimitSifsSpec;
impl crate::RegisterSpec for RegRetryLimitSifsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_retry_limit_sifs::R`](R) reader structure"]
impl crate::Readable for RegRetryLimitSifsSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_retry_limit_sifs::W`](W) writer structure"]
impl crate::Writable for RegRetryLimitSifsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RETRY_LIMIT_SIFS to value 0x2020_100a"]
impl crate::Resettable for RegRetryLimitSifsSpec {
    const RESET_VALUE: u32 = 0x2020_100a;
}
