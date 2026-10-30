#[doc = "Register `DMA_COMP_PARAMS_3_H` reader"]
pub type R = crate::R<DmaCompParams3HSpec>;
#[doc = "Field `CH1_DTW` reader - The value of this register is derived from the DMAC_CH1_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch1DtwR = crate::FieldReader;
#[doc = "Field `CH1_STW` reader - The value of this register is derived from the DMAC_CH1_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch1StwR = crate::FieldReader;
#[doc = "Field `CH1_STAT_DST` reader - The value of this register is derived from the DMAC_CH1_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch1StatDstR = crate::BitReader;
#[doc = "Field `CH1_STAT_SRC` reader - The value of this register is derived from the DMAC_CH1_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch1StatSrcR = crate::BitReader;
#[doc = "Field `CH1_DST_SCA_EN` reader - The value of this register is derived from the DMAC_CH1_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch1DstScaEnR = crate::BitReader;
#[doc = "Field `CH1_SRC_GAT_EN` reader - The value of this register is derived from the DMAC_CH1_SRC_GAT_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch1SrcGatEnR = crate::BitReader;
#[doc = "Field `CH1_LOCK_EN` reader - The value of this register is derived from the DMAC_CH1_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch1LockEnR = crate::BitReader;
#[doc = "Field `CH1_MULTI_BLK_EN` reader - The value of this register is derived from the DMAC_CH1_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch1MultiBlkEnR = crate::BitReader;
#[doc = "Field `CH1_CTL_WB_EN` reader - The value of this register is derived from the DMAC_CH1_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch1CtlWbEnR = crate::BitReader;
#[doc = "Field `CH1_HC_LLP` reader - The value of this register is derived from the DMAC_CH1_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch1HcLlpR = crate::BitReader;
#[doc = "Field `CH1_FC` reader - The value of this register is derived from the DMAC_CH1_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
pub type Ch1FcR = crate::FieldReader;
#[doc = "Field `CH1_MAX_MULT_SIZE` reader - The value of this register is derived from the DMAC_CH1_MULT_SIZE parameter. Maxium burst transaction size. CH1_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
pub type Ch1MaxMultSizeR = crate::FieldReader;
#[doc = "Field `CH1_FIFO_DEPTH` reader - The value of this register is derived from the DMAC_CH1_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
pub type Ch1FifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The value of this register is derived from the DMAC_CH1_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch1_dtw(&self) -> Ch1DtwR {
        Ch1DtwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The value of this register is derived from the DMAC_CH1_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch1_stw(&self) -> Ch1StwR {
        Ch1StwR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The value of this register is derived from the DMAC_CH1_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch1_stat_dst(&self) -> Ch1StatDstR {
        Ch1StatDstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the DMAC_CH1_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch1_stat_src(&self) -> Ch1StatSrcR {
        Ch1StatSrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the DMAC_CH1_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch1_dst_sca_en(&self) -> Ch1DstScaEnR {
        Ch1DstScaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the DMAC_CH1_SRC_GAT_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch1_src_gat_en(&self) -> Ch1SrcGatEnR {
        Ch1SrcGatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the DMAC_CH1_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch1_lock_en(&self) -> Ch1LockEnR {
        Ch1LockEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the DMAC_CH1_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch1_multi_blk_en(&self) -> Ch1MultiBlkEnR {
        Ch1MultiBlkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the DMAC_CH1_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch1_ctl_wb_en(&self) -> Ch1CtlWbEnR {
        Ch1CtlWbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the DMAC_CH1_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch1_hc_llp(&self) -> Ch1HcLlpR {
        Ch1HcLlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - The value of this register is derived from the DMAC_CH1_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
    #[inline(always)]
    pub fn ch1_fc(&self) -> Ch1FcR {
        Ch1FcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - The value of this register is derived from the DMAC_CH1_MULT_SIZE parameter. Maxium burst transaction size. CH1_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
    #[inline(always)]
    pub fn ch1_max_mult_size(&self) -> Ch1MaxMultSizeR {
        Ch1MaxMultSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The value of this register is derived from the DMAC_CH1_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
    #[inline(always)]
    pub fn ch1_fifo_depth(&self) -> Ch1FifoDepthR {
        Ch1FifoDepthR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_3_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams3HSpec;
impl crate::RegisterSpec for DmaCompParams3HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_3_h::R`](R) reader structure"]
impl crate::Readable for DmaCompParams3HSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_3_H to value 0x4003_cb1b"]
impl crate::Resettable for DmaCompParams3HSpec {
    const RESET_VALUE: u32 = 0x4003_cb1b;
}
