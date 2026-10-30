#[doc = "Register `CLK_DET_PLLSDIO0` reader"]
pub type R = crate::R<ClkDetPllsdio0Spec>;
#[doc = "Register `CLK_DET_PLLSDIO0` writer"]
pub type W = crate::W<ClkDetPllsdio0Spec>;
#[doc = "Field `ssc_usable_sdio` reader - (Reserved)"]
pub type SscUsableSdioR = crate::BitReader;
#[doc = "Field `RSVD` writer - -"]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `refclk_count_pllsdio` reader - (Reserved)"]
pub type RefclkCountPllsdioR = crate::FieldReader<u16>;
#[doc = "Field `clk_count_pllsdio` reader - (Reserved)"]
pub type ClkCountPllsdioR = crate::FieldReader<u32>;
#[doc = "Field `clkdet_done_pllsdio` reader - (Reserved)"]
pub type ClkdetDonePllsdioR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - (Reserved)"]
    #[inline(always)]
    pub fn ssc_usable_sdio(&self) -> SscUsableSdioR {
        SscUsableSdioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:12 - (Reserved)"]
    #[inline(always)]
    pub fn refclk_count_pllsdio(&self) -> RefclkCountPllsdioR {
        RefclkCountPllsdioR::new(((self.bits >> 2) & 0x07ff) as u16)
    }
    #[doc = "Bits 13:29 - (Reserved)"]
    #[inline(always)]
    pub fn clk_count_pllsdio(&self) -> ClkCountPllsdioR {
        ClkCountPllsdioR::new((self.bits >> 13) & 0x0001_ffff)
    }
    #[doc = "Bit 30 - (Reserved)"]
    #[inline(always)]
    pub fn clkdet_done_pllsdio(&self) -> ClkdetDonePllsdioR {
        ClkdetDonePllsdioR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - -"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, ClkDetPllsdio0Spec> {
        RsvdW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_pllsdio0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_det_pllsdio0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDetPllsdio0Spec;
impl crate::RegisterSpec for ClkDetPllsdio0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_det_pllsdio0::R`](R) reader structure"]
impl crate::Readable for ClkDetPllsdio0Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_det_pllsdio0::W`](W) writer structure"]
impl crate::Writable for ClkDetPllsdio0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_DET_PLLSDIO0 to value 0"]
impl crate::Resettable for ClkDetPllsdio0Spec {}
