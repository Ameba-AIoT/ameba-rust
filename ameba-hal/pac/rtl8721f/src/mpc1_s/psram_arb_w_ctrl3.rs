#[doc = "Register `PSRAM_ARB_W_CTRL3` reader"]
pub type R = crate::R<PsramArbWCtrl3Spec>;
#[doc = "Register `PSRAM_ARB_W_CTRL3` writer"]
pub type W = crate::W<PsramArbWCtrl3Spec>;
#[doc = "Field `W_CTRL3_PSRAM_FINAL_ARB` reader - This is extended register of PSRAM_FINAL_ARB slot. \\[2:0\\]: slot 20 of arbiter \\[5:3\\]: slot 21 of arbiter \\[8:6\\]: slot 22 of arbiter \\[11:9\\]: slot 23 of arbiter"]
pub type WCtrl3PsramFinalArbR = crate::FieldReader<u16>;
#[doc = "Field `W_CTRL3_PSRAM_FINAL_ARB` writer - This is extended register of PSRAM_FINAL_ARB slot. \\[2:0\\]: slot 20 of arbiter \\[5:3\\]: slot 21 of arbiter \\[8:6\\]: slot 22 of arbiter \\[11:9\\]: slot 23 of arbiter"]
pub type WCtrl3PsramFinalArbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - This is extended register of PSRAM_FINAL_ARB slot. \\[2:0\\]: slot 20 of arbiter \\[5:3\\]: slot 21 of arbiter \\[8:6\\]: slot 22 of arbiter \\[11:9\\]: slot 23 of arbiter"]
    #[inline(always)]
    pub fn w_ctrl3_psram_final_arb(&self) -> WCtrl3PsramFinalArbR {
        WCtrl3PsramFinalArbR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This is extended register of PSRAM_FINAL_ARB slot. \\[2:0\\]: slot 20 of arbiter \\[5:3\\]: slot 21 of arbiter \\[8:6\\]: slot 22 of arbiter \\[11:9\\]: slot 23 of arbiter"]
    #[inline(always)]
    pub fn w_ctrl3_psram_final_arb(&mut self) -> WCtrl3PsramFinalArbW<'_, PsramArbWCtrl3Spec> {
        WCtrl3PsramFinalArbW::new(self, 0)
    }
}
#[doc = "The register for PSRAM write arbitration\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_arb_w_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_arb_w_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramArbWCtrl3Spec;
impl crate::RegisterSpec for PsramArbWCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_arb_w_ctrl3::R`](R) reader structure"]
impl crate::Readable for PsramArbWCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_arb_w_ctrl3::W`](W) writer structure"]
impl crate::Writable for PsramArbWCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ARB_W_CTRL3 to value 0x06"]
impl crate::Resettable for PsramArbWCtrl3Spec {
    const RESET_VALUE: u32 = 0x06;
}
