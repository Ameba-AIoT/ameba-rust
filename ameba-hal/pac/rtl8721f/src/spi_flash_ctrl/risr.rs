#[doc = "Register `RISR` reader"]
pub type R = crate::R<RisrSpec>;
#[doc = "Field `TXEIR` reader - Transmit FIFO empty raw interrupt status prior to masking. 1: spi_txeir is active prior to masking. 0: spi_txeir is not active prior to masking."]
pub type TxeirR = crate::BitReader;
#[doc = "Field `TXOIR` reader - Transmit FIFO overflow raw interrupt status prior to masking. 1: spi_txoir_r is active prior to masking. 0: spi_txoir_r is not active prior to masking."]
pub type TxoirR = crate::BitReader;
#[doc = "Field `RXUIR` reader - Receive FIFO underflows raw interrupt status prior to masking. 1: spi_rxuir_r is active prior to masking. 0: spi_rxuir_r is not active prior to masking."]
pub type RxuirR = crate::BitReader;
#[doc = "Field `RXOIR` reader - Receive FIFO overflows raw interrupt status prior to masking. 1: spi_rxoir_r is active prior to masking. 0: spi_rxoir_r is not active prior to masking."]
pub type RxoirR = crate::BitReader;
#[doc = "Field `RXFIR` reader - Receive FIFO full raw interrupt status prior to masking 1:spi_rxfir is active prior to masking. 0: spi_rxfir is not active prior to masking."]
pub type RxfirR = crate::BitReader;
#[doc = "Field `FSEIR` reader - FIFO size error interrupt status raw interrupt status prior to masking. 1: spi_fseir_r is active prior to masking. 0: spi_fseir _r is not active prior to masking."]
pub type FseirR = crate::BitReader;
#[doc = "Field `WBEIR` reader - Write burst error interrupt status raw interrupt status prior to masking. 1: spi_wbeir_r is active prior to masking. 0: spi_wbeir_r is not active prior to masking."]
pub type WbeirR = crate::BitReader;
#[doc = "Field `BYEIR` reader - The Byte-Enable error interrupt status raw interrupt status prior to mask. 1: spi_byeir_r is active prior to masking. 0: spi_byeir_r is not active prior to masking."]
pub type ByeirR = crate::BitReader;
#[doc = "Field `ACEIR` reader - Auto-check timeout error status raw interrupt status prior to masking 1: spi_aceir_r is active prior to masking. 0: spi_aceir_r is not active prior to masking."]
pub type AceirR = crate::BitReader;
#[doc = "Field `USEIR` reader - User_mode error status raw interrupt status prior to masking 1: spi_useir_r is active prior to masking. 0: spi_useir_r is not active prior to masking."]
pub type UseirR = crate::BitReader;
#[doc = "Field `TFSIR` reader - Transmit Finish Status raw interrupt status prior to masking 1: spi_tfsir_r is active prior to masking. 0: spi_tfsir_r is not active prior to masking."]
pub type TfsirR = crate::BitReader;
#[doc = "Field `ACSIR` reader - Auto-check Flash Status raw interrupt status prior to masking 1: spi_acsir_r is active prior to masking. 0: spi_acsir_r is not active prior to masking."]
pub type AcsirR = crate::BitReader;
#[doc = "Field `DREIR` reader - DR Timeout error status raw interrupt status prior to masking 1: spi_dreir_r is active prior to masking. 0: spi_dreir_r is not active prior to masking."]
pub type DreirR = crate::BitReader;
#[doc = "Field `STUIR` reader - Status FIFO underflows raw interrupt status prior to masking. 1: spi_stuir_r is active prior to masking. 0: spi_stuir_r is not active prior to masking. Note: Available only if configuration of NAND_EN is defined."]
pub type StuirR = crate::BitReader;
#[doc = "Field `STOIR` reader - Status FIFO overflows raw interrupt status prior to masking. 1: spi_stoir_r is active prior to masking. 0: spi_stoir_r is not active prior to masking. Note: Available only if configuration of NAND_EN is defined."]
pub type StoirR = crate::BitReader;
#[doc = "Field `STFIR` reader - Status FIFO full raw interrupt status prior to masking 1: spi_stfir is active prior to masking. 0: spi_stfir is not active prior to masking. Note: Available only if configuration of NAND_EN is defined."]
pub type StfirR = crate::BitReader;
#[doc = "Field `NWEIR` reader - NAND Flash write error interrupt status raw interrupt status prior to masking. 1: spi_nweir_r is active prior to masking. 0: spi_nweir_r is not active prior to masking. Note: Available only if configuration of NAND_EN is defined."]
pub type NweirR = crate::BitReader;
#[doc = "Field `AWEIR` reader - Nor Flash auto write error interrupt status prior to masking. 1: spi_aweir_r is active prior to masking. 0: spi_aweir_r is not active prior to masking. Exists: TAG >= 2.0.0."]
pub type AweirR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty raw interrupt status prior to masking. 1: spi_txeir is active prior to masking. 0: spi_txeir is not active prior to masking."]
    #[inline(always)]
    pub fn txeir(&self) -> TxeirR {
        TxeirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow raw interrupt status prior to masking. 1: spi_txoir_r is active prior to masking. 0: spi_txoir_r is not active prior to masking."]
    #[inline(always)]
    pub fn txoir(&self) -> TxoirR {
        TxoirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflows raw interrupt status prior to masking. 1: spi_rxuir_r is active prior to masking. 0: spi_rxuir_r is not active prior to masking."]
    #[inline(always)]
    pub fn rxuir(&self) -> RxuirR {
        RxuirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflows raw interrupt status prior to masking. 1: spi_rxoir_r is active prior to masking. 0: spi_rxoir_r is not active prior to masking."]
    #[inline(always)]
    pub fn rxoir(&self) -> RxoirR {
        RxoirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full raw interrupt status prior to masking 1:spi_rxfir is active prior to masking. 0: spi_rxfir is not active prior to masking."]
    #[inline(always)]
    pub fn rxfir(&self) -> RxfirR {
        RxfirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO size error interrupt status raw interrupt status prior to masking. 1: spi_fseir_r is active prior to masking. 0: spi_fseir _r is not active prior to masking."]
    #[inline(always)]
    pub fn fseir(&self) -> FseirR {
        FseirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write burst error interrupt status raw interrupt status prior to masking. 1: spi_wbeir_r is active prior to masking. 0: spi_wbeir_r is not active prior to masking."]
    #[inline(always)]
    pub fn wbeir(&self) -> WbeirR {
        WbeirR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The Byte-Enable error interrupt status raw interrupt status prior to mask. 1: spi_byeir_r is active prior to masking. 0: spi_byeir_r is not active prior to masking."]
    #[inline(always)]
    pub fn byeir(&self) -> ByeirR {
        ByeirR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto-check timeout error status raw interrupt status prior to masking 1: spi_aceir_r is active prior to masking. 0: spi_aceir_r is not active prior to masking."]
    #[inline(always)]
    pub fn aceir(&self) -> AceirR {
        AceirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - User_mode error status raw interrupt status prior to masking 1: spi_useir_r is active prior to masking. 0: spi_useir_r is not active prior to masking."]
    #[inline(always)]
    pub fn useir(&self) -> UseirR {
        UseirR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Finish Status raw interrupt status prior to masking 1: spi_tfsir_r is active prior to masking. 0: spi_tfsir_r is not active prior to masking."]
    #[inline(always)]
    pub fn tfsir(&self) -> TfsirR {
        TfsirR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Auto-check Flash Status raw interrupt status prior to masking 1: spi_acsir_r is active prior to masking. 0: spi_acsir_r is not active prior to masking."]
    #[inline(always)]
    pub fn acsir(&self) -> AcsirR {
        AcsirR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DR Timeout error status raw interrupt status prior to masking 1: spi_dreir_r is active prior to masking. 0: spi_dreir_r is not active prior to masking."]
    #[inline(always)]
    pub fn dreir(&self) -> DreirR {
        DreirR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Status FIFO underflows raw interrupt status prior to masking. 1: spi_stuir_r is active prior to masking. 0: spi_stuir_r is not active prior to masking. Note: Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stuir(&self) -> StuirR {
        StuirR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Status FIFO overflows raw interrupt status prior to masking. 1: spi_stoir_r is active prior to masking. 0: spi_stoir_r is not active prior to masking. Note: Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stoir(&self) -> StoirR {
        StoirR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Status FIFO full raw interrupt status prior to masking 1: spi_stfir is active prior to masking. 0: spi_stfir is not active prior to masking. Note: Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stfir(&self) -> StfirR {
        StfirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAND Flash write error interrupt status raw interrupt status prior to masking. 1: spi_nweir_r is active prior to masking. 0: spi_nweir_r is not active prior to masking. Note: Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn nweir(&self) -> NweirR {
        NweirR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Nor Flash auto write error interrupt status prior to masking. 1: spi_aweir_r is active prior to masking. 0: spi_aweir_r is not active prior to masking. Exists: TAG >= 2.0.0."]
    #[inline(always)]
    pub fn aweir(&self) -> AweirR {
        AweirR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`risr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisrSpec;
impl crate::RegisterSpec for RisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`risr::R`](R) reader structure"]
impl crate::Readable for RisrSpec {}
#[doc = "`reset()` method sets RISR to value 0"]
impl crate::Resettable for RisrSpec {}
