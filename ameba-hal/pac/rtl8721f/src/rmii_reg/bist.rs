#[doc = "Register `BIST` reader"]
pub type R = crate::R<BistSpec>;
#[doc = "Register `BIST` writer"]
pub type W = crate::W<BistSpec>;
#[doc = "Field `csr_tff_dvs` reader - Set DVS for TXFIFO"]
pub type CsrTffDvsR = crate::FieldReader;
#[doc = "Field `csr_tff_dvs` writer - Set DVS for TXFIFO"]
pub type CsrTffDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `csr_tff_dvse` reader - Set DVSE for TXFIFO"]
pub type CsrTffDvseR = crate::BitReader;
#[doc = "Field `csr_tff_dvse` writer - Set DVSE for TXFIFO"]
pub type CsrTffDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `csr_tff_nap` reader - Set NAP for TXFIFO"]
pub type CsrTffNapR = crate::BitReader;
#[doc = "Field `csr_tff_nap` writer - Set NAP for TXFIFO"]
pub type CsrTffNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `csr_tff_tmck` reader - Set TMCK for TXFIFO"]
pub type CsrTffTmckR = crate::BitReader;
#[doc = "Field `csr_tff_tmck` writer - Set TMCK for TXFIFO"]
pub type CsrTffTmckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `csr_rff_dvs` reader - Set DVS for RXFIFO"]
pub type CsrRffDvsR = crate::FieldReader;
#[doc = "Field `csr_rff_dvs` writer - Set DVS for RXFIFO"]
pub type CsrRffDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `csr_rff_dvse` reader - Set DVSE for RXFIFO"]
pub type CsrRffDvseR = crate::BitReader;
#[doc = "Field `csr_rff_dvse` writer - Set DVSE for RXFIFO"]
pub type CsrRffDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `csr_rff_nap` reader - Set NAP for RXFIFO"]
pub type CsrRffNapR = crate::BitReader;
#[doc = "Field `csr_rff_nap` writer - Set NAP for RXFIFO"]
pub type CsrRffNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `csr_rff_tmck` reader - Set TMCK for RXFIFO"]
pub type CsrRffTmckR = crate::BitReader;
#[doc = "Field `csr_rff_tmck` writer - Set TMCK for RXFIFO"]
pub type CsrRffTmckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Set DVS for TXFIFO"]
    #[inline(always)]
    pub fn csr_tff_dvs(&self) -> CsrTffDvsR {
        CsrTffDvsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Set DVSE for TXFIFO"]
    #[inline(always)]
    pub fn csr_tff_dvse(&self) -> CsrTffDvseR {
        CsrTffDvseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set NAP for TXFIFO"]
    #[inline(always)]
    pub fn csr_tff_nap(&self) -> CsrTffNapR {
        CsrTffNapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set TMCK for TXFIFO"]
    #[inline(always)]
    pub fn csr_tff_tmck(&self) -> CsrTffTmckR {
        CsrTffTmckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Set DVS for RXFIFO"]
    #[inline(always)]
    pub fn csr_rff_dvs(&self) -> CsrRffDvsR {
        CsrRffDvsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Set DVSE for RXFIFO"]
    #[inline(always)]
    pub fn csr_rff_dvse(&self) -> CsrRffDvseR {
        CsrRffDvseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set NAP for RXFIFO"]
    #[inline(always)]
    pub fn csr_rff_nap(&self) -> CsrRffNapR {
        CsrRffNapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set TMCK for RXFIFO"]
    #[inline(always)]
    pub fn csr_rff_tmck(&self) -> CsrRffTmckR {
        CsrRffTmckR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set DVS for TXFIFO"]
    #[inline(always)]
    pub fn csr_tff_dvs(&mut self) -> CsrTffDvsW<'_, BistSpec> {
        CsrTffDvsW::new(self, 0)
    }
    #[doc = "Bit 4 - Set DVSE for TXFIFO"]
    #[inline(always)]
    pub fn csr_tff_dvse(&mut self) -> CsrTffDvseW<'_, BistSpec> {
        CsrTffDvseW::new(self, 4)
    }
    #[doc = "Bit 5 - Set NAP for TXFIFO"]
    #[inline(always)]
    pub fn csr_tff_nap(&mut self) -> CsrTffNapW<'_, BistSpec> {
        CsrTffNapW::new(self, 5)
    }
    #[doc = "Bit 6 - Set TMCK for TXFIFO"]
    #[inline(always)]
    pub fn csr_tff_tmck(&mut self) -> CsrTffTmckW<'_, BistSpec> {
        CsrTffTmckW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Set DVS for RXFIFO"]
    #[inline(always)]
    pub fn csr_rff_dvs(&mut self) -> CsrRffDvsW<'_, BistSpec> {
        CsrRffDvsW::new(self, 8)
    }
    #[doc = "Bit 12 - Set DVSE for RXFIFO"]
    #[inline(always)]
    pub fn csr_rff_dvse(&mut self) -> CsrRffDvseW<'_, BistSpec> {
        CsrRffDvseW::new(self, 12)
    }
    #[doc = "Bit 13 - Set NAP for RXFIFO"]
    #[inline(always)]
    pub fn csr_rff_nap(&mut self) -> CsrRffNapW<'_, BistSpec> {
        CsrRffNapW::new(self, 13)
    }
    #[doc = "Bit 14 - Set TMCK for RXFIFO"]
    #[inline(always)]
    pub fn csr_rff_tmck(&mut self) -> CsrRffTmckW<'_, BistSpec> {
        CsrRffTmckW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bist::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bist::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BistSpec;
impl crate::RegisterSpec for BistSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bist::R`](R) reader structure"]
impl crate::Readable for BistSpec {}
#[doc = "`write(|w| ..)` method takes [`bist::W`](W) writer structure"]
impl crate::Writable for BistSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIST to value 0"]
impl crate::Resettable for BistSpec {}
