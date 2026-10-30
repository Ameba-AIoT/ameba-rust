#[doc = "Register `TPR2` reader"]
pub type R = crate::R<Tpr2Spec>;
#[doc = "Register `TPR2` writer"]
pub type W = crate::W<Tpr2Spec>;
#[doc = "Field `PDE_CS_H_CNT` reader - Indicate the time of cs required keep high after PDE The expression of PDE_CS_H_CNT is following PDE_CYCLE_NUM = SPIC_CLK_FRQ * tDP ( unit of SPIC_CLK_FRQ is MHz, and tDP is us). If 768>=PDX_CYCLE_NUM > 0, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/256). If 15872>=PDX_CYCLE_NUM > 768, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/512). If 524288>=PDX_CYCLE_NUM > 15872, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/16384) + 31. Ex: SPIC_CLK_FRQ = 400MHz, tDP = 100us, so PDX_CYCLE_NUM = 400*100=40000 524288 >= 40000 > 15872, so CR_PDX_CS_H_CNT = ceil(40000/512) + 31= 3+31 = 34. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeCsHCntR = crate::FieldReader;
#[doc = "Field `PDE_CS_H_CNT` writer - Indicate the time of cs required keep high after PDE The expression of PDE_CS_H_CNT is following PDE_CYCLE_NUM = SPIC_CLK_FRQ * tDP ( unit of SPIC_CLK_FRQ is MHz, and tDP is us). If 768>=PDX_CYCLE_NUM > 0, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/256). If 15872>=PDX_CYCLE_NUM > 768, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/512). If 524288>=PDX_CYCLE_NUM > 15872, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/16384) + 31. Ex: SPIC_CLK_FRQ = 400MHz, tDP = 100us, so PDX_CYCLE_NUM = 400*100=40000 524288 >= 40000 > 15872, so CR_PDX_CS_H_CNT = ceil(40000/512) + 31= 3+31 = 34. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeCsHCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PDX_CS_H_CNT` reader - Indicate the time of cs required keep high after PDX The expression of PDX_CS_H_CNT is following PDX_CYCLE_NUM = SPIC_CLK_FRQ * tDP (unit of SPIC_CLK_FRQ is MHz, and tDP is us). If 768>=PDX_CYCLE_NUM > 0, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/256). If 15872>=PDX_CYCLE_NUM > 768, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/512). If 524288>=PDX_CYCLE_NUM > 15872, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/16384) + 31. Ex: SPIC_CLK_FRQ = 400MHz, tDP = 100us, so PDX_CYCLE_NUM = 400*100=40000 524288 >= 40000 > 15872, so CR_PDX_CS_H_CNT = ceil(40000/512) + 31= 3+31 = 34. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdxCsHCntR = crate::FieldReader;
#[doc = "Field `PDX_CS_H_CNT` writer - Indicate the time of cs required keep high after PDX The expression of PDX_CS_H_CNT is following PDX_CYCLE_NUM = SPIC_CLK_FRQ * tDP (unit of SPIC_CLK_FRQ is MHz, and tDP is us). If 768>=PDX_CYCLE_NUM > 0, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/256). If 15872>=PDX_CYCLE_NUM > 768, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/512). If 524288>=PDX_CYCLE_NUM > 15872, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/16384) + 31. Ex: SPIC_CLK_FRQ = 400MHz, tDP = 100us, so PDX_CYCLE_NUM = 400*100=40000 524288 >= 40000 > 15872, so CR_PDX_CS_H_CNT = ceil(40000/512) + 31= 3+31 = 34. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdxCsHCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Indicate the time of cs required keep high after PDE The expression of PDE_CS_H_CNT is following PDE_CYCLE_NUM = SPIC_CLK_FRQ * tDP ( unit of SPIC_CLK_FRQ is MHz, and tDP is us). If 768>=PDX_CYCLE_NUM > 0, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/256). If 15872>=PDX_CYCLE_NUM > 768, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/512). If 524288>=PDX_CYCLE_NUM > 15872, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/16384) + 31. Ex: SPIC_CLK_FRQ = 400MHz, tDP = 100us, so PDX_CYCLE_NUM = 400*100=40000 524288 >= 40000 > 15872, so CR_PDX_CS_H_CNT = ceil(40000/512) + 31= 3+31 = 34. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_cs_h_cnt(&self) -> PdeCsHCntR {
        PdeCsHCntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Indicate the time of cs required keep high after PDX The expression of PDX_CS_H_CNT is following PDX_CYCLE_NUM = SPIC_CLK_FRQ * tDP (unit of SPIC_CLK_FRQ is MHz, and tDP is us). If 768>=PDX_CYCLE_NUM > 0, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/256). If 15872>=PDX_CYCLE_NUM > 768, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/512). If 524288>=PDX_CYCLE_NUM > 15872, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/16384) + 31. Ex: SPIC_CLK_FRQ = 400MHz, tDP = 100us, so PDX_CYCLE_NUM = 400*100=40000 524288 >= 40000 > 15872, so CR_PDX_CS_H_CNT = ceil(40000/512) + 31= 3+31 = 34. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pdx_cs_h_cnt(&self) -> PdxCsHCntR {
        PdxCsHCntR::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Indicate the time of cs required keep high after PDE The expression of PDE_CS_H_CNT is following PDE_CYCLE_NUM = SPIC_CLK_FRQ * tDP ( unit of SPIC_CLK_FRQ is MHz, and tDP is us). If 768>=PDX_CYCLE_NUM > 0, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/256). If 15872>=PDX_CYCLE_NUM > 768, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/512). If 524288>=PDX_CYCLE_NUM > 15872, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/16384) + 31. Ex: SPIC_CLK_FRQ = 400MHz, tDP = 100us, so PDX_CYCLE_NUM = 400*100=40000 524288 >= 40000 > 15872, so CR_PDX_CS_H_CNT = ceil(40000/512) + 31= 3+31 = 34. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_cs_h_cnt(&mut self) -> PdeCsHCntW<'_, Tpr2Spec> {
        PdeCsHCntW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Indicate the time of cs required keep high after PDX The expression of PDX_CS_H_CNT is following PDX_CYCLE_NUM = SPIC_CLK_FRQ * tDP (unit of SPIC_CLK_FRQ is MHz, and tDP is us). If 768>=PDX_CYCLE_NUM > 0, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/256). If 15872>=PDX_CYCLE_NUM > 768, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/512). If 524288>=PDX_CYCLE_NUM > 15872, CR_PDX_CS_H_CNT = ceil (PDE_CYCLE_NUM/16384) + 31. Ex: SPIC_CLK_FRQ = 400MHz, tDP = 100us, so PDX_CYCLE_NUM = 400*100=40000 524288 >= 40000 > 15872, so CR_PDX_CS_H_CNT = ceil(40000/512) + 31= 3+31 = 34. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pdx_cs_h_cnt(&mut self) -> PdxCsHCntW<'_, Tpr2Spec> {
        PdxCsHCntW::new(self, 6)
    }
}
#[doc = "Timing Parameters Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tpr2Spec;
impl crate::RegisterSpec for Tpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpr2::R`](R) reader structure"]
impl crate::Readable for Tpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tpr2::W`](W) writer structure"]
impl crate::Writable for Tpr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPR2 to value 0x0604"]
impl crate::Resettable for Tpr2Spec {
    const RESET_VALUE: u32 = 0x0604;
}
