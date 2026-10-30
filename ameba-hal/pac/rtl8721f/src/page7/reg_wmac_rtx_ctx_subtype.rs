#[doc = "Register `REG_WMAC_RTX_CTX_SUBTYPE` reader"]
pub type R = crate::R<RegWmacRtxCtxSubtypeSpec>;
#[doc = "Register `REG_WMAC_RTX_CTX_SUBTYPE` writer"]
pub type W = crate::W<RegWmacRtxCtxSubtypeSpec>;
#[doc = "Field `BIT_r_WMAC_RTX_SUBTYPE` reader - "]
pub type BitRWmacRtxSubtypeR = crate::FieldReader;
#[doc = "Field `BIT_r_WMAC_RTX_SUBTYPE` writer - "]
pub type BitRWmacRtxSubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_r_WMAC_CTX_SUBTYPE` reader - "]
pub type BitRWmacCtxSubtypeR = crate::FieldReader;
#[doc = "Field `BIT_r_WMAC_CTX_SUBTYPE` writer - "]
pub type BitRWmacCtxSubtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bit_r_wmac_rtx_subtype(&self) -> BitRWmacRtxSubtypeR {
        BitRWmacRtxSubtypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn bit_r_wmac_ctx_subtype(&self) -> BitRWmacCtxSubtypeR {
        BitRWmacCtxSubtypeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bit_r_wmac_rtx_subtype(&mut self) -> BitRWmacRtxSubtypeW<'_, RegWmacRtxCtxSubtypeSpec> {
        BitRWmacRtxSubtypeW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn bit_r_wmac_ctx_subtype(&mut self) -> BitRWmacCtxSubtypeW<'_, RegWmacRtxCtxSubtypeSpec> {
        BitRWmacCtxSubtypeW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_rtx_ctx_subtype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_rtx_ctx_subtype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacRtxCtxSubtypeSpec;
impl crate::RegisterSpec for RegWmacRtxCtxSubtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_rtx_ctx_subtype::R`](R) reader structure"]
impl crate::Readable for RegWmacRtxCtxSubtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_rtx_ctx_subtype::W`](W) writer structure"]
impl crate::Writable for RegWmacRtxCtxSubtypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_RTX_CTX_SUBTYPE to value 0x76"]
impl crate::Resettable for RegWmacRtxCtxSubtypeSpec {
    const RESET_VALUE: u32 = 0x76;
}
