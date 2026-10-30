#[doc = "Register `PSRAM_ARB_W_CTRL1` reader"]
pub type R = crate::R<PsramArbWCtrl1Spec>;
#[doc = "Register `PSRAM_ARB_W_CTRL1` writer"]
pub type W = crate::W<PsramArbWCtrl1Spec>;
#[doc = "Field `W_CTRL1_PSRAM_FINAL_ARB` reader - Write arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0/CPU1 0x1: HPERI_TO_PSRAM_ARB group 0x2: WIFI 0x3: USB 0x4: MJPEG 0x5: LCDC 0x6: PPE And the following shows bit field of each slots: \\[2:0\\]: slot 0 of arbiter \\[5:3\\]: slot 1 of arbiter \\[8:6\\]: slot 2 of arbiter \\[11:9\\]: slot 3 of arbiter \\[14:12\\]: slot 4 of arbiter \\[17:15\\]: slot 5 of arbiter \\[20:18\\]: slot 6 of arbiter \\[23:21\\]: slot 7 of arbiter \\[26:24\\]: slot 8 of arbiter \\[29:27\\]: slot 9 of arbiter"]
pub type WCtrl1PsramFinalArbR = crate::FieldReader<u32>;
#[doc = "Field `W_CTRL1_PSRAM_FINAL_ARB` writer - Write arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0/CPU1 0x1: HPERI_TO_PSRAM_ARB group 0x2: WIFI 0x3: USB 0x4: MJPEG 0x5: LCDC 0x6: PPE And the following shows bit field of each slots: \\[2:0\\]: slot 0 of arbiter \\[5:3\\]: slot 1 of arbiter \\[8:6\\]: slot 2 of arbiter \\[11:9\\]: slot 3 of arbiter \\[14:12\\]: slot 4 of arbiter \\[17:15\\]: slot 5 of arbiter \\[20:18\\]: slot 6 of arbiter \\[23:21\\]: slot 7 of arbiter \\[26:24\\]: slot 8 of arbiter \\[29:27\\]: slot 9 of arbiter"]
pub type WCtrl1PsramFinalArbW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Write arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0/CPU1 0x1: HPERI_TO_PSRAM_ARB group 0x2: WIFI 0x3: USB 0x4: MJPEG 0x5: LCDC 0x6: PPE And the following shows bit field of each slots: \\[2:0\\]: slot 0 of arbiter \\[5:3\\]: slot 1 of arbiter \\[8:6\\]: slot 2 of arbiter \\[11:9\\]: slot 3 of arbiter \\[14:12\\]: slot 4 of arbiter \\[17:15\\]: slot 5 of arbiter \\[20:18\\]: slot 6 of arbiter \\[23:21\\]: slot 7 of arbiter \\[26:24\\]: slot 8 of arbiter \\[29:27\\]: slot 9 of arbiter"]
    #[inline(always)]
    pub fn w_ctrl1_psram_final_arb(&self) -> WCtrl1PsramFinalArbR {
        WCtrl1PsramFinalArbR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Write arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0/CPU1 0x1: HPERI_TO_PSRAM_ARB group 0x2: WIFI 0x3: USB 0x4: MJPEG 0x5: LCDC 0x6: PPE And the following shows bit field of each slots: \\[2:0\\]: slot 0 of arbiter \\[5:3\\]: slot 1 of arbiter \\[8:6\\]: slot 2 of arbiter \\[11:9\\]: slot 3 of arbiter \\[14:12\\]: slot 4 of arbiter \\[17:15\\]: slot 5 of arbiter \\[20:18\\]: slot 6 of arbiter \\[23:21\\]: slot 7 of arbiter \\[26:24\\]: slot 8 of arbiter \\[29:27\\]: slot 9 of arbiter"]
    #[inline(always)]
    pub fn w_ctrl1_psram_final_arb(&mut self) -> WCtrl1PsramFinalArbW<'_, PsramArbWCtrl1Spec> {
        WCtrl1PsramFinalArbW::new(self, 0)
    }
}
#[doc = "The register for PSRAM write arbitration\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_arb_w_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_arb_w_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramArbWCtrl1Spec;
impl crate::RegisterSpec for PsramArbWCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_arb_w_ctrl1::R`](R) reader structure"]
impl crate::Readable for PsramArbWCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_arb_w_ctrl1::W`](W) writer structure"]
impl crate::Writable for PsramArbWCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ARB_W_CTRL1 to value 0x1a48_9200"]
impl crate::Resettable for PsramArbWCtrl1Spec {
    const RESET_VALUE: u32 = 0x1a48_9200;
}
