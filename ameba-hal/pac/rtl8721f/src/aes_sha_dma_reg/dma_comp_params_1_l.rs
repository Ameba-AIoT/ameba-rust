#[doc = "Register `DMA_COMP_PARAMS_1_L` reader"]
pub type R = crate::R<DmaCompParams1LSpec>;
#[doc = "Field `CH0_MAX_BLK_SIZE` reader - The value of this register is derived from the DMAC_CH0_MAX_BLK_SIZE from config_form.vh CH0_MAX_BLK_SIZE = log2(DMAC_CH0_MAX_BLK_SIZE) + 1"]
pub type Ch0MaxBlkSizeR = crate::FieldReader;
#[doc = "Field `CH1_MAX_BLK_SIZE` reader - The value of this register is derived from the DMAC_CH1_MAX_BLK_SIZE from config_form.vh CH1_MAX_BLK_SIZE = log2(DMAC_CH1_MAX_BLK_SIZE) + 1"]
pub type Ch1MaxBlkSizeR = crate::FieldReader;
#[doc = "Field `CH2_MAX_BLK_SIZE` reader - The value of this register is derived from the DMAC_CH2_MAX_BLK_SIZE from config_form.vh CH2_MAX_BLK_SIZE = log2(DMAC_CH2_MAX_BLK_SIZE) + 1"]
pub type Ch2MaxBlkSizeR = crate::FieldReader;
#[doc = "Field `CH3_MAX_BLK_SIZE` reader - The value of this register is derived from the DMAC_CH3_MAX_BLK_SIZE from config_form.vh CH3_MAX_BLK_SIZE = log2(DMAC_CH3_MAX_BLK_SIZE) + 1"]
pub type Ch3MaxBlkSizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The value of this register is derived from the DMAC_CH0_MAX_BLK_SIZE from config_form.vh CH0_MAX_BLK_SIZE = log2(DMAC_CH0_MAX_BLK_SIZE) + 1"]
    #[inline(always)]
    pub fn ch0_max_blk_size(&self) -> Ch0MaxBlkSizeR {
        Ch0MaxBlkSizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The value of this register is derived from the DMAC_CH1_MAX_BLK_SIZE from config_form.vh CH1_MAX_BLK_SIZE = log2(DMAC_CH1_MAX_BLK_SIZE) + 1"]
    #[inline(always)]
    pub fn ch1_max_blk_size(&self) -> Ch1MaxBlkSizeR {
        Ch1MaxBlkSizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The value of this register is derived from the DMAC_CH2_MAX_BLK_SIZE from config_form.vh CH2_MAX_BLK_SIZE = log2(DMAC_CH2_MAX_BLK_SIZE) + 1"]
    #[inline(always)]
    pub fn ch2_max_blk_size(&self) -> Ch2MaxBlkSizeR {
        Ch2MaxBlkSizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The value of this register is derived from the DMAC_CH3_MAX_BLK_SIZE from config_form.vh CH3_MAX_BLK_SIZE = log2(DMAC_CH3_MAX_BLK_SIZE) + 1"]
    #[inline(always)]
    pub fn ch3_max_blk_size(&self) -> Ch3MaxBlkSizeR {
        Ch3MaxBlkSizeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_1_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams1LSpec;
impl crate::RegisterSpec for DmaCompParams1LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_1_l::R`](R) reader structure"]
impl crate::Readable for DmaCompParams1LSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_1_L to value 0x1515_1515"]
impl crate::Resettable for DmaCompParams1LSpec {
    const RESET_VALUE: u32 = 0x1515_1515;
}
