#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TXEIS` reader - Transmit FIFO empty raw interrupt status after masking. 1: spi_txeir is active after masking. 0: spi_txeir is not active after masking."]
pub type TxeisR = crate::BitReader;
#[doc = "Field `TXOIS` reader - Transmit FIFO overflow raw interrupt status after masking. 1: spi_txoir_r is active after masking. 0: spi_txoir_r is not active after masking."]
pub type TxoisR = crate::BitReader;
#[doc = "Field `RXUIS` reader - Receive FIFO underflows interrupt status after masking. 1: spi_rxuir_r is active after masking. 0: spi_rxuir_r is not active after masking."]
pub type RxuisR = crate::BitReader;
#[doc = "Field `RXOIS` reader - Receive FIFO overflow interrupt status after masking. 1: spi_rxoir_r is active after masking. 0: spi_rxoir_r is not active after masking."]
pub type RxoisR = crate::BitReader;
#[doc = "Field `RXFIS` reader - Receive FIFO full interrupt status after masking 1: spi_rxfir is active after masking. 0: spi_rxfir is not active after masking."]
pub type RxfisR = crate::BitReader;
#[doc = "Field `FSEIS` reader - FIFO size error interrupts status after masking. 1: spi_fesir_r is active after masking. 0: spi_fesir_r is not active after masking."]
pub type FseisR = crate::BitReader;
#[doc = "Field `WBEIS` reader - Write burst error interrupt status after masking. 1: spi_wbeir_r is active after masking. 0: spi_wbeir_r is not active after masking."]
pub type WbeisR = crate::BitReader;
#[doc = "Field `BYEIS` reader - The byte-Enable error interrupts status after masking. 1: spi_byeir_r is active after masking. 0: spi_byeir_r is not active after masking."]
pub type ByeisR = crate::BitReader;
#[doc = "Field `ACEIS` reader - Auto-check timeout error status after masking. 1: spi_aceir_r is active after masking. 0: spi_aceir_r is not active after masking."]
pub type AceisR = crate::BitReader;
#[doc = "Field `USEIS` reader - User mode error status after masking. 1: spi_useir_r is active after masking. 0: spi_useir_r is not active after masking."]
pub type UseisR = crate::BitReader;
#[doc = "Field `TFSIS` reader - Transmit finish interrupt status after masking. 1: spi_tfsir_r is active after masking. 0: spi_tfsir_r is not active after masking."]
pub type TfsisR = crate::BitReader;
#[doc = "Field `ACSIS` reader - Auto-check Flash Status after masking. 1: spi_aceir_r is active after masking. 0: spi_aceir_r is not active after masking."]
pub type AcsisR = crate::BitReader;
#[doc = "Field `DREIS` reader - DR timeout error status after masking. 1: spi_dreir_r is active after masking. 0: spi_dreir_r is not active after masking."]
pub type DreisR = crate::BitReader;
#[doc = "Field `STUIS` reader - Status FIFO underflows interrupt status after masking. 1: spi_stuir_r is active after masking. 0: spi_stuir_r is not active after masking. Available only if configuration of NAND_EN is defined."]
pub type StuisR = crate::BitReader;
#[doc = "Field `STOIS` reader - Status FIFO overflow interrupt status after masking. 1: spi_stoir_r is active after masking. 0: spi_stoir_r is not active after masking. Available only if configuration of NAND_EN is defined."]
pub type StoisR = crate::BitReader;
#[doc = "Field `STFIS` reader - Status FIFO full raw interrupt status after masking. 1: spi_stfir_r is active after masking. 0: spi_stfir_r is not active after masking. Available only if configuration of NAND_EN is defined."]
pub type StfisR = crate::BitReader;
#[doc = "Field `NWEIS` reader - NAND Flash write error interrupt status raw interrupt status after masking. 1: spi_nweir_r is active after masking. 0: spi_nweir_r is not active after masking. Available only if configuration of NAND_EN is defined."]
pub type NweisR = crate::BitReader;
#[doc = "Field `AWEIS` reader - Nor Flash auto write error interrupt status after masking. 1: spi_aweir_r is active after masking. 0: spi_aweir_r is not active after masking. Exists: TAG >= 2.0.0."]
pub type AweisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty raw interrupt status after masking. 1: spi_txeir is active after masking. 0: spi_txeir is not active after masking."]
    #[inline(always)]
    pub fn txeis(&self) -> TxeisR {
        TxeisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow raw interrupt status after masking. 1: spi_txoir_r is active after masking. 0: spi_txoir_r is not active after masking."]
    #[inline(always)]
    pub fn txois(&self) -> TxoisR {
        TxoisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflows interrupt status after masking. 1: spi_rxuir_r is active after masking. 0: spi_rxuir_r is not active after masking."]
    #[inline(always)]
    pub fn rxuis(&self) -> RxuisR {
        RxuisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt status after masking. 1: spi_rxoir_r is active after masking. 0: spi_rxoir_r is not active after masking."]
    #[inline(always)]
    pub fn rxois(&self) -> RxoisR {
        RxoisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt status after masking 1: spi_rxfir is active after masking. 0: spi_rxfir is not active after masking."]
    #[inline(always)]
    pub fn rxfis(&self) -> RxfisR {
        RxfisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO size error interrupts status after masking. 1: spi_fesir_r is active after masking. 0: spi_fesir_r is not active after masking."]
    #[inline(always)]
    pub fn fseis(&self) -> FseisR {
        FseisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write burst error interrupt status after masking. 1: spi_wbeir_r is active after masking. 0: spi_wbeir_r is not active after masking."]
    #[inline(always)]
    pub fn wbeis(&self) -> WbeisR {
        WbeisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The byte-Enable error interrupts status after masking. 1: spi_byeir_r is active after masking. 0: spi_byeir_r is not active after masking."]
    #[inline(always)]
    pub fn byeis(&self) -> ByeisR {
        ByeisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto-check timeout error status after masking. 1: spi_aceir_r is active after masking. 0: spi_aceir_r is not active after masking."]
    #[inline(always)]
    pub fn aceis(&self) -> AceisR {
        AceisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - User mode error status after masking. 1: spi_useir_r is active after masking. 0: spi_useir_r is not active after masking."]
    #[inline(always)]
    pub fn useis(&self) -> UseisR {
        UseisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit finish interrupt status after masking. 1: spi_tfsir_r is active after masking. 0: spi_tfsir_r is not active after masking."]
    #[inline(always)]
    pub fn tfsis(&self) -> TfsisR {
        TfsisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Auto-check Flash Status after masking. 1: spi_aceir_r is active after masking. 0: spi_aceir_r is not active after masking."]
    #[inline(always)]
    pub fn acsis(&self) -> AcsisR {
        AcsisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DR timeout error status after masking. 1: spi_dreir_r is active after masking. 0: spi_dreir_r is not active after masking."]
    #[inline(always)]
    pub fn dreis(&self) -> DreisR {
        DreisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Status FIFO underflows interrupt status after masking. 1: spi_stuir_r is active after masking. 0: spi_stuir_r is not active after masking. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stuis(&self) -> StuisR {
        StuisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Status FIFO overflow interrupt status after masking. 1: spi_stoir_r is active after masking. 0: spi_stoir_r is not active after masking. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stois(&self) -> StoisR {
        StoisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Status FIFO full raw interrupt status after masking. 1: spi_stfir_r is active after masking. 0: spi_stfir_r is not active after masking. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stfis(&self) -> StfisR {
        StfisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAND Flash write error interrupt status raw interrupt status after masking. 1: spi_nweir_r is active after masking. 0: spi_nweir_r is not active after masking. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn nweis(&self) -> NweisR {
        NweisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Nor Flash auto write error interrupt status after masking. 1: spi_aweir_r is active after masking. 0: spi_aweir_r is not active after masking. Exists: TAG >= 2.0.0."]
    #[inline(always)]
    pub fn aweis(&self) -> AweisR {
        AweisR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
