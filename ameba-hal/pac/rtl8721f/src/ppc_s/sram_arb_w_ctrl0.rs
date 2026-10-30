#[doc = "Register `SRAM_ARB_W_CTRL0` reader"]
pub type R = crate::R<SramArbWCtrl0Spec>;
#[doc = "Register `SRAM_ARB_W_CTRL0` writer"]
pub type W = crate::W<SramArbWCtrl0Spec>;
#[doc = "Field `32B_HPERI_TO_SRAM_ARB` reader - Write arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: GDMA 0x1: WIFI 0x2: USB/SDIO_BT/SDIO_WIFI 0x3: OTPC/SIC 0x4: SDIO_HOST 0x5: RMII And the following shows bit field of each slots: \\[2:0\\]: slot 0 of arbiter \\[5:3\\]: slot 1 of arbiter \\[8:6\\]: slot 2 of arbiter \\[11:9\\]: slot 3 of arbiter \\[14:12\\]: slot 4 of arbiter \\[17:15\\]: slot 5 of arbiter \\[20:18\\]: slot 6 of arbiter \\[23:21\\]: slot 7 of arbiter \\[26:24\\]: slot 8 of arbiter \\[29:27\\]: slot 9 of arbiter"]
pub type _32bHperiToSramArbR = crate::FieldReader<u32>;
#[doc = "Field `32B_HPERI_TO_SRAM_ARB` writer - Write arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: GDMA 0x1: WIFI 0x2: USB/SDIO_BT/SDIO_WIFI 0x3: OTPC/SIC 0x4: SDIO_HOST 0x5: RMII And the following shows bit field of each slots: \\[2:0\\]: slot 0 of arbiter \\[5:3\\]: slot 1 of arbiter \\[8:6\\]: slot 2 of arbiter \\[11:9\\]: slot 3 of arbiter \\[14:12\\]: slot 4 of arbiter \\[17:15\\]: slot 5 of arbiter \\[20:18\\]: slot 6 of arbiter \\[23:21\\]: slot 7 of arbiter \\[26:24\\]: slot 8 of arbiter \\[29:27\\]: slot 9 of arbiter"]
pub type _32bHperiToSramArbW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Write arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: GDMA 0x1: WIFI 0x2: USB/SDIO_BT/SDIO_WIFI 0x3: OTPC/SIC 0x4: SDIO_HOST 0x5: RMII And the following shows bit field of each slots: \\[2:0\\]: slot 0 of arbiter \\[5:3\\]: slot 1 of arbiter \\[8:6\\]: slot 2 of arbiter \\[11:9\\]: slot 3 of arbiter \\[14:12\\]: slot 4 of arbiter \\[17:15\\]: slot 5 of arbiter \\[20:18\\]: slot 6 of arbiter \\[23:21\\]: slot 7 of arbiter \\[26:24\\]: slot 8 of arbiter \\[29:27\\]: slot 9 of arbiter"]
    #[inline(always)]
    pub fn _32b_hperi_to_sram_arb(&self) -> _32bHperiToSramArbR {
        _32bHperiToSramArbR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Write arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: GDMA 0x1: WIFI 0x2: USB/SDIO_BT/SDIO_WIFI 0x3: OTPC/SIC 0x4: SDIO_HOST 0x5: RMII And the following shows bit field of each slots: \\[2:0\\]: slot 0 of arbiter \\[5:3\\]: slot 1 of arbiter \\[8:6\\]: slot 2 of arbiter \\[11:9\\]: slot 3 of arbiter \\[14:12\\]: slot 4 of arbiter \\[17:15\\]: slot 5 of arbiter \\[20:18\\]: slot 6 of arbiter \\[23:21\\]: slot 7 of arbiter \\[26:24\\]: slot 8 of arbiter \\[29:27\\]: slot 9 of arbiter"]
    #[inline(always)]
    pub fn _32b_hperi_to_sram_arb(&mut self) -> _32bHperiToSramArbW<'_, SramArbWCtrl0Spec> {
        _32bHperiToSramArbW::new(self, 0)
    }
}
#[doc = "The register 32-bit HPERI master group to SRAM write arbitration\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_arb_w_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_arb_w_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramArbWCtrl0Spec;
impl crate::RegisterSpec for SramArbWCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_arb_w_ctrl0::R`](R) reader structure"]
impl crate::Readable for SramArbWCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_arb_w_ctrl0::W`](W) writer structure"]
impl crate::Writable for SramArbWCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_ARB_W_CTRL0 to value 0x2d91_a240"]
impl crate::Resettable for SramArbWCtrl0Spec {
    const RESET_VALUE: u32 = 0x2d91_a240;
}
