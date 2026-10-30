#[doc = "Register `DMA_COMP_PARAMS_5_L` reader"]
pub type R = crate::R<DmaCompParams5LSpec>;
#[doc = "Field `CH6_DTW` reader - The value of this register is derived from the DMAC_CH6_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch6DtwR = crate::FieldReader;
#[doc = "Field `CH6_STW` reader - The value of this register is derived from the DMAC_CH6_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch6StwR = crate::FieldReader;
#[doc = "Field `CH6_STAT_DST` reader - The value of this register is derived from the DMAC_CH6_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch6StatDstR = crate::BitReader;
#[doc = "Field `CH6_STAT_SRC` reader - The value of this register is derived from the DMAC_CH6_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch6StatSrcR = crate::BitReader;
#[doc = "Field `CH6_DST_SCA_EN` reader - The value of this register is derived from the DMAC_CH6_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch6DstScaEnR = crate::BitReader;
#[doc = "Field `CH6_SRC_GAT_EN` reader - The value of this register is derived from the DMAC_CH6_SRC_GAT_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch6SrcGatEnR = crate::BitReader;
#[doc = "Field `CH6_LOCK_EN` reader - The value of this register is derived from the DMAC_CH6_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch6LockEnR = crate::BitReader;
#[doc = "Field `CH6_MULTI_BLK_EN` reader - The value of this register is derived from the DMAC_CH6_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch6MultiBlkEnR = crate::BitReader;
#[doc = "Field `CH6_CTL_WB_EN` reader - The value of this register is derived from the DMAC_CH6_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch6CtlWbEnR = crate::BitReader;
#[doc = "Field `CH6_HC_LLP` reader - The value of this register is derived from the DMAC_CH6_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch6HcLlpR = crate::BitReader;
#[doc = "Field `CH6_FC` reader - The value of this register is derived from the DMAC_CH6_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
pub type Ch6FcR = crate::FieldReader;
#[doc = "Field `CH6_MAX_MULT_SIZE` reader - The value of this register is derived from the DMAC_CH6_MULT_SIZE parameter. Maxium burst transaction size. CH6_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
pub type Ch6MaxMultSizeR = crate::FieldReader;
#[doc = "Field `CH6_FIFO_DEPTH` reader - The value of this register is derived from the DMAC_CH6_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
pub type Ch6FifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The value of this register is derived from the DMAC_CH6_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch6_dtw(&self) -> Ch6DtwR {
        Ch6DtwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The value of this register is derived from the DMAC_CH6_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch6_stw(&self) -> Ch6StwR {
        Ch6StwR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The value of this register is derived from the DMAC_CH6_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch6_stat_dst(&self) -> Ch6StatDstR {
        Ch6StatDstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the DMAC_CH6_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch6_stat_src(&self) -> Ch6StatSrcR {
        Ch6StatSrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the DMAC_CH6_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch6_dst_sca_en(&self) -> Ch6DstScaEnR {
        Ch6DstScaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the DMAC_CH6_SRC_GAT_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch6_src_gat_en(&self) -> Ch6SrcGatEnR {
        Ch6SrcGatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the DMAC_CH6_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch6_lock_en(&self) -> Ch6LockEnR {
        Ch6LockEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the DMAC_CH6_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch6_multi_blk_en(&self) -> Ch6MultiBlkEnR {
        Ch6MultiBlkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the DMAC_CH6_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch6_ctl_wb_en(&self) -> Ch6CtlWbEnR {
        Ch6CtlWbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the DMAC_CH6_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch6_hc_llp(&self) -> Ch6HcLlpR {
        Ch6HcLlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - The value of this register is derived from the DMAC_CH6_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
    #[inline(always)]
    pub fn ch6_fc(&self) -> Ch6FcR {
        Ch6FcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - The value of this register is derived from the DMAC_CH6_MULT_SIZE parameter. Maxium burst transaction size. CH6_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
    #[inline(always)]
    pub fn ch6_max_mult_size(&self) -> Ch6MaxMultSizeR {
        Ch6MaxMultSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The value of this register is derived from the DMAC_CH6_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
    #[inline(always)]
    pub fn ch6_fifo_depth(&self) -> Ch6FifoDepthR {
        Ch6FifoDepthR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_5_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams5LSpec;
impl crate::RegisterSpec for DmaCompParams5LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_5_l::R`](R) reader structure"]
impl crate::Readable for DmaCompParams5LSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_5_L to value 0x2003_c81b"]
impl crate::Resettable for DmaCompParams5LSpec {
    const RESET_VALUE: u32 = 0x2003_c81b;
}
