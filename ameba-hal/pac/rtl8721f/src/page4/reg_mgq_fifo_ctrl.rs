#[doc = "Register `REG_MGQ_FIFO_CTRL` reader"]
pub type R = crate::R<RegMgqFifoCtrlSpec>;
#[doc = "Register `REG_MGQ_FIFO_CTRL` writer"]
pub type W = crate::W<RegMgqFifoCtrlSpec>;
#[doc = "Field `BIT_MGQ_FIFO_WPTR` reader - The write pointer of the MGQ FIFO"]
pub type BitMgqFifoWptrR = crate::FieldReader;
#[doc = "Field `BIT_MGQ_FIFO_WPTR` writer - The write pointer of the MGQ FIFO"]
pub type BitMgqFifoWptrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_CPUM_LIFETIME_EN` reader - 1: Enable Lifetime function of the MGQ FIFO pkts. 0: Disable Lifetime function of the MGQ FIFO pkts."]
pub type BitCpumLifetimeEnR = crate::BitReader;
#[doc = "Field `BIT_CPUM_LIFETIME_EN` writer - 1: Enable Lifetime function of the MGQ FIFO pkts. 0: Disable Lifetime function of the MGQ FIFO pkts."]
pub type BitCpumLifetimeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MGQ_FIFO_WPTR_ERR` writer - 1: Error of MGQ FIFO write pointer setting. 0: No error of MGQ FIFO write pointer setting. (clear when disable MGQ FIFO function)"]
pub type BitMgqFifoWptrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MGQ_FIFO_OV` writer - 1: MGQ FIFO overflow. 0: NO MGQ FIFO overflow."]
pub type BitMgqFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MGQ_FIFO_RPTR` writer - The read pointer of the MGQ FIFO.(clear when disable MGQ FIFO function)"]
pub type BitMgqFifoRptrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_MGQ_FIFO_PAUSE` reader - 1: use all vo queue pktnum compare to pktnum_threshold; 0: use signle macid pktnum compare to pktnum_threshold;"]
pub type BitMgqFifoPauseR = crate::BitReader;
#[doc = "Field `BIT_MGQ_FIFO_PAUSE` writer - 1: use all vo queue pktnum compare to pktnum_threshold; 0: use signle macid pktnum compare to pktnum_threshold;"]
pub type BitMgqFifoPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MGQ_FIFO_SIZE` reader - 2'b00 : FIFO size 4 pkts. 2'b01 : FIFO size 8 pkts. 2'b1x : FIFO size 16 pkts."]
pub type BitMgqFifoSizeR = crate::FieldReader;
#[doc = "Field `BIT_MGQ_FIFO_SIZE` writer - 2'b00 : FIFO size 4 pkts. 2'b01 : FIFO size 8 pkts. 2'b1x : FIFO size 16 pkts."]
pub type BitMgqFifoSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MGQ_FIFO_START_PG` reader - The start page of MGQ FIFO."]
pub type BitMgqFifoStartPgR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MGQ_FIFO_START_PG` writer - The start page of MGQ FIFO."]
pub type BitMgqFifoStartPgW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MGQ_FIFO_PG_SIZE` reader - The page size of MGQ FIFO PKTs."]
pub type BitMgqFifoPgSizeR = crate::FieldReader;
#[doc = "Field `BIT_MGQ_FIFO_PG_SIZE` writer - The page size of MGQ FIFO PKTs."]
pub type BitMgqFifoPgSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_MGQ_FIFO_EN` reader - 1: Enable MGQ FIFO Function. 0: Disable MGQ FIFO Function."]
pub type BitMgqFifoEnR = crate::BitReader;
#[doc = "Field `BIT_MGQ_FIFO_EN` writer - 1: Enable MGQ FIFO Function. 0: Disable MGQ FIFO Function."]
pub type BitMgqFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - The write pointer of the MGQ FIFO"]
    #[inline(always)]
    pub fn bit_mgq_fifo_wptr(&self) -> BitMgqFifoWptrR {
        BitMgqFifoWptrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 1: Enable Lifetime function of the MGQ FIFO pkts. 0: Disable Lifetime function of the MGQ FIFO pkts."]
    #[inline(always)]
    pub fn bit_cpum_lifetime_en(&self) -> BitCpumLifetimeEnR {
        BitCpumLifetimeEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: use all vo queue pktnum compare to pktnum_threshold; 0: use signle macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_mgq_fifo_pause(&self) -> BitMgqFifoPauseR {
        BitMgqFifoPauseR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 2'b00 : FIFO size 4 pkts. 2'b01 : FIFO size 8 pkts. 2'b1x : FIFO size 16 pkts."]
    #[inline(always)]
    pub fn bit_mgq_fifo_size(&self) -> BitMgqFifoSizeR {
        BitMgqFifoSizeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:27 - The start page of MGQ FIFO."]
    #[inline(always)]
    pub fn bit_mgq_fifo_start_pg(&self) -> BitMgqFifoStartPgR {
        BitMgqFifoStartPgR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - The page size of MGQ FIFO PKTs."]
    #[inline(always)]
    pub fn bit_mgq_fifo_pg_size(&self) -> BitMgqFifoPgSizeR {
        BitMgqFifoPgSizeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 1: Enable MGQ FIFO Function. 0: Disable MGQ FIFO Function."]
    #[inline(always)]
    pub fn bit_mgq_fifo_en(&self) -> BitMgqFifoEnR {
        BitMgqFifoEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - The write pointer of the MGQ FIFO"]
    #[inline(always)]
    pub fn bit_mgq_fifo_wptr(&mut self) -> BitMgqFifoWptrW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoWptrW::new(self, 0)
    }
    #[doc = "Bit 5 - 1: Enable Lifetime function of the MGQ FIFO pkts. 0: Disable Lifetime function of the MGQ FIFO pkts."]
    #[inline(always)]
    pub fn bit_cpum_lifetime_en(&mut self) -> BitCpumLifetimeEnW<'_, RegMgqFifoCtrlSpec> {
        BitCpumLifetimeEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Error of MGQ FIFO write pointer setting. 0: No error of MGQ FIFO write pointer setting. (clear when disable MGQ FIFO function)"]
    #[inline(always)]
    pub fn bit_mgq_fifo_wptr_err(&mut self) -> BitMgqFifoWptrErrW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoWptrErrW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: MGQ FIFO overflow. 0: NO MGQ FIFO overflow."]
    #[inline(always)]
    pub fn bit_mgq_fifo_ov(&mut self) -> BitMgqFifoOvW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoOvW::new(self, 7)
    }
    #[doc = "Bits 8:12 - The read pointer of the MGQ FIFO.(clear when disable MGQ FIFO function)"]
    #[inline(always)]
    pub fn bit_mgq_fifo_rptr(&mut self) -> BitMgqFifoRptrW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoRptrW::new(self, 8)
    }
    #[doc = "Bit 13 - 1: use all vo queue pktnum compare to pktnum_threshold; 0: use signle macid pktnum compare to pktnum_threshold;"]
    #[inline(always)]
    pub fn bit_mgq_fifo_pause(&mut self) -> BitMgqFifoPauseW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoPauseW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 2'b00 : FIFO size 4 pkts. 2'b01 : FIFO size 8 pkts. 2'b1x : FIFO size 16 pkts."]
    #[inline(always)]
    pub fn bit_mgq_fifo_size(&mut self) -> BitMgqFifoSizeW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoSizeW::new(self, 14)
    }
    #[doc = "Bits 16:27 - The start page of MGQ FIFO."]
    #[inline(always)]
    pub fn bit_mgq_fifo_start_pg(&mut self) -> BitMgqFifoStartPgW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoStartPgW::new(self, 16)
    }
    #[doc = "Bits 28:30 - The page size of MGQ FIFO PKTs."]
    #[inline(always)]
    pub fn bit_mgq_fifo_pg_size(&mut self) -> BitMgqFifoPgSizeW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoPgSizeW::new(self, 28)
    }
    #[doc = "Bit 31 - 1: Enable MGQ FIFO Function. 0: Disable MGQ FIFO Function."]
    #[inline(always)]
    pub fn bit_mgq_fifo_en(&mut self) -> BitMgqFifoEnW<'_, RegMgqFifoCtrlSpec> {
        BitMgqFifoEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mgq_fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mgq_fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMgqFifoCtrlSpec;
impl crate::RegisterSpec for RegMgqFifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mgq_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for RegMgqFifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mgq_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for RegMgqFifoCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MGQ_FIFO_CTRL to value 0x1000_0000"]
impl crate::Resettable for RegMgqFifoCtrlSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
