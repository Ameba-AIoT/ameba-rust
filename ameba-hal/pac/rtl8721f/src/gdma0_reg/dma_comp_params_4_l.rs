#[doc = "Register `DMA_COMP_PARAMS_4_L` reader"]
pub type R = crate::R<DmaCompParams4LSpec>;
#[doc = "Field `CH4_DTW` reader - The value of this register is derived from the DMAC_CH4_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch4DtwR = crate::FieldReader;
#[doc = "Field `CH4_STW` reader - The value of this register is derived from the DMAC_CH4_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch4StwR = crate::FieldReader;
#[doc = "Field `CH4_STAT_DST` reader - The value of this register is derived from the DMAC_CH4_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch4StatDstR = crate::BitReader;
#[doc = "Field `CH4_STAT_SRC` reader - The value of this register is derived from the DMAC_CH4_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch4StatSrcR = crate::BitReader;
#[doc = "Field `CH4_DST_SCA_EN` reader - The value of this register is derived from the DMAC_CH4_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch4DstScaEnR = crate::BitReader;
#[doc = "Field `CH4_SRC_GAT_EN` reader - The value of this register is derived from the DMAC_CH4_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112.0x0 = FALSE 0x1 = TRUE"]
pub type Ch4SrcGatEnR = crate::BitReader;
#[doc = "Field `CH4_LOCK_EN` reader - The value of this register is derived from the DMAC_CH4_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch4LockEnR = crate::BitReader;
#[doc = "Field `CH4_MULTI_BLK_EN` reader - The value of this register is derived from the DMAC_CH4_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch4MultiBlkEnR = crate::BitReader;
#[doc = "Field `CH4_CTL_WB_EN` reader - The value of this register is derived from the DMAC_CH4_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch4CtlWbEnR = crate::BitReader;
#[doc = "Field `CH4_HC_LLP` reader - The value of this register is derived from the DMAC_CH4_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch4HcLlpR = crate::BitReader;
#[doc = "Field `CH4_FC` reader - The value of this register is derived from the DMAC_CH4_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
pub type Ch4FcR = crate::FieldReader;
#[doc = "Field `CH4_MAX_MULT_SIZE` reader - The value of this register is derived from the DMAC_CH4_MULT_SIZE parameter. Maxium burst transaction size. CH4_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
pub type Ch4MaxMultSizeR = crate::FieldReader;
#[doc = "Field `CH4_FIFO_DEPTH` reader - The value of this register is derived from the DMAC_CH4_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
pub type Ch4FifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The value of this register is derived from the DMAC_CH4_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch4_dtw(&self) -> Ch4DtwR {
        Ch4DtwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The value of this register is derived from the DMAC_CH4_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch4_stw(&self) -> Ch4StwR {
        Ch4StwR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The value of this register is derived from the DMAC_CH4_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch4_stat_dst(&self) -> Ch4StatDstR {
        Ch4StatDstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the DMAC_CH4_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch4_stat_src(&self) -> Ch4StatSrcR {
        Ch4StatSrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the DMAC_CH4_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch4_dst_sca_en(&self) -> Ch4DstScaEnR {
        Ch4DstScaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the DMAC_CH4_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112.0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch4_src_gat_en(&self) -> Ch4SrcGatEnR {
        Ch4SrcGatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the DMAC_CH4_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch4_lock_en(&self) -> Ch4LockEnR {
        Ch4LockEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the DMAC_CH4_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch4_multi_blk_en(&self) -> Ch4MultiBlkEnR {
        Ch4MultiBlkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the DMAC_CH4_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch4_ctl_wb_en(&self) -> Ch4CtlWbEnR {
        Ch4CtlWbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the DMAC_CH4_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch4_hc_llp(&self) -> Ch4HcLlpR {
        Ch4HcLlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - The value of this register is derived from the DMAC_CH4_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
    #[inline(always)]
    pub fn ch4_fc(&self) -> Ch4FcR {
        Ch4FcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - The value of this register is derived from the DMAC_CH4_MULT_SIZE parameter. Maxium burst transaction size. CH4_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
    #[inline(always)]
    pub fn ch4_max_mult_size(&self) -> Ch4MaxMultSizeR {
        Ch4MaxMultSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The value of this register is derived from the DMAC_CH4_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
    #[inline(always)]
    pub fn ch4_fifo_depth(&self) -> Ch4FifoDepthR {
        Ch4FifoDepthR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_4_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams4LSpec;
impl crate::RegisterSpec for DmaCompParams4LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_4_l::R`](R) reader structure"]
impl crate::Readable for DmaCompParams4LSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_4_L to value 0x2003_c81b"]
impl crate::Resettable for DmaCompParams4LSpec {
    const RESET_VALUE: u32 = 0x2003_c81b;
}
