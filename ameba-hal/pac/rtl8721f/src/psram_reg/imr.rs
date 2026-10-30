#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `TXEIM` reader - Transmit FIFO empty interrupt masked. 1: spi_txeir is not masked. 0: spi_txeir is masked."]
pub type TxeimR = crate::BitReader;
#[doc = "Field `TXEIM` writer - Transmit FIFO empty interrupt masked. 1: spi_txeir is not masked. 0: spi_txeir is masked."]
pub type TxeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOIM` reader - Transmit FIFO overflow interrupt mask. 1: spi_txoir_r is not masked. 0: spi_txoir_r is masked."]
pub type TxoimR = crate::BitReader;
#[doc = "Field `TXOIM` writer - Transmit FIFO overflow interrupt mask. 1: spi_txoir_r is not masked. 0: spi_txoir_r is masked."]
pub type TxoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUIM` reader - Receive FIFO underflow interrupt masked. 1: spi_rxuir_r is not masked. 0: spi_rxuir_r is masked."]
pub type RxuimR = crate::BitReader;
#[doc = "Field `RXUIM` writer - Receive FIFO underflow interrupt masked. 1: spi_rxuir_r is not masked. 0: spi_rxuir_r is masked."]
pub type RxuimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOIM` reader - Receive FIFO overflow interrupt masked. 1: spi_rxoir_r is not masked. 0: spi_rxoir_r is masked."]
pub type RxoimR = crate::BitReader;
#[doc = "Field `RXOIM` writer - Receive FIFO overflow interrupt masked. 1: spi_rxoir_r is not masked. 0: spi_rxoir_r is masked."]
pub type RxoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIM` reader - Receive FIFO full interrupt mask. 1: spi_rxfir is not masked. 0: spi_rxfir is masked."]
pub type RxfimR = crate::BitReader;
#[doc = "Field `RXFIM` writer - Receive FIFO full interrupt mask. 1: spi_rxfir is not masked. 0: spi_rxfir is masked."]
pub type RxfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSEIM` reader - FIFO size error interrupt mask. 1: spi_fseir_r is not masked. 0: spi_fseir _r is masked"]
pub type FseimR = crate::BitReader;
#[doc = "Field `FSEIM` writer - FIFO size error interrupt mask. 1: spi_fseir_r is not masked. 0: spi_fseir _r is masked"]
pub type FseimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBEIM` reader - Write burst error interrupt mask. 1: spi_wbier_r is not masked. 0: spi_wbeir_r is masked."]
pub type WbeimR = crate::BitReader;
#[doc = "Field `WBEIM` writer - Write burst error interrupt mask. 1: spi_wbier_r is not masked. 0: spi_wbeir_r is masked."]
pub type WbeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYEIM` reader - The Byte-Enable error interrupt mask. 1: spi_byeir_r is not masked. 0: spi_byeir_r is masked."]
pub type ByeimR = crate::BitReader;
#[doc = "Field `BYEIM` writer - The Byte-Enable error interrupt mask. 1: spi_byeir_r is not masked. 0: spi_byeir_r is masked."]
pub type ByeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACEIM` reader - Auto-check timeout error interrupt mask. 1: spi_aceir_r is not masked. 0: spi_aceir_r is masked."]
pub type AceimR = crate::BitReader;
#[doc = "Field `ACEIM` writer - Auto-check timeout error interrupt mask. 1: spi_aceir_r is not masked. 0: spi_aceir_r is masked."]
pub type AceimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEIM` reader - User_mode error interrupt mask. 1: spi_useir_r is not masked. 0: spi_useir_r is masked."]
pub type UseimR = crate::BitReader;
#[doc = "Field `USEIM` writer - User_mode error interrupt mask. 1: spi_useir_r is not masked. 0: spi_useir_r is masked."]
pub type UseimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFSIM` reader - Transmit finish interrupt mask. 1: spi_tfsir_r is not masked. 0: spi_tfsir_r is masked."]
pub type TfsimR = crate::BitReader;
#[doc = "Field `TFSIM` writer - Transmit finish interrupt mask. 1: spi_tfsir_r is not masked. 0: spi_tfsir_r is masked."]
pub type TfsimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACSIM` reader - Auto-check Flash Status raw interrupt mask. 1: spi_acsir_r is not masked. 0: spi_acsis_r is masked."]
pub type AcsimR = crate::BitReader;
#[doc = "Field `ACSIM` writer - Auto-check Flash Status raw interrupt mask. 1: spi_acsir_r is not masked. 0: spi_acsis_r is masked."]
pub type AcsimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREIM` reader - DR timeout error interrupt mask. 1: spi_dreir_r is not masked. 0: spi_dreir_r is masked."]
pub type DreimR = crate::BitReader;
#[doc = "Field `DREIM` writer - DR timeout error interrupt mask. 1: spi_dreir_r is not masked. 0: spi_dreir_r is masked."]
pub type DreimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUIM` reader - Status FIFO underflow interrupt masked. 1: spi_stuir_r is not masked. 0: spi_stuir_r is masked. Available only if configuration of NAND_EN is defined."]
pub type StuimR = crate::BitReader;
#[doc = "Field `STUIM` writer - Status FIFO underflow interrupt masked. 1: spi_stuir_r is not masked. 0: spi_stuir_r is masked. Available only if configuration of NAND_EN is defined."]
pub type StuimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOIM` reader - Status FIFO overflow interrupt masked. 1: spi_stoir_r is not masked. 0: spi_stoir_r is masked. Available only if configuration of NAND_EN is defined."]
pub type StoimR = crate::BitReader;
#[doc = "Field `STOIM` writer - Status FIFO overflow interrupt masked. 1: spi_stoir_r is not masked. 0: spi_stoir_r is masked. Available only if configuration of NAND_EN is defined."]
pub type StoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STFIM` reader - Status FIFO full interrupt mask. 1: spi_stfir is not masked. 0: spi_stfir is masked. Available only if configuration of NAND_EN is defined."]
pub type StfimR = crate::BitReader;
#[doc = "Field `STFIM` writer - Status FIFO full interrupt mask. 1: spi_stfir is not masked. 0: spi_stfir is masked. Available only if configuration of NAND_EN is defined."]
pub type StfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWEIM` reader - NAND Flash write error interrupt mask. 1: spi_nweir_r is not masked. 0: spi_nweir_r is masked. Available only if configuration of NAND_EN is defined."]
pub type NweimR = crate::BitReader;
#[doc = "Field `NWEIM` writer - NAND Flash write error interrupt mask. 1: spi_nweir_r is not masked. 0: spi_nweir_r is masked. Available only if configuration of NAND_EN is defined."]
pub type NweimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEIM` reader - Nor Flash auto write error interrupt mask. 1: spi_aweir_r is not masked. 0: spi_aweir_r is masked. Exists: TAG >= 2.0.0."]
pub type AweimR = crate::BitReader;
#[doc = "Field `AWEIM` writer - Nor Flash auto write error interrupt mask. 1: spi_aweir_r is not masked. 0: spi_aweir_r is masked. Exists: TAG >= 2.0.0."]
pub type AweimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt masked. 1: spi_txeir is not masked. 0: spi_txeir is masked."]
    #[inline(always)]
    pub fn txeim(&self) -> TxeimR {
        TxeimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt mask. 1: spi_txoir_r is not masked. 0: spi_txoir_r is masked."]
    #[inline(always)]
    pub fn txoim(&self) -> TxoimR {
        TxoimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt masked. 1: spi_rxuir_r is not masked. 0: spi_rxuir_r is masked."]
    #[inline(always)]
    pub fn rxuim(&self) -> RxuimR {
        RxuimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt masked. 1: spi_rxoir_r is not masked. 0: spi_rxoir_r is masked."]
    #[inline(always)]
    pub fn rxoim(&self) -> RxoimR {
        RxoimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt mask. 1: spi_rxfir is not masked. 0: spi_rxfir is masked."]
    #[inline(always)]
    pub fn rxfim(&self) -> RxfimR {
        RxfimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO size error interrupt mask. 1: spi_fseir_r is not masked. 0: spi_fseir _r is masked"]
    #[inline(always)]
    pub fn fseim(&self) -> FseimR {
        FseimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write burst error interrupt mask. 1: spi_wbier_r is not masked. 0: spi_wbeir_r is masked."]
    #[inline(always)]
    pub fn wbeim(&self) -> WbeimR {
        WbeimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The Byte-Enable error interrupt mask. 1: spi_byeir_r is not masked. 0: spi_byeir_r is masked."]
    #[inline(always)]
    pub fn byeim(&self) -> ByeimR {
        ByeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto-check timeout error interrupt mask. 1: spi_aceir_r is not masked. 0: spi_aceir_r is masked."]
    #[inline(always)]
    pub fn aceim(&self) -> AceimR {
        AceimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - User_mode error interrupt mask. 1: spi_useir_r is not masked. 0: spi_useir_r is masked."]
    #[inline(always)]
    pub fn useim(&self) -> UseimR {
        UseimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit finish interrupt mask. 1: spi_tfsir_r is not masked. 0: spi_tfsir_r is masked."]
    #[inline(always)]
    pub fn tfsim(&self) -> TfsimR {
        TfsimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Auto-check Flash Status raw interrupt mask. 1: spi_acsir_r is not masked. 0: spi_acsis_r is masked."]
    #[inline(always)]
    pub fn acsim(&self) -> AcsimR {
        AcsimR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DR timeout error interrupt mask. 1: spi_dreir_r is not masked. 0: spi_dreir_r is masked."]
    #[inline(always)]
    pub fn dreim(&self) -> DreimR {
        DreimR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Status FIFO underflow interrupt masked. 1: spi_stuir_r is not masked. 0: spi_stuir_r is masked. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stuim(&self) -> StuimR {
        StuimR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Status FIFO overflow interrupt masked. 1: spi_stoir_r is not masked. 0: spi_stoir_r is masked. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stoim(&self) -> StoimR {
        StoimR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Status FIFO full interrupt mask. 1: spi_stfir is not masked. 0: spi_stfir is masked. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stfim(&self) -> StfimR {
        StfimR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAND Flash write error interrupt mask. 1: spi_nweir_r is not masked. 0: spi_nweir_r is masked. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn nweim(&self) -> NweimR {
        NweimR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Nor Flash auto write error interrupt mask. 1: spi_aweir_r is not masked. 0: spi_aweir_r is masked. Exists: TAG >= 2.0.0."]
    #[inline(always)]
    pub fn aweim(&self) -> AweimR {
        AweimR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO empty interrupt masked. 1: spi_txeir is not masked. 0: spi_txeir is masked."]
    #[inline(always)]
    pub fn txeim(&mut self) -> TxeimW<'_, ImrSpec> {
        TxeimW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt mask. 1: spi_txoir_r is not masked. 0: spi_txoir_r is masked."]
    #[inline(always)]
    pub fn txoim(&mut self) -> TxoimW<'_, ImrSpec> {
        TxoimW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt masked. 1: spi_rxuir_r is not masked. 0: spi_rxuir_r is masked."]
    #[inline(always)]
    pub fn rxuim(&mut self) -> RxuimW<'_, ImrSpec> {
        RxuimW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt masked. 1: spi_rxoir_r is not masked. 0: spi_rxoir_r is masked."]
    #[inline(always)]
    pub fn rxoim(&mut self) -> RxoimW<'_, ImrSpec> {
        RxoimW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt mask. 1: spi_rxfir is not masked. 0: spi_rxfir is masked."]
    #[inline(always)]
    pub fn rxfim(&mut self) -> RxfimW<'_, ImrSpec> {
        RxfimW::new(self, 4)
    }
    #[doc = "Bit 5 - FIFO size error interrupt mask. 1: spi_fseir_r is not masked. 0: spi_fseir _r is masked"]
    #[inline(always)]
    pub fn fseim(&mut self) -> FseimW<'_, ImrSpec> {
        FseimW::new(self, 5)
    }
    #[doc = "Bit 6 - Write burst error interrupt mask. 1: spi_wbier_r is not masked. 0: spi_wbeir_r is masked."]
    #[inline(always)]
    pub fn wbeim(&mut self) -> WbeimW<'_, ImrSpec> {
        WbeimW::new(self, 6)
    }
    #[doc = "Bit 7 - The Byte-Enable error interrupt mask. 1: spi_byeir_r is not masked. 0: spi_byeir_r is masked."]
    #[inline(always)]
    pub fn byeim(&mut self) -> ByeimW<'_, ImrSpec> {
        ByeimW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto-check timeout error interrupt mask. 1: spi_aceir_r is not masked. 0: spi_aceir_r is masked."]
    #[inline(always)]
    pub fn aceim(&mut self) -> AceimW<'_, ImrSpec> {
        AceimW::new(self, 8)
    }
    #[doc = "Bit 9 - User_mode error interrupt mask. 1: spi_useir_r is not masked. 0: spi_useir_r is masked."]
    #[inline(always)]
    pub fn useim(&mut self) -> UseimW<'_, ImrSpec> {
        UseimW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmit finish interrupt mask. 1: spi_tfsir_r is not masked. 0: spi_tfsir_r is masked."]
    #[inline(always)]
    pub fn tfsim(&mut self) -> TfsimW<'_, ImrSpec> {
        TfsimW::new(self, 10)
    }
    #[doc = "Bit 11 - Auto-check Flash Status raw interrupt mask. 1: spi_acsir_r is not masked. 0: spi_acsis_r is masked."]
    #[inline(always)]
    pub fn acsim(&mut self) -> AcsimW<'_, ImrSpec> {
        AcsimW::new(self, 11)
    }
    #[doc = "Bit 12 - DR timeout error interrupt mask. 1: spi_dreir_r is not masked. 0: spi_dreir_r is masked."]
    #[inline(always)]
    pub fn dreim(&mut self) -> DreimW<'_, ImrSpec> {
        DreimW::new(self, 12)
    }
    #[doc = "Bit 13 - Status FIFO underflow interrupt masked. 1: spi_stuir_r is not masked. 0: spi_stuir_r is masked. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stuim(&mut self) -> StuimW<'_, ImrSpec> {
        StuimW::new(self, 13)
    }
    #[doc = "Bit 14 - Status FIFO overflow interrupt masked. 1: spi_stoir_r is not masked. 0: spi_stoir_r is masked. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stoim(&mut self) -> StoimW<'_, ImrSpec> {
        StoimW::new(self, 14)
    }
    #[doc = "Bit 15 - Status FIFO full interrupt mask. 1: spi_stfir is not masked. 0: spi_stfir is masked. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn stfim(&mut self) -> StfimW<'_, ImrSpec> {
        StfimW::new(self, 15)
    }
    #[doc = "Bit 16 - NAND Flash write error interrupt mask. 1: spi_nweir_r is not masked. 0: spi_nweir_r is masked. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn nweim(&mut self) -> NweimW<'_, ImrSpec> {
        NweimW::new(self, 16)
    }
    #[doc = "Bit 17 - Nor Flash auto write error interrupt mask. 1: spi_aweir_r is not masked. 0: spi_aweir_r is masked. Exists: TAG >= 2.0.0."]
    #[inline(always)]
    pub fn aweim(&mut self) -> AweimW<'_, ImrSpec> {
        AweimW::new(self, 17)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0x01ee"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0x01ee;
}
