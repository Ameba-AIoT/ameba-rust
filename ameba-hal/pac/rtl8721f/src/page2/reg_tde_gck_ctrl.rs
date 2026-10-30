#[doc = "Register `REG_TDE_GCK_CTRL` reader"]
pub type R = crate::R<RegTdeGckCtrlSpec>;
#[doc = "Register `REG_TDE_GCK_CTRL` writer"]
pub type W = crate::W<RegTdeGckCtrlSpec>;
#[doc = "Field `BIT_TDE_GCLK_EN` reader - enable this bit,txdma engine will use gated clk"]
pub type BitTdeGclkEnR = crate::BitReader;
#[doc = "Field `BIT_TDE_GCLK_EN` writer - enable this bit,txdma engine will use gated clk"]
pub type BitTdeGclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DMA_WAIT_MORE_CYCLE` reader - enable this bit,tde_ctl will wait for 16 dma_clk when host dma last data to tde_sync_fifo 1:used for 20M mac clk 0:used for 40M/80M mac clk(will wait for 12 dma_clk)"]
pub type BitDmaWaitMoreCycleR = crate::BitReader;
#[doc = "Field `BIT_DMA_WAIT_MORE_CYCLE` writer - enable this bit,tde_ctl will wait for 16 dma_clk when host dma last data to tde_sync_fifo 1:used for 20M mac clk 0:used for 40M/80M mac clk(will wait for 12 dma_clk)"]
pub type BitDmaWaitMoreCycleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TDE_GCLK_DLY_CNT` reader - when start trigger of clk gating happens, count for several cycles then disable clk,default:16 cycles"]
pub type BitTdeGclkDlyCntR = crate::FieldReader;
#[doc = "Field `BIT_TDE_GCLK_DLY_CNT` writer - when start trigger of clk gating happens, count for several cycles then disable clk,default:16 cycles"]
pub type BitTdeGclkDlyCntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TDE_PTR_RST` writer - Write this bit reset FSM of tde_ptr_cs, ioreg_txfail"]
pub type BitTdePtrRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TDE_PAGE_RST` writer - Write this bit reset FSM of tde_page_cs, ioreg_txfail"]
pub type BitTdePageRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TDE_CTRL_RST` writer - Write this bit reset FSM of tde_ctrl, ioreg_txfail"]
pub type BitTdeCtrlRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 0 - enable this bit,txdma engine will use gated clk"]
    #[inline(always)]
    pub fn bit_tde_gclk_en(&self) -> BitTdeGclkEnR {
        BitTdeGclkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable this bit,tde_ctl will wait for 16 dma_clk when host dma last data to tde_sync_fifo 1:used for 20M mac clk 0:used for 40M/80M mac clk(will wait for 12 dma_clk)"]
    #[inline(always)]
    pub fn bit_dma_wait_more_cycle(&self) -> BitDmaWaitMoreCycleR {
        BitDmaWaitMoreCycleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - when start trigger of clk gating happens, count for several cycles then disable clk,default:16 cycles"]
    #[inline(always)]
    pub fn bit_tde_gclk_dly_cnt(&self) -> BitTdeGclkDlyCntR {
        BitTdeGclkDlyCntR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 11:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - enable this bit,txdma engine will use gated clk"]
    #[inline(always)]
    pub fn bit_tde_gclk_en(&mut self) -> BitTdeGclkEnW<'_, RegTdeGckCtrlSpec> {
        BitTdeGclkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - enable this bit,tde_ctl will wait for 16 dma_clk when host dma last data to tde_sync_fifo 1:used for 20M mac clk 0:used for 40M/80M mac clk(will wait for 12 dma_clk)"]
    #[inline(always)]
    pub fn bit_dma_wait_more_cycle(&mut self) -> BitDmaWaitMoreCycleW<'_, RegTdeGckCtrlSpec> {
        BitDmaWaitMoreCycleW::new(self, 1)
    }
    #[doc = "Bits 2:6 - when start trigger of clk gating happens, count for several cycles then disable clk,default:16 cycles"]
    #[inline(always)]
    pub fn bit_tde_gclk_dly_cnt(&mut self) -> BitTdeGclkDlyCntW<'_, RegTdeGckCtrlSpec> {
        BitTdeGclkDlyCntW::new(self, 2)
    }
    #[doc = "Bit 7 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegTdeGckCtrlSpec> {
        Rsvd2W::new(self, 7)
    }
    #[doc = "Bit 8 - Write this bit reset FSM of tde_ptr_cs, ioreg_txfail"]
    #[inline(always)]
    pub fn bit_tde_ptr_rst(&mut self) -> BitTdePtrRstW<'_, RegTdeGckCtrlSpec> {
        BitTdePtrRstW::new(self, 8)
    }
    #[doc = "Bit 9 - Write this bit reset FSM of tde_page_cs, ioreg_txfail"]
    #[inline(always)]
    pub fn bit_tde_page_rst(&mut self) -> BitTdePageRstW<'_, RegTdeGckCtrlSpec> {
        BitTdePageRstW::new(self, 9)
    }
    #[doc = "Bit 10 - Write this bit reset FSM of tde_ctrl, ioreg_txfail"]
    #[inline(always)]
    pub fn bit_tde_ctrl_rst(&mut self) -> BitTdeCtrlRstW<'_, RegTdeGckCtrlSpec> {
        BitTdeCtrlRstW::new(self, 10)
    }
    #[doc = "Bits 11:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTdeGckCtrlSpec> {
        RsvdW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tde_gck_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tde_gck_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTdeGckCtrlSpec;
impl crate::RegisterSpec for RegTdeGckCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tde_gck_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTdeGckCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tde_gck_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTdeGckCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TDE_GCK_CTRL to value 0x3f"]
impl crate::Resettable for RegTdeGckCtrlSpec {
    const RESET_VALUE: u32 = 0x3f;
}
