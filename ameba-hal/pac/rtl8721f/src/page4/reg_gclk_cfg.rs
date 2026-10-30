#[doc = "Register `REG_GCLK_CFG` reader"]
pub type R = crate::R<RegGclkCfgSpec>;
#[doc = "Register `REG_GCLK_CFG` writer"]
pub type W = crate::W<RegGclkCfgSpec>;
#[doc = "Field `BIT_EN_PTCL_GCLK` reader - 1: enable clock gating"]
pub type BitEnPtclGclkR = crate::BitReader;
#[doc = "Field `BIT_EN_PTCL_GCLK` writer - 1: enable clock gating"]
pub type BitEnPtclGclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_QUEUE_RPT_B8` reader - 8\\] of BIT_EN_QUEUE_RPT, BIT8: EVENT Queue"]
pub type BitEnQueueRptB8R = crate::BitReader;
#[doc = "Field `BIT_EN_QUEUE_RPT_B8` writer - 8\\] of BIT_EN_QUEUE_RPT, BIT8: EVENT Queue"]
pub type BitEnQueueRptB8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_TXRPT_RW_PTR` reader - Write 1 to clear BIT_AMPDU_TXRPT_W_PTR & BIT_AMPDU_TXRPT_R_PTR (0x47C / 0x47D)"]
pub type BitClrTxrptRwPtrR = crate::BitReader;
#[doc = "Field `BIT_CLR_TXRPT_RW_PTR` writer - Write 1 to clear BIT_AMPDU_TXRPT_W_PTR & BIT_AMPDU_TXRPT_R_PTR (0x47C / 0x47D)"]
pub type BitClrTxrptRwPtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TIME_UNIT_SEL_1US` reader - select queue_time and lifetime unit 1: unit 1us 0: unit 256us"]
pub type BitTimeUnitSel1usR = crate::BitReader;
#[doc = "Field `BIT_TIME_UNIT_SEL_1US` writer - select queue_time and lifetime unit 1: unit 1us 0: unit 256us"]
pub type BitTimeUnitSel1usW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TBQ_BLK_ACQ_REQ` reader - 1: start select acq to gen_cmd need to wait tbq_txreq == 1"]
pub type BitTbqBlkAcqReqR = crate::BitReader;
#[doc = "Field `BIT_TBQ_BLK_ACQ_REQ` writer - 1: start select acq to gen_cmd need to wait tbq_txreq == 1"]
pub type BitTbqBlkAcqReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable clock gating"]
    #[inline(always)]
    pub fn bit_en_ptcl_gclk(&self) -> BitEnPtclGclkR {
        BitEnPtclGclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8\\] of BIT_EN_QUEUE_RPT, BIT8: EVENT Queue"]
    #[inline(always)]
    pub fn bit_en_queue_rpt_b8(&self) -> BitEnQueueRptB8R {
        BitEnQueueRptB8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to clear BIT_AMPDU_TXRPT_W_PTR & BIT_AMPDU_TXRPT_R_PTR (0x47C / 0x47D)"]
    #[inline(always)]
    pub fn bit_clr_txrpt_rw_ptr(&self) -> BitClrTxrptRwPtrR {
        BitClrTxrptRwPtrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - select queue_time and lifetime unit 1: unit 1us 0: unit 256us"]
    #[inline(always)]
    pub fn bit_time_unit_sel_1us(&self) -> BitTimeUnitSel1usR {
        BitTimeUnitSel1usR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: start select acq to gen_cmd need to wait tbq_txreq == 1"]
    #[inline(always)]
    pub fn bit_tbq_blk_acq_req(&self) -> BitTbqBlkAcqReqR {
        BitTbqBlkAcqReqR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable clock gating"]
    #[inline(always)]
    pub fn bit_en_ptcl_gclk(&mut self) -> BitEnPtclGclkW<'_, RegGclkCfgSpec> {
        BitEnPtclGclkW::new(self, 0)
    }
    #[doc = "Bit 1 - 8\\] of BIT_EN_QUEUE_RPT, BIT8: EVENT Queue"]
    #[inline(always)]
    pub fn bit_en_queue_rpt_b8(&mut self) -> BitEnQueueRptB8W<'_, RegGclkCfgSpec> {
        BitEnQueueRptB8W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear BIT_AMPDU_TXRPT_W_PTR & BIT_AMPDU_TXRPT_R_PTR (0x47C / 0x47D)"]
    #[inline(always)]
    pub fn bit_clr_txrpt_rw_ptr(&mut self) -> BitClrTxrptRwPtrW<'_, RegGclkCfgSpec> {
        BitClrTxrptRwPtrW::new(self, 2)
    }
    #[doc = "Bit 3 - select queue_time and lifetime unit 1: unit 1us 0: unit 256us"]
    #[inline(always)]
    pub fn bit_time_unit_sel_1us(&mut self) -> BitTimeUnitSel1usW<'_, RegGclkCfgSpec> {
        BitTimeUnitSel1usW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: start select acq to gen_cmd need to wait tbq_txreq == 1"]
    #[inline(always)]
    pub fn bit_tbq_blk_acq_req(&mut self) -> BitTbqBlkAcqReqW<'_, RegGclkCfgSpec> {
        BitTbqBlkAcqReqW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gclk_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_gclk_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGclkCfgSpec;
impl crate::RegisterSpec for RegGclkCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gclk_cfg::R`](R) reader structure"]
impl crate::Readable for RegGclkCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_gclk_cfg::W`](W) writer structure"]
impl crate::Writable for RegGclkCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GCLK_CFG to value 0x13"]
impl crate::Resettable for RegGclkCfgSpec {
    const RESET_VALUE: u32 = 0x13;
}
