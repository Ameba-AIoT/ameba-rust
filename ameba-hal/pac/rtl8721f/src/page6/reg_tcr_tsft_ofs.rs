#[doc = "Register `REG_TCR_TSFT_OFS` reader"]
pub type R = crate::R<RegTcrTsftOfsSpec>;
#[doc = "Register `REG_TCR_TSFT_OFS` writer"]
pub type W = crate::W<RegTcrTsftOfsSpec>;
#[doc = "Field `r_wmac_tcr_tsft_ns_ofs` reader - TCR TSFT NS OFS"]
pub type RWmacTcrTsftNsOfsR = crate::FieldReader<u32>;
#[doc = "Field `r_wmac_tcr_tsft_ns_ofs` writer - TCR TSFT NS OFS"]
pub type RWmacTcrTsftNsOfsW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - TCR TSFT NS OFS"]
    #[inline(always)]
    pub fn r_wmac_tcr_tsft_ns_ofs(&self) -> RWmacTcrTsftNsOfsR {
        RWmacTcrTsftNsOfsR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - TCR TSFT NS OFS"]
    #[inline(always)]
    pub fn r_wmac_tcr_tsft_ns_ofs(&mut self) -> RWmacTcrTsftNsOfsW<'_, RegTcrTsftOfsSpec> {
        RWmacTcrTsftNsOfsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tcr_tsft_ofs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tcr_tsft_ofs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTcrTsftOfsSpec;
impl crate::RegisterSpec for RegTcrTsftOfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tcr_tsft_ofs::R`](R) reader structure"]
impl crate::Readable for RegTcrTsftOfsSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tcr_tsft_ofs::W`](W) writer structure"]
impl crate::Writable for RegTcrTsftOfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TCR_TSFT_OFS to value 0"]
impl crate::Resettable for RegTcrTsftOfsSpec {}
