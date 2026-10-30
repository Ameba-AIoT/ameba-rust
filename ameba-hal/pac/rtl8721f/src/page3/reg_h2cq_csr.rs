#[doc = "Register `REG_H2CQ_CSR` reader"]
pub type R = crate::R<RegH2cqCsrSpec>;
#[doc = "Register `REG_H2CQ_CSR` writer"]
pub type W = crate::W<RegH2cqCsrSpec>;
#[doc = "Field `BIT_STOPH2C` reader - Stop h2c queue dma"]
pub type BitStoph2cR = crate::BitReader;
#[doc = "Field `BIT_STOPH2C` writer - Stop h2c queue dma"]
pub type BitStoph2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_H2CQ_HW_IDX` writer - Write 1 to clear H2CQ hardware index, and then hardware auto clear the bit"]
pub type BitClrH2cqHwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_H2CQ_HOST_IDX` writer - Write 1 to clear H2CQ host index, and then hardware auto clear the bit"]
pub type BitClrH2cqHostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_H2CQ_FULL` reader - Indicate H2CQ full. Write 1 to clear the bit."]
pub type BitH2cqFullR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stop h2c queue dma"]
    #[inline(always)]
    pub fn bit_stoph2c(&self) -> BitStoph2cR {
        BitStoph2cR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Indicate H2CQ full. Write 1 to clear the bit."]
    #[inline(always)]
    pub fn bit_h2cq_full(&self) -> BitH2cqFullR {
        BitH2cqFullR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop h2c queue dma"]
    #[inline(always)]
    pub fn bit_stoph2c(&mut self) -> BitStoph2cW<'_, RegH2cqCsrSpec> {
        BitStoph2cW::new(self, 0)
    }
    #[doc = "Bit 8 - Write 1 to clear H2CQ hardware index, and then hardware auto clear the bit"]
    #[inline(always)]
    pub fn bit_clr_h2cq_hw_idx(&mut self) -> BitClrH2cqHwIdxW<'_, RegH2cqCsrSpec> {
        BitClrH2cqHwIdxW::new(self, 8)
    }
    #[doc = "Bit 16 - Write 1 to clear H2CQ host index, and then hardware auto clear the bit"]
    #[inline(always)]
    pub fn bit_clr_h2cq_host_idx(&mut self) -> BitClrH2cqHostIdxW<'_, RegH2cqCsrSpec> {
        BitClrH2cqHostIdxW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_h2cq_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_h2cq_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegH2cqCsrSpec;
impl crate::RegisterSpec for RegH2cqCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_h2cq_csr::R`](R) reader structure"]
impl crate::Readable for RegH2cqCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_h2cq_csr::W`](W) writer structure"]
impl crate::Writable for RegH2cqCsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_H2CQ_CSR to value 0"]
impl crate::Resettable for RegH2cqCsrSpec {}
