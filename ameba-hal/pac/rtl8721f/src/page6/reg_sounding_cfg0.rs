#[doc = "Register `REG_SOUNDING_CFG0` reader"]
pub type R = crate::R<RegSoundingCfg0Spec>;
#[doc = "Register `REG_SOUNDING_CFG0` writer"]
pub type W = crate::W<RegSoundingCfg0Spec>;
#[doc = "Field `BIT_r_WMAC_BFINFO_20M_0` reader - BF Info of the associated beamformer0 at BW=20M bit\\[2:0\\]: Nc index bit\\[5:3\\]: Nr index bit\\[7:6\\]: grouping bit\\[9:8\\]: codebook info bit\\[11:10\\]: coefficient size"]
pub type BitRWmacBfinfo20m0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_BFINFO_20M_0` writer - BF Info of the associated beamformer0 at BW=20M bit\\[2:0\\]: Nc index bit\\[5:3\\]: Nr index bit\\[7:6\\]: grouping bit\\[9:8\\]: codebook info bit\\[11:10\\]: coefficient size"]
pub type BitRWmacBfinfo20m0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_r_WMAC_BFINFO_20M_1` reader - BF Info of the associated beamformer1 at BW=20M bit\\[18:16\\]: Nc index bit\\[21:19\\]: Nr index bit\\[23:22\\]: grouping bit\\[25:24\\]: codebook info bit\\[27:26\\]: coefficient size"]
pub type BitRWmacBfinfo20m1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_r_WMAC_BFINFO_20M_1` writer - BF Info of the associated beamformer1 at BW=20M bit\\[18:16\\]: Nc index bit\\[21:19\\]: Nr index bit\\[23:22\\]: grouping bit\\[25:24\\]: codebook info bit\\[27:26\\]: coefficient size"]
pub type BitRWmacBfinfo20m1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - BF Info of the associated beamformer0 at BW=20M bit\\[2:0\\]: Nc index bit\\[5:3\\]: Nr index bit\\[7:6\\]: grouping bit\\[9:8\\]: codebook info bit\\[11:10\\]: coefficient size"]
    #[inline(always)]
    pub fn bit_r_wmac_bfinfo_20m_0(&self) -> BitRWmacBfinfo20m0R {
        BitRWmacBfinfo20m0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - BF Info of the associated beamformer1 at BW=20M bit\\[18:16\\]: Nc index bit\\[21:19\\]: Nr index bit\\[23:22\\]: grouping bit\\[25:24\\]: codebook info bit\\[27:26\\]: coefficient size"]
    #[inline(always)]
    pub fn bit_r_wmac_bfinfo_20m_1(&self) -> BitRWmacBfinfo20m1R {
        BitRWmacBfinfo20m1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - BF Info of the associated beamformer0 at BW=20M bit\\[2:0\\]: Nc index bit\\[5:3\\]: Nr index bit\\[7:6\\]: grouping bit\\[9:8\\]: codebook info bit\\[11:10\\]: coefficient size"]
    #[inline(always)]
    pub fn bit_r_wmac_bfinfo_20m_0(&mut self) -> BitRWmacBfinfo20m0W<'_, RegSoundingCfg0Spec> {
        BitRWmacBfinfo20m0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - BF Info of the associated beamformer1 at BW=20M bit\\[18:16\\]: Nc index bit\\[21:19\\]: Nr index bit\\[23:22\\]: grouping bit\\[25:24\\]: codebook info bit\\[27:26\\]: coefficient size"]
    #[inline(always)]
    pub fn bit_r_wmac_bfinfo_20m_1(&mut self) -> BitRWmacBfinfo20m1W<'_, RegSoundingCfg0Spec> {
        BitRWmacBfinfo20m1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sounding_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sounding_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSoundingCfg0Spec;
impl crate::RegisterSpec for RegSoundingCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sounding_cfg0::R`](R) reader structure"]
impl crate::Readable for RegSoundingCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sounding_cfg0::W`](W) writer structure"]
impl crate::Writable for RegSoundingCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SOUNDING_CFG0 to value 0"]
impl crate::Resettable for RegSoundingCfg0Spec {}
