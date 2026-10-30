#[doc = "Register `DMA_COMP_PARAMS_6_L` reader"]
pub type R = crate::R<DmaCompParams6LSpec>;
#[doc = "Field `CH4_MAX_BLK_SIZE` reader - The value of this register is derived from the DMAC_CH4_MAX_BLK_SIZE from config_form.vh CH4_MAX_BLK_SIZE = log2(DMAC_CH4_MAX_BLK_SIZE) + 1"]
pub type Ch4MaxBlkSizeR = crate::FieldReader;
#[doc = "Field `CH5_MAX_BLK_SIZE` reader - The value of this register is derived from the DMAC_CH5_MAX_BLK_SIZE from config_form.vh CH5_MAX_BLK_SIZE = log2(DMAC_CH5_MAX_BLK_SIZE) + 1"]
pub type Ch5MaxBlkSizeR = crate::FieldReader;
#[doc = "Field `CH6_MAX_BLK_SIZE` reader - The value of this register is derived from the DMAC_CH6_MAX_BLK_SIZE from config_form.vh CH6_MAX_BLK_SIZE = log2(DMAC_CH6_MAX_BLK_SIZE) + 1"]
pub type Ch6MaxBlkSizeR = crate::FieldReader;
#[doc = "Field `CH7_MAX_BLK_SIZE` reader - The value of this register is derived from the DMAC_CH7_MAX_BLK_SIZE from config_form.vh CH7_MAX_BLK_SIZE = log2(DMAC_CH7_MAX_BLK_SIZE) + 1"]
pub type Ch7MaxBlkSizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The value of this register is derived from the DMAC_CH4_MAX_BLK_SIZE from config_form.vh CH4_MAX_BLK_SIZE = log2(DMAC_CH4_MAX_BLK_SIZE) + 1"]
    #[inline(always)]
    pub fn ch4_max_blk_size(&self) -> Ch4MaxBlkSizeR {
        Ch4MaxBlkSizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The value of this register is derived from the DMAC_CH5_MAX_BLK_SIZE from config_form.vh CH5_MAX_BLK_SIZE = log2(DMAC_CH5_MAX_BLK_SIZE) + 1"]
    #[inline(always)]
    pub fn ch5_max_blk_size(&self) -> Ch5MaxBlkSizeR {
        Ch5MaxBlkSizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The value of this register is derived from the DMAC_CH6_MAX_BLK_SIZE from config_form.vh CH6_MAX_BLK_SIZE = log2(DMAC_CH6_MAX_BLK_SIZE) + 1"]
    #[inline(always)]
    pub fn ch6_max_blk_size(&self) -> Ch6MaxBlkSizeR {
        Ch6MaxBlkSizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The value of this register is derived from the DMAC_CH7_MAX_BLK_SIZE from config_form.vh CH7_MAX_BLK_SIZE = log2(DMAC_CH7_MAX_BLK_SIZE) + 1"]
    #[inline(always)]
    pub fn ch7_max_blk_size(&self) -> Ch7MaxBlkSizeR {
        Ch7MaxBlkSizeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_6_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams6LSpec;
impl crate::RegisterSpec for DmaCompParams6LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_6_l::R`](R) reader structure"]
impl crate::Readable for DmaCompParams6LSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_6_L to value 0x1515_1515"]
impl crate::Resettable for DmaCompParams6LSpec {
    const RESET_VALUE: u32 = 0x1515_1515;
}
