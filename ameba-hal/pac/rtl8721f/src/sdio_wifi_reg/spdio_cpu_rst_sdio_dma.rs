#[doc = "Register `SPDIO_CPU_RST_SDIO_DMA` reader"]
pub type R = crate::R<SpdioCpuRstSdioDmaSpec>;
#[doc = "Register `SPDIO_CPU_RST_SDIO_DMA` writer"]
pub type W = crate::W<SpdioCpuRstSdioDmaSpec>;
#[doc = "Field `AVAL_PG_INT_OPT` reader - set 1 to disable clearing tx_aval_pg_int by read tx_aval_pg_num"]
pub type AvalPgIntOptR = crate::BitReader;
#[doc = "Field `AVAL_PG_INT_OPT` writer - set 1 to disable clearing tx_aval_pg_int by read tx_aval_pg_num"]
pub type AvalPgIntOptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_RST_SDIO_DMA` reader - CPU set this bit to reset SDIO DMA. This bit is auto clear."]
pub type CpuRstSdioDmaR = crate::BitReader;
#[doc = "Field `CPU_RST_SDIO_DMA` writer - CPU set this bit to reset SDIO DMA. This bit is auto clear."]
pub type CpuRstSdioDmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set 1 to disable clearing tx_aval_pg_int by read tx_aval_pg_num"]
    #[inline(always)]
    pub fn aval_pg_int_opt(&self) -> AvalPgIntOptR {
        AvalPgIntOptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - CPU set this bit to reset SDIO DMA. This bit is auto clear."]
    #[inline(always)]
    pub fn cpu_rst_sdio_dma(&self) -> CpuRstSdioDmaR {
        CpuRstSdioDmaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set 1 to disable clearing tx_aval_pg_int by read tx_aval_pg_num"]
    #[inline(always)]
    pub fn aval_pg_int_opt(&mut self) -> AvalPgIntOptW<'_, SpdioCpuRstSdioDmaSpec> {
        AvalPgIntOptW::new(self, 0)
    }
    #[doc = "Bit 7 - CPU set this bit to reset SDIO DMA. This bit is auto clear."]
    #[inline(always)]
    pub fn cpu_rst_sdio_dma(&mut self) -> CpuRstSdioDmaW<'_, SpdioCpuRstSdioDmaSpec> {
        CpuRstSdioDmaW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_cpu_rst_sdio_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_cpu_rst_sdio_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCpuRstSdioDmaSpec;
impl crate::RegisterSpec for SpdioCpuRstSdioDmaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spdio_cpu_rst_sdio_dma::R`](R) reader structure"]
impl crate::Readable for SpdioCpuRstSdioDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_cpu_rst_sdio_dma::W`](W) writer structure"]
impl crate::Writable for SpdioCpuRstSdioDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_CPU_RST_SDIO_DMA to value 0"]
impl crate::Resettable for SpdioCpuRstSdioDmaSpec {}
