#[doc = "Register `CLK_DET_PLLSDIO1` reader"]
pub type R = crate::R<ClkDetPllsdio1Spec>;
#[doc = "Register `CLK_DET_PLLSDIO1` writer"]
pub type W = crate::W<ClkDetPllsdio1Spec>;
#[doc = "Field `reg_rstn_pllsdio` reader - (Reserved)"]
pub type RegRstnPllsdioR = crate::BitReader;
#[doc = "Field `reg_rstn_pllsdio` writer - (Reserved)"]
pub type RegRstnPllsdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_count_en_pllsdio` reader - (Reserved)"]
pub type RegCountEnPllsdioR = crate::BitReader;
#[doc = "Field `reg_count_en_pllsdio` writer - (Reserved)"]
pub type RegCountEnPllsdioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (Reserved)"]
    #[inline(always)]
    pub fn reg_rstn_pllsdio(&self) -> RegRstnPllsdioR {
        RegRstnPllsdioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (Reserved)"]
    #[inline(always)]
    pub fn reg_count_en_pllsdio(&self) -> RegCountEnPllsdioR {
        RegCountEnPllsdioR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (Reserved)"]
    #[inline(always)]
    pub fn reg_rstn_pllsdio(&mut self) -> RegRstnPllsdioW<'_, ClkDetPllsdio1Spec> {
        RegRstnPllsdioW::new(self, 0)
    }
    #[doc = "Bit 1 - (Reserved)"]
    #[inline(always)]
    pub fn reg_count_en_pllsdio(&mut self) -> RegCountEnPllsdioW<'_, ClkDetPllsdio1Spec> {
        RegCountEnPllsdioW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_det_pllsdio1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_det_pllsdio1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDetPllsdio1Spec;
impl crate::RegisterSpec for ClkDetPllsdio1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_det_pllsdio1::R`](R) reader structure"]
impl crate::Readable for ClkDetPllsdio1Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_det_pllsdio1::W`](W) writer structure"]
impl crate::Writable for ClkDetPllsdio1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_DET_PLLSDIO1 to value 0"]
impl crate::Resettable for ClkDetPllsdio1Spec {}
