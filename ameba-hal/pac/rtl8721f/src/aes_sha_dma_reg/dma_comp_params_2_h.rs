#[doc = "Register `DMA_COMP_PARAMS_2_H` reader"]
pub type R = crate::R<DmaCompParams2HSpec>;
#[doc = "Field `CH0_MULTI_BLK_TYPE` reader - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
pub type Ch0MultiBlkTypeR = crate::FieldReader;
#[doc = "Field `CH1_MULTI_BLK_TYPE` reader - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
pub type Ch1MultiBlkTypeR = crate::FieldReader;
#[doc = "Field `CH2_MULTI_BLK_TYPE` reader - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
pub type Ch2MultiBlkTypeR = crate::FieldReader;
#[doc = "Field `CH3_MULTI_BLK_TYPE` reader - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
pub type Ch3MultiBlkTypeR = crate::FieldReader;
#[doc = "Field `CH4_MULTI_BLK_TYPE` reader - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
pub type Ch4MultiBlkTypeR = crate::FieldReader;
#[doc = "Field `CH5_MULTI_BLK_TYPE` reader - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
pub type Ch5MultiBlkTypeR = crate::FieldReader;
#[doc = "Field `CH6_MULTI_BLK_TYPE` reader - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
pub type Ch6MultiBlkTypeR = crate::FieldReader;
#[doc = "Field `CH7_MULTI_BLK_TYPE` reader - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
pub type Ch7MultiBlkTypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
    #[inline(always)]
    pub fn ch0_multi_blk_type(&self) -> Ch0MultiBlkTypeR {
        Ch0MultiBlkTypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
    #[inline(always)]
    pub fn ch1_multi_blk_type(&self) -> Ch1MultiBlkTypeR {
        Ch1MultiBlkTypeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
    #[inline(always)]
    pub fn ch2_multi_blk_type(&self) -> Ch2MultiBlkTypeR {
        Ch2MultiBlkTypeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
    #[inline(always)]
    pub fn ch3_multi_blk_type(&self) -> Ch3MultiBlkTypeR {
        Ch3MultiBlkTypeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
    #[inline(always)]
    pub fn ch4_multi_blk_type(&self) -> Ch4MultiBlkTypeR {
        Ch4MultiBlkTypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
    #[inline(always)]
    pub fn ch5_multi_blk_type(&self) -> Ch5MultiBlkTypeR {
        Ch5MultiBlkTypeR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
    #[inline(always)]
    pub fn ch6_multi_blk_type(&self) -> Ch6MultiBlkTypeR {
        Ch6MultiBlkTypeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - The values of these bit fields are derived from the DMAC_CHx_MULTI_BLK_TYPE parameter. 0x0 = NO_HARDCODE 0x1 = CONT_RELOAD 0x2 = RELOAD_CONT 0x3 = RELOAD_RELOAD 0x4 = CONT_LLP 0x5 = RELOAD_LLP 0x6 = LLP_CONT 0x7 = LLP_RELOAD 0x8 = LLP_LLP"]
    #[inline(always)]
    pub fn ch7_multi_blk_type(&self) -> Ch7MultiBlkTypeR {
        Ch7MultiBlkTypeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_2_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams2HSpec;
impl crate::RegisterSpec for DmaCompParams2HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_2_h::R`](R) reader structure"]
impl crate::Readable for DmaCompParams2HSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_2_H to value 0"]
impl crate::Resettable for DmaCompParams2HSpec {}
