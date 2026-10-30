#[doc = "Register `PSRAM_ARB_R_CTRL3_RD_WEIGHT` reader"]
pub type R = crate::R<PsramArbRCtrl3RdWeightSpec>;
#[doc = "Register `PSRAM_ARB_R_CTRL3_RD_WEIGHT` writer"]
pub type W = crate::W<PsramArbRCtrl3RdWeightSpec>;
#[doc = "Field `PSRAM_ARB_CTRL1` reader - This is extended register of PSRAM_ARB_CTRL0 slot. \\[3:0\\]: slot 8 of arbiter \\[7:4\\]: slot 9 of arbiter \\[11:8\\]: slot 10 of arbiter \\[15:12\\]: slot 11 of arbiter \\[19:16\\]: slot 12 of arbiter \\[23:20\\]: slot 13 of arbiter \\[27:24\\]: slot 14 of arbiter \\[31:28\\]: slot 15 of arbiter"]
pub type PsramArbCtrl1R = crate::FieldReader<u16>;
#[doc = "Field `PSRAM_ARB_CTRL1` writer - This is extended register of PSRAM_ARB_CTRL0 slot. \\[3:0\\]: slot 8 of arbiter \\[7:4\\]: slot 9 of arbiter \\[11:8\\]: slot 10 of arbiter \\[15:12\\]: slot 11 of arbiter \\[19:16\\]: slot 12 of arbiter \\[23:20\\]: slot 13 of arbiter \\[27:24\\]: slot 14 of arbiter \\[31:28\\]: slot 15 of arbiter"]
pub type PsramArbCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RD_WEIGHT_SRAM` reader - Read CMD weight of hs_sram0"]
pub type RdWeightSramR = crate::FieldReader<u16>;
#[doc = "Field `RD_WEIGHT_SRAM` writer - Read CMD weight of hs_sram0"]
pub type RdWeightSramW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RD_WEIGHT_BUFFER` reader - Read CMD weight of buffer_ram"]
pub type RdWeightBufferR = crate::FieldReader<u16>;
#[doc = "Field `RD_WEIGHT_BUFFER` writer - Read CMD weight of buffer_ram"]
pub type RdWeightBufferW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:11 - This is extended register of PSRAM_ARB_CTRL0 slot. \\[3:0\\]: slot 8 of arbiter \\[7:4\\]: slot 9 of arbiter \\[11:8\\]: slot 10 of arbiter \\[15:12\\]: slot 11 of arbiter \\[19:16\\]: slot 12 of arbiter \\[23:20\\]: slot 13 of arbiter \\[27:24\\]: slot 14 of arbiter \\[31:28\\]: slot 15 of arbiter"]
    #[inline(always)]
    pub fn psram_arb_ctrl1(&self) -> PsramArbCtrl1R {
        PsramArbCtrl1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:20 - Read CMD weight of hs_sram0"]
    #[inline(always)]
    pub fn rd_weight_sram(&self) -> RdWeightSramR {
        RdWeightSramR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 21:29 - Read CMD weight of buffer_ram"]
    #[inline(always)]
    pub fn rd_weight_buffer(&self) -> RdWeightBufferR {
        RdWeightBufferR::new(((self.bits >> 21) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This is extended register of PSRAM_ARB_CTRL0 slot. \\[3:0\\]: slot 8 of arbiter \\[7:4\\]: slot 9 of arbiter \\[11:8\\]: slot 10 of arbiter \\[15:12\\]: slot 11 of arbiter \\[19:16\\]: slot 12 of arbiter \\[23:20\\]: slot 13 of arbiter \\[27:24\\]: slot 14 of arbiter \\[31:28\\]: slot 15 of arbiter"]
    #[inline(always)]
    pub fn psram_arb_ctrl1(&mut self) -> PsramArbCtrl1W<'_, PsramArbRCtrl3RdWeightSpec> {
        PsramArbCtrl1W::new(self, 0)
    }
    #[doc = "Bits 12:20 - Read CMD weight of hs_sram0"]
    #[inline(always)]
    pub fn rd_weight_sram(&mut self) -> RdWeightSramW<'_, PsramArbRCtrl3RdWeightSpec> {
        RdWeightSramW::new(self, 12)
    }
    #[doc = "Bits 21:29 - Read CMD weight of buffer_ram"]
    #[inline(always)]
    pub fn rd_weight_buffer(&mut self) -> RdWeightBufferW<'_, PsramArbRCtrl3RdWeightSpec> {
        RdWeightBufferW::new(self, 21)
    }
}
#[doc = "The register for PSRAM read arbitration and weight control of SRAM controller\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_arb_r_ctrl3_rd_weight::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_arb_r_ctrl3_rd_weight::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramArbRCtrl3RdWeightSpec;
impl crate::RegisterSpec for PsramArbRCtrl3RdWeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_arb_r_ctrl3_rd_weight::R`](R) reader structure"]
impl crate::Readable for PsramArbRCtrl3RdWeightSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_arb_r_ctrl3_rd_weight::W`](W) writer structure"]
impl crate::Writable for PsramArbRCtrl3RdWeightSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ARB_R_CTRL3_RD_WEIGHT to value 0x0080_2006"]
impl crate::Resettable for PsramArbRCtrl3RdWeightSpec {
    const RESET_VALUE: u32 = 0x0080_2006;
}
