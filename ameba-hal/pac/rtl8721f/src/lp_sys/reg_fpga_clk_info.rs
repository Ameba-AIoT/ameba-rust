#[doc = "Register `REG_FPGA_CLK_INFO` reader"]
pub type R = crate::R<RegFpgaClkInfoSpec>;
#[doc = "Field `FPGA_SPIC_CLK` reader - FPGA SPIC clock"]
pub type FpgaSpicClkR = crate::FieldReader;
#[doc = "Field `FPGA_PSRAM_CLK` reader - FPGA PSRAM clock"]
pub type FpgaPsramClkR = crate::FieldReader;
#[doc = "Field `FPGA_KM4_CLK` reader - FPGA KM4 clock"]
pub type FpgaKm4ClkR = crate::FieldReader;
#[doc = "Field `FPGA_KR4_CLK` reader - FPGA KR4 clock"]
pub type FpgaKr4ClkR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - FPGA SPIC clock"]
    #[inline(always)]
    pub fn fpga_spic_clk(&self) -> FpgaSpicClkR {
        FpgaSpicClkR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - FPGA PSRAM clock"]
    #[inline(always)]
    pub fn fpga_psram_clk(&self) -> FpgaPsramClkR {
        FpgaPsramClkR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - FPGA KM4 clock"]
    #[inline(always)]
    pub fn fpga_km4_clk(&self) -> FpgaKm4ClkR {
        FpgaKm4ClkR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FPGA KR4 clock"]
    #[inline(always)]
    pub fn fpga_kr4_clk(&self) -> FpgaKr4ClkR {
        FpgaKr4ClkR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fpga_clk_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFpgaClkInfoSpec;
impl crate::RegisterSpec for RegFpgaClkInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fpga_clk_info::R`](R) reader structure"]
impl crate::Readable for RegFpgaClkInfoSpec {}
#[doc = "`reset()` method sets REG_FPGA_CLK_INFO to value 0"]
impl crate::Resettable for RegFpgaClkInfoSpec {}
