#[doc = "Register `PSRAM_ARB_W_CTRL2` reader"]
pub type R = crate::R<PsramArbWCtrl2Spec>;
#[doc = "Register `PSRAM_ARB_W_CTRL2` writer"]
pub type W = crate::W<PsramArbWCtrl2Spec>;
#[doc = "Field `W_CTRL2_PSRAM_FINAL_ARB` reader - This is extended register of PSRAM_FINAL_ARB slot \\[2:0\\]: slot 10 of arbiter \\[5:3\\]: slot 11 of arbiter \\[8:6\\]: slot 12 of arbiter \\[11:9\\]: slot 13 of arbiter \\[14:12\\]: slot 14 of arbiter \\[17:15\\]: slot 15 of arbiter \\[20:18\\]: slot 16 of arbiter \\[23:21\\]: slot 17 of arbiter \\[26:24\\]: slot 18 of arbiter \\[29:27\\]: slot 19 of arbiter"]
pub type WCtrl2PsramFinalArbR = crate::FieldReader<u32>;
#[doc = "Field `W_CTRL2_PSRAM_FINAL_ARB` writer - This is extended register of PSRAM_FINAL_ARB slot \\[2:0\\]: slot 10 of arbiter \\[5:3\\]: slot 11 of arbiter \\[8:6\\]: slot 12 of arbiter \\[11:9\\]: slot 13 of arbiter \\[14:12\\]: slot 14 of arbiter \\[17:15\\]: slot 15 of arbiter \\[20:18\\]: slot 16 of arbiter \\[23:21\\]: slot 17 of arbiter \\[26:24\\]: slot 18 of arbiter \\[29:27\\]: slot 19 of arbiter"]
pub type WCtrl2PsramFinalArbW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - This is extended register of PSRAM_FINAL_ARB slot \\[2:0\\]: slot 10 of arbiter \\[5:3\\]: slot 11 of arbiter \\[8:6\\]: slot 12 of arbiter \\[11:9\\]: slot 13 of arbiter \\[14:12\\]: slot 14 of arbiter \\[17:15\\]: slot 15 of arbiter \\[20:18\\]: slot 16 of arbiter \\[23:21\\]: slot 17 of arbiter \\[26:24\\]: slot 18 of arbiter \\[29:27\\]: slot 19 of arbiter"]
    #[inline(always)]
    pub fn w_ctrl2_psram_final_arb(&self) -> WCtrl2PsramFinalArbR {
        WCtrl2PsramFinalArbR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - This is extended register of PSRAM_FINAL_ARB slot \\[2:0\\]: slot 10 of arbiter \\[5:3\\]: slot 11 of arbiter \\[8:6\\]: slot 12 of arbiter \\[11:9\\]: slot 13 of arbiter \\[14:12\\]: slot 14 of arbiter \\[17:15\\]: slot 15 of arbiter \\[20:18\\]: slot 16 of arbiter \\[23:21\\]: slot 17 of arbiter \\[26:24\\]: slot 18 of arbiter \\[29:27\\]: slot 19 of arbiter"]
    #[inline(always)]
    pub fn w_ctrl2_psram_final_arb(&mut self) -> WCtrl2PsramFinalArbW<'_, PsramArbWCtrl2Spec> {
        WCtrl2PsramFinalArbW::new(self, 0)
    }
}
#[doc = "The register for PSRAM write arbitration\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_arb_w_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_arb_w_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramArbWCtrl2Spec;
impl crate::RegisterSpec for PsramArbWCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_arb_w_ctrl2::R`](R) reader structure"]
impl crate::Readable for PsramArbWCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_arb_w_ctrl2::W`](W) writer structure"]
impl crate::Writable for PsramArbWCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ARB_W_CTRL2 to value 0x36b6_c91b"]
impl crate::Resettable for PsramArbWCtrl2Spec {
    const RESET_VALUE: u32 = 0x36b6_c91b;
}
