#[doc = "Register `REG_MGQ_TXBD_RX_RXBD_NUM` reader"]
pub type R = crate::R<RegMgqTxbdRxRxbdNumSpec>;
#[doc = "Register `REG_MGQ_TXBD_RX_RXBD_NUM` writer"]
pub type W = crate::W<RegMgqTxbdRxRxbdNumSpec>;
#[doc = "Field `BIT_MGQDES_NUM` reader - MG queue descriptor's number"]
pub type BitMgqdesNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MGQDES_NUM` writer - MG queue descriptor's number"]
pub type BitMgqdesNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MGQ_DESC_MODE` reader - MGQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMgqDescModeR = crate::FieldReader;
#[doc = "Field `BIT_MGQ_DESC_MODE` writer - MGQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitMgqDescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MGQ_FLAG` reader - MGQ FLAGing bit, read result will be bpqflag"]
pub type BitMgqFlagR = crate::BitReader;
#[doc = "Field `BIT_RXDES_NUM` reader - RX queue descriptor's number"]
pub type BitRxdesNumR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXDES_NUM` writer - RX queue descriptor's number"]
pub type BitRxdesNumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_BCNQ_FLAG` reader - BCNQ FLAGing bit Read : BCNQ work Write : BCNQ Polling"]
pub type BitBcnqFlagR = crate::BitReader;
#[doc = "Field `BIT_BCNQ_FLAG` writer - BCNQ FLAGing bit Read : BCNQ work Write : BCNQ Polling"]
pub type BitBcnqFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCNQ_DESC_MODE` reader - BCNQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitBcnqDescModeR = crate::FieldReader;
#[doc = "Field `BIT_BCNQ_DESC_MODE` writer - BCNQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
pub type BitBcnqDescModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - MG queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mgqdes_num(&self) -> BitMgqdesNumR {
        BitMgqdesNumR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - MGQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mgq_desc_mode(&self) -> BitMgqDescModeR {
        BitMgqDescModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MGQ FLAGing bit, read result will be bpqflag"]
    #[inline(always)]
    pub fn bit_mgq_flag(&self) -> BitMgqFlagR {
        BitMgqFlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - RX queue descriptor's number"]
    #[inline(always)]
    pub fn bit_rxdes_num(&self) -> BitRxdesNumR {
        BitRxdesNumR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - BCNQ FLAGing bit Read : BCNQ work Write : BCNQ Polling"]
    #[inline(always)]
    pub fn bit_bcnq_flag(&self) -> BitBcnqFlagR {
        BitBcnqFlagR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - BCNQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_bcnq_desc_mode(&self) -> BitBcnqDescModeR {
        BitBcnqDescModeR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - MG queue descriptor's number"]
    #[inline(always)]
    pub fn bit_mgqdes_num(&mut self) -> BitMgqdesNumW<'_, RegMgqTxbdRxRxbdNumSpec> {
        BitMgqdesNumW::new(self, 0)
    }
    #[doc = "Bits 12:13 - MGQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_mgq_desc_mode(&mut self) -> BitMgqDescModeW<'_, RegMgqTxbdRxRxbdNumSpec> {
        BitMgqDescModeW::new(self, 12)
    }
    #[doc = "Bits 16:27 - RX queue descriptor's number"]
    #[inline(always)]
    pub fn bit_rxdes_num(&mut self) -> BitRxdesNumW<'_, RegMgqTxbdRxRxbdNumSpec> {
        BitRxdesNumW::new(self, 16)
    }
    #[doc = "Bit 28 - BCNQ FLAGing bit Read : BCNQ work Write : BCNQ Polling"]
    #[inline(always)]
    pub fn bit_bcnq_flag(&mut self) -> BitBcnqFlagW<'_, RegMgqTxbdRxRxbdNumSpec> {
        BitBcnqFlagW::new(self, 28)
    }
    #[doc = "Bits 29:30 - BCNQ descriptor mode : 0 : 2 segments each descriptor 1 : 4 segments each descriptor 2 : 8 segments each descriptor"]
    #[inline(always)]
    pub fn bit_bcnq_desc_mode(&mut self) -> BitBcnqDescModeW<'_, RegMgqTxbdRxRxbdNumSpec> {
        BitBcnqDescModeW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mgq_txbd_rx_rxbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mgq_txbd_rx_rxbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMgqTxbdRxRxbdNumSpec;
impl crate::RegisterSpec for RegMgqTxbdRxRxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mgq_txbd_rx_rxbd_num::R`](R) reader structure"]
impl crate::Readable for RegMgqTxbdRxRxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mgq_txbd_rx_rxbd_num::W`](W) writer structure"]
impl crate::Writable for RegMgqTxbdRxRxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MGQ_TXBD_RX_RXBD_NUM to value 0x0100_0100"]
impl crate::Resettable for RegMgqTxbdRxRxbdNumSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
