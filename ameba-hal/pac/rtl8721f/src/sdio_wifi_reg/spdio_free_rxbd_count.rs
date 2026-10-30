#[doc = "Register `SPDIO_FREE_RXBD_COUNT` reader"]
pub type R = crate::R<SpdioFreeRxbdCountSpec>;
#[doc = "Register `SPDIO_FREE_RXBD_COUNT` writer"]
pub type W = crate::W<SpdioFreeRxbdCountSpec>;
#[doc = "Field `FIFO_CNT` reader - The SPDIO Local FIFO counter, for debug usage"]
pub type FifoCntR = crate::FieldReader;
#[doc = "Field `TX_BUFF_UNIT_SIZE` reader - The Size of each single TX Buffer which is addressed by TX_BD Unit: 64Byte Ex: 0x01=>64Byte 0x10=>1024Byte"]
pub type TxBuffUnitSizeR = crate::FieldReader;
#[doc = "Field `TX_BUFF_UNIT_SIZE` writer - The Size of each single TX Buffer which is addressed by TX_BD Unit: 64Byte Ex: 0x01=>64Byte 0x10=>1024Byte"]
pub type TxBuffUnitSizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FREE_RXBD_COUNT` reader - If SPDIO_RXBD_C2H_RPTR is updated and the free RXBD Number is larger than FREE_RXBD_COUNT, trigger RX_BD_AVAI_INT interrupt."]
pub type FreeRxbdCountR = crate::FieldReader<u16>;
#[doc = "Field `FREE_RXBD_COUNT` writer - If SPDIO_RXBD_C2H_RPTR is updated and the free RXBD Number is larger than FREE_RXBD_COUNT, trigger RX_BD_AVAI_INT interrupt."]
pub type FreeRxbdCountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - The SPDIO Local FIFO counter, for debug usage"]
    #[inline(always)]
    pub fn fifo_cnt(&self) -> FifoCntR {
        FifoCntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - The Size of each single TX Buffer which is addressed by TX_BD Unit: 64Byte Ex: 0x01=>64Byte 0x10=>1024Byte"]
    #[inline(always)]
    pub fn tx_buff_unit_size(&self) -> TxBuffUnitSizeR {
        TxBuffUnitSizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - If SPDIO_RXBD_C2H_RPTR is updated and the free RXBD Number is larger than FREE_RXBD_COUNT, trigger RX_BD_AVAI_INT interrupt."]
    #[inline(always)]
    pub fn free_rxbd_count(&self) -> FreeRxbdCountR {
        FreeRxbdCountR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - The Size of each single TX Buffer which is addressed by TX_BD Unit: 64Byte Ex: 0x01=>64Byte 0x10=>1024Byte"]
    #[inline(always)]
    pub fn tx_buff_unit_size(&mut self) -> TxBuffUnitSizeW<'_, SpdioFreeRxbdCountSpec> {
        TxBuffUnitSizeW::new(self, 8)
    }
    #[doc = "Bits 16:31 - If SPDIO_RXBD_C2H_RPTR is updated and the free RXBD Number is larger than FREE_RXBD_COUNT, trigger RX_BD_AVAI_INT interrupt."]
    #[inline(always)]
    pub fn free_rxbd_count(&mut self) -> FreeRxbdCountW<'_, SpdioFreeRxbdCountSpec> {
        FreeRxbdCountW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_free_rxbd_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_free_rxbd_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioFreeRxbdCountSpec;
impl crate::RegisterSpec for SpdioFreeRxbdCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_free_rxbd_count::R`](R) reader structure"]
impl crate::Readable for SpdioFreeRxbdCountSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_free_rxbd_count::W`](W) writer structure"]
impl crate::Writable for SpdioFreeRxbdCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_FREE_RXBD_COUNT to value 0x0001_0100"]
impl crate::Resettable for SpdioFreeRxbdCountSpec {
    const RESET_VALUE: u32 = 0x0001_0100;
}
