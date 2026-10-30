#[doc = "Register `REG_BD_RWPTR_CLR` writer"]
pub type W = crate::W<RegBdRwptrClrSpec>;
#[doc = "Field `BIT_CLR_RXQ_HOST_IDX` writer - Clear RXQ host index, write 1 and hardware auto clear"]
pub type BitClrRxqHostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MGQ_HOST_IDX` writer - Clear MGQ host index, write 1 and hardware auto clear"]
pub type BitClrMgqHostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_VOQ_HOST_IDX` writer - Clear VOQ host index, write 1 and hardware auto clear"]
pub type BitClrVoqHostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_VIQ_HOST_IDX` writer - Clear VIQ host index, write 1 and hardware auto clear"]
pub type BitClrViqHostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_BEQ_HOST_IDX` writer - Clear BEQ host index, write 1 and hardware auto clear"]
pub type BitClrBeqHostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_BKQ_HOST_IDX` writer - Clear MQ host index, write 1 and hardware auto clear"]
pub type BitClrBkqHostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ0_HOST_IDX` writer - Clear MQ0 host index, write 1 and hardware auto clear"]
pub type BitClrMq0HostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ1_HOST_IDX` writer - Clear MQ1 host index, write 1 and hardware auto clear"]
pub type BitClrMq1HostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ2_HOST_IDX` writer - Clear MQ2 host index, write 1 and hardware auto clear"]
pub type BitClrMq2HostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ3_HOST_IDX` writer - Clear MQ3 host index, write 1 and hardware auto clear"]
pub type BitClrMq3HostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ4_HOST_IDX` writer - Clear MQ4 host index, write 1 and hardware auto clear"]
pub type BitClrMq4HostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ5_HOST_IDX` writer - Clear MQ5 host index, write 1 and hardware auto clear"]
pub type BitClrMq5HostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ6_HOST_IDX` writer - Clear MQ6 host index, write 1 and hardware auto clear"]
pub type BitClrMq6HostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ7_HOST_IDX` writer - Clear MQ7 host index, write 1 and hardware auto clear"]
pub type BitClrMq7HostIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SRST_RX_N` writer - Reset rx dma"]
pub type BitSrstRxNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SRST_TX_N` writer - Reset tx dma"]
pub type BitSrstTxNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_RXQ_HW_IDX` writer - Clear RXQ hardware index, write 1 and hardware auto clear"]
pub type BitClrRxqHwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MGQ_HW_IDX` writer - Clear MGQ hardware index, write 1 and hardware auto clear"]
pub type BitClrMgqHwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_VOQ_HW_IDX` writer - Clear VOQ hardware index, write 1 and hardware auto clear"]
pub type BitClrVoqHwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_VIQ_HW_IDX` writer - Clear VIQ hardware index, write 1 and hardware auto clear"]
pub type BitClrViqHwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_BEQ_HW_IDX` writer - Clear BEQ hardware index, write 1 and hardware auto clear"]
pub type BitClrBeqHwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_BKQ_HW_IDX` writer - Clear BKQ hardware index, write 1 and hardware auto clear"]
pub type BitClrBkqHwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ0_HW_IDX` writer - Clear MQ0 hardware index, write 1 and hardware auto clear"]
pub type BitClrMq0HwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ1_HW_IDX` writer - Clear MQ1 hardware index, write 1 and hardware auto clear"]
pub type BitClrMq1HwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ2_HW_IDX` writer - Clear MQ2 hardware index, write 1 and hardware auto clear"]
pub type BitClrMq2HwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ3_HW_IDX` writer - Clear MQ3 hardware index, write 1 and hardware auto clear"]
pub type BitClrMq3HwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ4_HW_IDX` writer - Clear MQ4 hardware index, write 1 and hardware auto clear"]
pub type BitClrMq4HwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ5_HW_IDX` writer - Clear MQ5 hardware index, write 1 and hardware auto clear"]
pub type BitClrMq5HwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ6_HW_IDX` writer - Clear MQ6 hardware index, write 1 and hardware auto clear"]
pub type BitClrMq6HwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLR_MQ7_HW_IDX` writer - Clear MQ7 hardware index, write 1 and hardware auto clear"]
pub type BitClrMq7HwIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SRST_RX_FSM` writer - Reset rx dma state"]
pub type BitSrstRxFsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SRST_TX_FSM` writer - Reset tx dma state"]
pub type BitSrstTxFsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear RXQ host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_rxq_host_idx(&mut self) -> BitClrRxqHostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrRxqHostIdxW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear MGQ host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mgq_host_idx(&mut self) -> BitClrMgqHostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMgqHostIdxW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear VOQ host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_voq_host_idx(&mut self) -> BitClrVoqHostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrVoqHostIdxW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear VIQ host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_viq_host_idx(&mut self) -> BitClrViqHostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrViqHostIdxW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear BEQ host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_beq_host_idx(&mut self) -> BitClrBeqHostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrBeqHostIdxW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear MQ host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_bkq_host_idx(&mut self) -> BitClrBkqHostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrBkqHostIdxW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear MQ0 host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq0_host_idx(&mut self) -> BitClrMq0HostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq0HostIdxW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear MQ1 host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq1_host_idx(&mut self) -> BitClrMq1HostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq1HostIdxW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear MQ2 host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq2_host_idx(&mut self) -> BitClrMq2HostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq2HostIdxW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear MQ3 host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq3_host_idx(&mut self) -> BitClrMq3HostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq3HostIdxW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear MQ4 host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq4_host_idx(&mut self) -> BitClrMq4HostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq4HostIdxW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear MQ5 host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq5_host_idx(&mut self) -> BitClrMq5HostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq5HostIdxW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear MQ6 host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq6_host_idx(&mut self) -> BitClrMq6HostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq6HostIdxW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear MQ7 host index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq7_host_idx(&mut self) -> BitClrMq7HostIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq7HostIdxW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset rx dma"]
    #[inline(always)]
    pub fn bit_srst_rx_n(&mut self) -> BitSrstRxNW<'_, RegBdRwptrClrSpec> {
        BitSrstRxNW::new(self, 14)
    }
    #[doc = "Bit 15 - Reset tx dma"]
    #[inline(always)]
    pub fn bit_srst_tx_n(&mut self) -> BitSrstTxNW<'_, RegBdRwptrClrSpec> {
        BitSrstTxNW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear RXQ hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_rxq_hw_idx(&mut self) -> BitClrRxqHwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrRxqHwIdxW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear MGQ hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mgq_hw_idx(&mut self) -> BitClrMgqHwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMgqHwIdxW::new(self, 17)
    }
    #[doc = "Bit 18 - Clear VOQ hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_voq_hw_idx(&mut self) -> BitClrVoqHwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrVoqHwIdxW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear VIQ hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_viq_hw_idx(&mut self) -> BitClrViqHwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrViqHwIdxW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear BEQ hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_beq_hw_idx(&mut self) -> BitClrBeqHwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrBeqHwIdxW::new(self, 20)
    }
    #[doc = "Bit 21 - Clear BKQ hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_bkq_hw_idx(&mut self) -> BitClrBkqHwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrBkqHwIdxW::new(self, 21)
    }
    #[doc = "Bit 22 - Clear MQ0 hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq0_hw_idx(&mut self) -> BitClrMq0HwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq0HwIdxW::new(self, 22)
    }
    #[doc = "Bit 23 - Clear MQ1 hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq1_hw_idx(&mut self) -> BitClrMq1HwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq1HwIdxW::new(self, 23)
    }
    #[doc = "Bit 24 - Clear MQ2 hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq2_hw_idx(&mut self) -> BitClrMq2HwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq2HwIdxW::new(self, 24)
    }
    #[doc = "Bit 25 - Clear MQ3 hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq3_hw_idx(&mut self) -> BitClrMq3HwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq3HwIdxW::new(self, 25)
    }
    #[doc = "Bit 26 - Clear MQ4 hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq4_hw_idx(&mut self) -> BitClrMq4HwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq4HwIdxW::new(self, 26)
    }
    #[doc = "Bit 27 - Clear MQ5 hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq5_hw_idx(&mut self) -> BitClrMq5HwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq5HwIdxW::new(self, 27)
    }
    #[doc = "Bit 28 - Clear MQ6 hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq6_hw_idx(&mut self) -> BitClrMq6HwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq6HwIdxW::new(self, 28)
    }
    #[doc = "Bit 29 - Clear MQ7 hardware index, write 1 and hardware auto clear"]
    #[inline(always)]
    pub fn bit_clr_mq7_hw_idx(&mut self) -> BitClrMq7HwIdxW<'_, RegBdRwptrClrSpec> {
        BitClrMq7HwIdxW::new(self, 29)
    }
    #[doc = "Bit 30 - Reset rx dma state"]
    #[inline(always)]
    pub fn bit_srst_rx_fsm(&mut self) -> BitSrstRxFsmW<'_, RegBdRwptrClrSpec> {
        BitSrstRxFsmW::new(self, 30)
    }
    #[doc = "Bit 31 - Reset tx dma state"]
    #[inline(always)]
    pub fn bit_srst_tx_fsm(&mut self) -> BitSrstTxFsmW<'_, RegBdRwptrClrSpec> {
        BitSrstTxFsmW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bd_rwptr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBdRwptrClrSpec;
impl crate::RegisterSpec for RegBdRwptrClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reg_bd_rwptr_clr::W`](W) writer structure"]
impl crate::Writable for RegBdRwptrClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BD_RWPTR_CLR to value 0xc000"]
impl crate::Resettable for RegBdRwptrClrSpec {
    const RESET_VALUE: u32 = 0xc000;
}
