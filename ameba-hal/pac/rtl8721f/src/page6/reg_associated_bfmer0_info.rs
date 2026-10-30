#[doc = "Register `REG_ASSOCIATED_BFMER0_INFO` reader"]
pub type R = crate::R<RegAssociatedBfmer0InfoSpec>;
#[doc = "Register `REG_ASSOCIATED_BFMER0_INFO` writer"]
pub type W = crate::W<RegAssociatedBfmer0InfoSpec>;
#[doc = "Field `BIT_r_WMAC_SOUNDING_RXADD_R0_V1` reader - MACID of the associated beamformer0 bit\\[31:0\\]"]
pub type BitRWmacSoundingRxaddR0V1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_r_WMAC_SOUNDING_RXADD_R0_V1` writer - MACID of the associated beamformer0 bit\\[31:0\\]"]
pub type BitRWmacSoundingRxaddR0V1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MACID of the associated beamformer0 bit\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_r_wmac_sounding_rxadd_r0_v1(&self) -> BitRWmacSoundingRxaddR0V1R {
        BitRWmacSoundingRxaddR0V1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MACID of the associated beamformer0 bit\\[31:0\\]"]
    #[inline(always)]
    pub fn bit_r_wmac_sounding_rxadd_r0_v1(
        &mut self,
    ) -> BitRWmacSoundingRxaddR0V1W<'_, RegAssociatedBfmer0InfoSpec> {
        BitRWmacSoundingRxaddR0V1W::new(self, 0)
    }
}
#[doc = "Associated Beamformer0 Info Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_associated_bfmer0_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_associated_bfmer0_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAssociatedBfmer0InfoSpec;
impl crate::RegisterSpec for RegAssociatedBfmer0InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_associated_bfmer0_info::R`](R) reader structure"]
impl crate::Readable for RegAssociatedBfmer0InfoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_associated_bfmer0_info::W`](W) writer structure"]
impl crate::Writable for RegAssociatedBfmer0InfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ASSOCIATED_BFMER0_INFO to value 0"]
impl crate::Resettable for RegAssociatedBfmer0InfoSpec {}
