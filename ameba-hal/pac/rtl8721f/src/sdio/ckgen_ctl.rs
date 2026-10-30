#[doc = "Register `CKGEN_CTL` reader"]
pub type R = crate::R<CkgenCtlSpec>;
#[doc = "Register `CKGEN_CTL` writer"]
pub type W = crate::W<CkgenCtlSpec>;
#[doc = "Field `crc_clk_change` reader - 0: from crc_clk_src 1: clk"]
pub type CrcClkChangeR = crate::BitReader;
#[doc = "Field `crc_clk_change` writer - 0: from crc_clk_src 1: clk"]
pub type CrcClkChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd30_push_change` reader - 0: from sd30_push_clk_src 1: clk"]
pub type Sd30PushChangeR = crate::BitReader;
#[doc = "Field `sd30_push_change` writer - 0: from sd30_push_clk_src 1: clk"]
pub type Sd30PushChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd30_sample_change` reader - 0: from sd30_sample_clk_src 1: clk"]
pub type Sd30SampleChangeR = crate::BitReader;
#[doc = "Field `sd30_sample_change` writer - 0: from sd30_sample_clk_src 1: clk"]
pub type Sd30SampleChangeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - 0: from crc_clk_src 1: clk"]
    #[inline(always)]
    pub fn crc_clk_change(&self) -> CrcClkChangeR {
        CrcClkChangeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: from sd30_push_clk_src 1: clk"]
    #[inline(always)]
    pub fn sd30_push_change(&self) -> Sd30PushChangeR {
        Sd30PushChangeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: from sd30_sample_clk_src 1: clk"]
    #[inline(always)]
    pub fn sd30_sample_change(&self) -> Sd30SampleChangeR {
        Sd30SampleChangeR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - 0: from crc_clk_src 1: clk"]
    #[inline(always)]
    pub fn crc_clk_change(&mut self) -> CrcClkChangeW<'_, CkgenCtlSpec> {
        CrcClkChangeW::new(self, 16)
    }
    #[doc = "Bit 17 - 0: from sd30_push_clk_src 1: clk"]
    #[inline(always)]
    pub fn sd30_push_change(&mut self) -> Sd30PushChangeW<'_, CkgenCtlSpec> {
        Sd30PushChangeW::new(self, 17)
    }
    #[doc = "Bit 18 - 0: from sd30_sample_clk_src 1: clk"]
    #[inline(always)]
    pub fn sd30_sample_change(&mut self) -> Sd30SampleChangeW<'_, CkgenCtlSpec> {
        Sd30SampleChangeW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgen_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgen_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgenCtlSpec;
impl crate::RegisterSpec for CkgenCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgen_ctl::R`](R) reader structure"]
impl crate::Readable for CkgenCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgen_ctl::W`](W) writer structure"]
impl crate::Writable for CkgenCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKGEN_CTL to value 0"]
impl crate::Resettable for CkgenCtlSpec {}
