#[doc = "Register `Clk_Ctrl_REG` reader"]
pub type R = crate::R<ClkCtrlRegSpec>;
#[doc = "Register `Clk_Ctrl_REG` writer"]
pub type W = crate::W<ClkCtrlRegSpec>;
#[doc = "Field `TXCLK_EN` reader - 0: Tx Clock Gated (rxc_out)"]
pub type TxclkEnR = crate::BitReader;
#[doc = "Field `TXCLK_EN` writer - 0: Tx Clock Gated (rxc_out)"]
pub type TxclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCLK_EN` reader - 0: Rx Clock Gated (txc_out)"]
pub type RxclkEnR = crate::BitReader;
#[doc = "Field `RXCLK_EN` writer - 0: Rx Clock Gated (txc_out)"]
pub type RxclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_data_swap` reader - 1: Swap data order for dma_mst"]
pub type DmaDataSwapR = crate::BitReader;
#[doc = "Field `dma_data_swap` writer - 1: Swap data order for dma_mst"]
pub type DmaDataSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Rev_00e4_3` reader - Reserved bit"]
pub type Rev00e4_3R = crate::BitReader;
#[doc = "Field `Rev_00e4_3` writer - Reserved bit"]
pub type Rev00e4_3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Rev_00e4_4` reader - Reserved bit"]
pub type Rev00e4_4R = crate::BitReader;
#[doc = "Field `Rev_00e4_4` writer - Reserved bit"]
pub type Rev00e4_4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Rev_00e4_5` reader - Reserved bit"]
pub type Rev00e4_5R = crate::BitReader;
#[doc = "Field `Rev_00e4_5` writer - Reserved bit"]
pub type Rev00e4_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Rev_00e4_6` reader - Reserved bit"]
pub type Rev00e4_6R = crate::BitReader;
#[doc = "Field `Rev_00e4_6` writer - Reserved bit"]
pub type Rev00e4_6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Rev_00e4_7` reader - Reserved bit"]
pub type Rev00e4_7R = crate::BitReader;
#[doc = "Field `Rev_00e4_7` writer - Reserved bit"]
pub type Rev00e4_7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Tx Clock Gated (rxc_out)"]
    #[inline(always)]
    pub fn txclk_en(&self) -> TxclkEnR {
        TxclkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Rx Clock Gated (txc_out)"]
    #[inline(always)]
    pub fn rxclk_en(&self) -> RxclkEnR {
        RxclkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Swap data order for dma_mst"]
    #[inline(always)]
    pub fn dma_data_swap(&self) -> DmaDataSwapR {
        DmaDataSwapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_3(&self) -> Rev00e4_3R {
        Rev00e4_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_4(&self) -> Rev00e4_4R {
        Rev00e4_4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_5(&self) -> Rev00e4_5R {
        Rev00e4_5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_6(&self) -> Rev00e4_6R {
        Rev00e4_6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_7(&self) -> Rev00e4_7R {
        Rev00e4_7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Tx Clock Gated (rxc_out)"]
    #[inline(always)]
    pub fn txclk_en(&mut self) -> TxclkEnW<'_, ClkCtrlRegSpec> {
        TxclkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Rx Clock Gated (txc_out)"]
    #[inline(always)]
    pub fn rxclk_en(&mut self) -> RxclkEnW<'_, ClkCtrlRegSpec> {
        RxclkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Swap data order for dma_mst"]
    #[inline(always)]
    pub fn dma_data_swap(&mut self) -> DmaDataSwapW<'_, ClkCtrlRegSpec> {
        DmaDataSwapW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_3(&mut self) -> Rev00e4_3W<'_, ClkCtrlRegSpec> {
        Rev00e4_3W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_4(&mut self) -> Rev00e4_4W<'_, ClkCtrlRegSpec> {
        Rev00e4_4W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_5(&mut self) -> Rev00e4_5W<'_, ClkCtrlRegSpec> {
        Rev00e4_5W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_6(&mut self) -> Rev00e4_6W<'_, ClkCtrlRegSpec> {
        Rev00e4_6W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved bit"]
    #[inline(always)]
    pub fn rev_00e4_7(&mut self) -> Rev00e4_7W<'_, ClkCtrlRegSpec> {
        Rev00e4_7W::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCtrlRegSpec;
impl crate::RegisterSpec for ClkCtrlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ctrl_reg::R`](R) reader structure"]
impl crate::Readable for ClkCtrlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ctrl_reg::W`](W) writer structure"]
impl crate::Writable for ClkCtrlRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Clk_Ctrl_REG to value 0x1f"]
impl crate::Resettable for ClkCtrlRegSpec {
    const RESET_VALUE: u32 = 0x1f;
}
