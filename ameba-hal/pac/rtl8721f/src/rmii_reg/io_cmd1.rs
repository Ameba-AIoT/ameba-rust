#[doc = "Register `IO_CMD1` reader"]
pub type R = crate::R<IoCmd1Spec>;
#[doc = "Register `IO_CMD1` writer"]
pub type W = crate::W<IoCmd1Spec>;
#[doc = "Field `txq1_h` reader - 1: TxFN1st is a high queue. 0: TxFN1st is a low queue. This field is only used in eee."]
pub type Txq1HR = crate::BitReader;
#[doc = "Field `txq1_h` writer - 1: TxFN1st is a high queue. 0: TxFN1st is a low queue. This field is only used in eee."]
pub type Txq1HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txq2_h` reader - 1: TxFN2nd is a high queue. 0: TxFN2nd is a low queue. This field is only used in eee."]
pub type Txq2HR = crate::BitReader;
#[doc = "Field `txq2_h` writer - 1: TxFN2nd is a high queue. 0: TxFN2nd is a low queue. This field is only used in eee."]
pub type Txq2HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txq3_h` reader - 1: TxFN3rd is a high queue. 0: TxFN3rd is a low queue. This field is only used in eee."]
pub type Txq3HR = crate::BitReader;
#[doc = "Field `txq3_h` writer - 1: TxFN3rd is a high queue. 0: TxFN3rd is a low queue. This field is only used in eee."]
pub type Txq3HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txq4_h` reader - 1: TxFN4th is a high queue. 0: TxFN4th is a low queue. This field is only used in eee."]
pub type Txq4HR = crate::BitReader;
#[doc = "Field `txq4_h` writer - 1: TxFN4th is a high queue. 0: TxFN4th is a low queue. This field is only used in eee."]
pub type Txq4HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txq5_h` reader - 1: TxFN5th is a high queue. 0: TxFN5th is a low queue. This field is only used in eee."]
pub type Txq5HR = crate::BitReader;
#[doc = "Field `txq5_h` writer - 1: TxFN5th is a high queue. 0: TxFN5th is a low queue. This field is only used in eee."]
pub type Txq5HW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFN5th` writer - 5th Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable. After IO_CMD.TE is set high, TxFN5th is writable."]
pub type TxFn5thW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Tx_hl_pri_sel` reader - b00: TX ring uses strict priority. b01: TX ring uses high and low queue priority. Inside high queue, tx ring is round robin. Inside low queue, tx ring is round robin. Strict priority is used for high and low queue selection. b10 and b11: reserved. In RL6266, when tx_hl_pri is 1, txq1_h and txq2_h are 1 and txq3_h, txq4_h and txq5_h are 0. SW should set this field before set IO_CMD.TE to 1. AmebaI/AmebaPro/AmebaPro2 only support the fixed value 2b'00"]
pub type TxHlPriSelR = crate::FieldReader;
#[doc = "Field `Tx_hl_pri_sel` writer - b00: TX ring uses strict priority. b01: TX ring uses high and low queue priority. Inside high queue, tx ring is round robin. Inside low queue, tx ring is round robin. Strict priority is used for high and low queue selection. b10 and b11: reserved. In RL6266, when tx_hl_pri is 1, txq1_h and txq2_h are 1 and txq3_h, txq4_h and txq5_h are 0. SW should set this field before set IO_CMD.TE to 1. AmebaI/AmebaPro/AmebaPro2 only support the fixed value 2b'00"]
pub type TxHlPriSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXRING1` writer - Ethernet-DMA Receive Ring1 enable. 1: Enable. 0: Disable. If RXRING1 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP1_reg."]
pub type Rxring1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRING2` writer - Ethernet-DMA Receive Ring2 enable. 1: Enable. 0: Disable. If RXRING2 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP2_reg."]
pub type Rxring2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRING3` writer - Ethernet-DMA Receive Ring3 enable. 1: Enable. 0: Disable. If RXRING3 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP3_reg."]
pub type Rxring3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRING4` writer - Ethernet-DMA Receive Ring4 enable. 1: Enable. 0: Disable. If RXRING4 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP4_reg."]
pub type Rxring4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRING5` writer - Ethernet-DMA Receive Ring5 enable. 1: Enable. 0: Disable. If RXRING5 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP5_reg."]
pub type Rxring5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRING6` writer - Ethernet-DMA Receive Ring6 enable. 1: Enable. 0: Disable. If RXRING6 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP6_reg."]
pub type Rxring6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_4GB` reader - 1: support 4GB addressing in lx master bus. For gmac used in rl0371 and after. 0: no support. For project used in rle0390 and before. SW should set this field before set IO_CMD.TE or RE to 1 AmebaPro2 support 4GB addressing AmebaI/AmebaPro only support 1GB addressing"]
pub type En4gbR = crate::BitReader;
#[doc = "Field `EN_4GB` writer - 1: support 4GB addressing in lx master bus. For gmac used in rl0371 and after. 0: no support. For project used in rle0390 and before. SW should set this field before set IO_CMD.TE or RE to 1 AmebaPro2 support 4GB addressing AmebaI/AmebaPro only support 1GB addressing"]
pub type En4gbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_RX_MRING` reader - Enable rx multiple rings. 1: rx using multiple rings. max: 6rings (ring1 to ring6). 0 rx using single ring (ring1 only). SW should set this field before set IO_CMD.TE or RE to 1 AmebaI/AmebaPro/AmebaPro2 only support single rx ring"]
pub type EnRxMringR = crate::BitReader;
#[doc = "Field `EN_RX_MRING` writer - Enable rx multiple rings. 1: rx using multiple rings. max: 6rings (ring1 to ring6). 0 rx using single ring (ring1 only). SW should set this field before set IO_CMD.TE or RE to 1 AmebaI/AmebaPro/AmebaPro2 only support single rx ring"]
pub type EnRxMringW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Rxokint_msk_128B` reader - 1: For ingress pkt which is short than 128B, RxOK interrupt asserts after DMA completes (compatible issue). 0: For ingress pkt which is short than 128B, RxOK interrupt does not assert after DMA completes."]
pub type RxokintMsk128bR = crate::BitReader;
#[doc = "Field `Rxokint_msk_128B` writer - 1: For ingress pkt which is short than 128B, RxOK interrupt asserts after DMA completes (compatible issue). 0: For ingress pkt which is short than 128B, RxOK interrupt does not assert after DMA completes."]
pub type RxokintMsk128bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_precise_dma` reader - 0: disable precise DMA transfer. 1: enable precise DMA transfer. AmebaI/AmebaPro/AmebaPro2 only support 1b'0 (disable precise DMA transfer)"]
pub type EnPreciseDmaR = crate::BitReader;
#[doc = "Field `en_precise_dma` writer - 0: disable precise DMA transfer. 1: enable precise DMA transfer. AmebaI/AmebaPro/AmebaPro2 only support 1b'0 (disable precise DMA transfer)"]
pub type EnPreciseDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Dsc_format_extra` reader - Extra descriptor format. In rle0437, bit28 is is write only. SW should set this field before set IO_CMD.TE or RE to 1. AmebaI/AmebaPro/AmebaPro 2 only support the fixed value 3b'011"]
pub type DscFormatExtraR = crate::FieldReader;
#[doc = "Field `Dsc_format_extra` writer - Extra descriptor format. In rle0437, bit28 is is write only. SW should set this field before set IO_CMD.TE or RE to 1. AmebaI/AmebaPro/AmebaPro 2 only support the fixed value 3b'011"]
pub type DscFormatExtraW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cf_orgcol_sel` reader - 0: using GMAC internal generated collision signal 1: using MII interface collision signal( (default 1: AmebaPro2)"]
pub type CfOrgcolSelR = crate::BitReader;
#[doc = "Field `cf_orgcol_sel` writer - 0: using GMAC internal generated collision signal 1: using MII interface collision signal( (default 1: AmebaPro2)"]
pub type CfOrgcolSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: TxFN1st is a high queue. 0: TxFN1st is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq1_h(&self) -> Txq1HR {
        Txq1HR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: TxFN2nd is a high queue. 0: TxFN2nd is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq2_h(&self) -> Txq2HR {
        Txq2HR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: TxFN3rd is a high queue. 0: TxFN3rd is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq3_h(&self) -> Txq3HR {
        Txq3HR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: TxFN4th is a high queue. 0: TxFN4th is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq4_h(&self) -> Txq4HR {
        Txq4HR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: TxFN5th is a high queue. 0: TxFN5th is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq5_h(&self) -> Txq5HR {
        Txq5HR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 14:15 - b00: TX ring uses strict priority. b01: TX ring uses high and low queue priority. Inside high queue, tx ring is round robin. Inside low queue, tx ring is round robin. Strict priority is used for high and low queue selection. b10 and b11: reserved. In RL6266, when tx_hl_pri is 1, txq1_h and txq2_h are 1 and txq3_h, txq4_h and txq5_h are 0. SW should set this field before set IO_CMD.TE to 1. AmebaI/AmebaPro/AmebaPro2 only support the fixed value 2b'00"]
    #[inline(always)]
    pub fn tx_hl_pri_sel(&self) -> TxHlPriSelR {
        TxHlPriSelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 24 - 1: support 4GB addressing in lx master bus. For gmac used in rl0371 and after. 0: no support. For project used in rle0390 and before. SW should set this field before set IO_CMD.TE or RE to 1 AmebaPro2 support 4GB addressing AmebaI/AmebaPro only support 1GB addressing"]
    #[inline(always)]
    pub fn en_4gb(&self) -> En4gbR {
        En4gbR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable rx multiple rings. 1: rx using multiple rings. max: 6rings (ring1 to ring6). 0 rx using single ring (ring1 only). SW should set this field before set IO_CMD.TE or RE to 1 AmebaI/AmebaPro/AmebaPro2 only support single rx ring"]
    #[inline(always)]
    pub fn en_rx_mring(&self) -> EnRxMringR {
        EnRxMringR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: For ingress pkt which is short than 128B, RxOK interrupt asserts after DMA completes (compatible issue). 0: For ingress pkt which is short than 128B, RxOK interrupt does not assert after DMA completes."]
    #[inline(always)]
    pub fn rxokint_msk_128b(&self) -> RxokintMsk128bR {
        RxokintMsk128bR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 0: disable precise DMA transfer. 1: enable precise DMA transfer. AmebaI/AmebaPro/AmebaPro2 only support 1b'0 (disable precise DMA transfer)"]
    #[inline(always)]
    pub fn en_precise_dma(&self) -> EnPreciseDmaR {
        EnPreciseDmaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Extra descriptor format. In rle0437, bit28 is is write only. SW should set this field before set IO_CMD.TE or RE to 1. AmebaI/AmebaPro/AmebaPro 2 only support the fixed value 3b'011"]
    #[inline(always)]
    pub fn dsc_format_extra(&self) -> DscFormatExtraR {
        DscFormatExtraR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 0: using GMAC internal generated collision signal 1: using MII interface collision signal( (default 1: AmebaPro2)"]
    #[inline(always)]
    pub fn cf_orgcol_sel(&self) -> CfOrgcolSelR {
        CfOrgcolSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: TxFN1st is a high queue. 0: TxFN1st is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq1_h(&mut self) -> Txq1HW<'_, IoCmd1Spec> {
        Txq1HW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: TxFN2nd is a high queue. 0: TxFN2nd is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq2_h(&mut self) -> Txq2HW<'_, IoCmd1Spec> {
        Txq2HW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: TxFN3rd is a high queue. 0: TxFN3rd is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq3_h(&mut self) -> Txq3HW<'_, IoCmd1Spec> {
        Txq3HW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: TxFN4th is a high queue. 0: TxFN4th is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq4_h(&mut self) -> Txq4HW<'_, IoCmd1Spec> {
        Txq4HW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: TxFN5th is a high queue. 0: TxFN5th is a low queue. This field is only used in eee."]
    #[inline(always)]
    pub fn txq5_h(&mut self) -> Txq5HW<'_, IoCmd1Spec> {
        Txq5HW::new(self, 4)
    }
    #[doc = "Bit 8 - 5th Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable. After IO_CMD.TE is set high, TxFN5th is writable."]
    #[inline(always)]
    pub fn tx_fn5th(&mut self) -> TxFn5thW<'_, IoCmd1Spec> {
        TxFn5thW::new(self, 8)
    }
    #[doc = "Bits 14:15 - b00: TX ring uses strict priority. b01: TX ring uses high and low queue priority. Inside high queue, tx ring is round robin. Inside low queue, tx ring is round robin. Strict priority is used for high and low queue selection. b10 and b11: reserved. In RL6266, when tx_hl_pri is 1, txq1_h and txq2_h are 1 and txq3_h, txq4_h and txq5_h are 0. SW should set this field before set IO_CMD.TE to 1. AmebaI/AmebaPro/AmebaPro2 only support the fixed value 2b'00"]
    #[inline(always)]
    pub fn tx_hl_pri_sel(&mut self) -> TxHlPriSelW<'_, IoCmd1Spec> {
        TxHlPriSelW::new(self, 14)
    }
    #[doc = "Bit 16 - Ethernet-DMA Receive Ring1 enable. 1: Enable. 0: Disable. If RXRING1 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP1_reg."]
    #[inline(always)]
    pub fn rxring1(&mut self) -> Rxring1W<'_, IoCmd1Spec> {
        Rxring1W::new(self, 16)
    }
    #[doc = "Bit 17 - Ethernet-DMA Receive Ring2 enable. 1: Enable. 0: Disable. If RXRING2 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP2_reg."]
    #[inline(always)]
    pub fn rxring2(&mut self) -> Rxring2W<'_, IoCmd1Spec> {
        Rxring2W::new(self, 17)
    }
    #[doc = "Bit 18 - Ethernet-DMA Receive Ring3 enable. 1: Enable. 0: Disable. If RXRING3 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP3_reg."]
    #[inline(always)]
    pub fn rxring3(&mut self) -> Rxring3W<'_, IoCmd1Spec> {
        Rxring3W::new(self, 18)
    }
    #[doc = "Bit 19 - Ethernet-DMA Receive Ring4 enable. 1: Enable. 0: Disable. If RXRING4 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP4_reg."]
    #[inline(always)]
    pub fn rxring4(&mut self) -> Rxring4W<'_, IoCmd1Spec> {
        Rxring4W::new(self, 19)
    }
    #[doc = "Bit 20 - Ethernet-DMA Receive Ring5 enable. 1: Enable. 0: Disable. If RXRING5 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP5_reg."]
    #[inline(always)]
    pub fn rxring5(&mut self) -> Rxring5W<'_, IoCmd1Spec> {
        Rxring5W::new(self, 20)
    }
    #[doc = "Bit 21 - Ethernet-DMA Receive Ring6 enable. 1: Enable. 0: Disable. If RXRING6 is not used, SW should put one dummy rx command descriptor (own = 0) at RXFDP6_reg."]
    #[inline(always)]
    pub fn rxring6(&mut self) -> Rxring6W<'_, IoCmd1Spec> {
        Rxring6W::new(self, 21)
    }
    #[doc = "Bit 24 - 1: support 4GB addressing in lx master bus. For gmac used in rl0371 and after. 0: no support. For project used in rle0390 and before. SW should set this field before set IO_CMD.TE or RE to 1 AmebaPro2 support 4GB addressing AmebaI/AmebaPro only support 1GB addressing"]
    #[inline(always)]
    pub fn en_4gb(&mut self) -> En4gbW<'_, IoCmd1Spec> {
        En4gbW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable rx multiple rings. 1: rx using multiple rings. max: 6rings (ring1 to ring6). 0 rx using single ring (ring1 only). SW should set this field before set IO_CMD.TE or RE to 1 AmebaI/AmebaPro/AmebaPro2 only support single rx ring"]
    #[inline(always)]
    pub fn en_rx_mring(&mut self) -> EnRxMringW<'_, IoCmd1Spec> {
        EnRxMringW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: For ingress pkt which is short than 128B, RxOK interrupt asserts after DMA completes (compatible issue). 0: For ingress pkt which is short than 128B, RxOK interrupt does not assert after DMA completes."]
    #[inline(always)]
    pub fn rxokint_msk_128b(&mut self) -> RxokintMsk128bW<'_, IoCmd1Spec> {
        RxokintMsk128bW::new(self, 26)
    }
    #[doc = "Bit 27 - 0: disable precise DMA transfer. 1: enable precise DMA transfer. AmebaI/AmebaPro/AmebaPro2 only support 1b'0 (disable precise DMA transfer)"]
    #[inline(always)]
    pub fn en_precise_dma(&mut self) -> EnPreciseDmaW<'_, IoCmd1Spec> {
        EnPreciseDmaW::new(self, 27)
    }
    #[doc = "Bits 28:30 - Extra descriptor format. In rle0437, bit28 is is write only. SW should set this field before set IO_CMD.TE or RE to 1. AmebaI/AmebaPro/AmebaPro 2 only support the fixed value 3b'011"]
    #[inline(always)]
    pub fn dsc_format_extra(&mut self) -> DscFormatExtraW<'_, IoCmd1Spec> {
        DscFormatExtraW::new(self, 28)
    }
    #[doc = "Bit 31 - 0: using GMAC internal generated collision signal 1: using MII interface collision signal( (default 1: AmebaPro2)"]
    #[inline(always)]
    pub fn cf_orgcol_sel(&mut self) -> CfOrgcolSelW<'_, IoCmd1Spec> {
        CfOrgcolSelW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`io_cmd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_cmd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoCmd1Spec;
impl crate::RegisterSpec for IoCmd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_cmd1::R`](R) reader structure"]
impl crate::Readable for IoCmd1Spec {}
#[doc = "`write(|w| ..)` method takes [`io_cmd1::W`](W) writer structure"]
impl crate::Writable for IoCmd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_CMD1 to value 0xb500_0000"]
impl crate::Resettable for IoCmd1Spec {
    const RESET_VALUE: u32 = 0xb500_0000;
}
