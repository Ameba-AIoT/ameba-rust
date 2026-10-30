#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `BUSY` reader - SPIC busy flag. Set if SPIC is still transmitting to or receiving data from SPI Flash, or TX_FIFO/RX_FIFO are not empty. User should check BUSY=0 before setting 0x00\\[31\\]USER_MODE."]
pub type BusyR = crate::BitReader;
#[doc = "Field `TFNF` reader - Transmit FIFO is not full. Set if FIFO is not full in transmit mode, else it is cleared when FIFO is full."]
pub type TfnfR = crate::BitReader;
#[doc = "Field `TFE` reader - Transmit FIFO is empty. Set if FIFO is empty in transmit mode, else it is cleared when it has data in FIFO."]
pub type TfeR = crate::BitReader;
#[doc = "Field `RFNE` reader - Receive FIFO is not empty. Set if FIFO is not empty in receiving mode. This bit is cleared when read."]
pub type RfneR = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO full. Set if FIFO is full in receiving mode. This bit is cleared when read."]
pub type RffR = crate::BitReader;
#[doc = "Field `TXE` reader - Transmission error. Set if FIFO is empty and starting to transmit data to SPI Flash. This bit is cleared when read."]
pub type TxeR = crate::BitReader;
#[doc = "Field `DCOL` reader - Data Collision, or in Transmitting Status. 1: Status shows that SPIC is transmitting spi_flash_cmd/spi_flash_addr/spi_flash_data to SPI Flash. Suggest not reading DR during this transmitting state. (Check this status can avoid reading wrong data and cause SPIC error.)"]
pub type DcolR = crate::BitReader;
#[doc = "Field `BOOT_FIN` reader - Boot Finish. Set if count waiting cycles (Boot Delay Count) for SPI Flash becomes a stable state after power on (or system reset). 1: Boot Finish Note: Auto_mode would be blocked until boot finish. User cannot set CR until boot finish."]
pub type BootFinR = crate::BitReader;
#[doc = "Field `ATWR_RDSR_N` reader - The previous auto write cmd didn't check the status register (RDSR). User should check the status register of Flash before next user mode transaction."]
pub type AtwrRdsrNR = crate::BitReader;
#[doc = "Field `ATWR_RDSR_N` writer - The previous auto write cmd didn't check the status register (RDSR). User should check the status register of Flash before next user mode transaction."]
pub type AtwrRdsrNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDE_ST` reader - PDE state. Set if device has entered DPD/Sleep Mode Exists: TAG >= 2.0.7. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPIC busy flag. Set if SPIC is still transmitting to or receiving data from SPI Flash, or TX_FIFO/RX_FIFO are not empty. User should check BUSY=0 before setting 0x00\\[31\\]USER_MODE."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO is not full. Set if FIFO is not full in transmit mode, else it is cleared when FIFO is full."]
    #[inline(always)]
    pub fn tfnf(&self) -> TfnfR {
        TfnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO is empty. Set if FIFO is empty in transmit mode, else it is cleared when it has data in FIFO."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO is not empty. Set if FIFO is not empty in receiving mode. This bit is cleared when read."]
    #[inline(always)]
    pub fn rfne(&self) -> RfneR {
        RfneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full. Set if FIFO is full in receiving mode. This bit is cleared when read."]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission error. Set if FIFO is empty and starting to transmit data to SPI Flash. This bit is cleared when read."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Collision, or in Transmitting Status. 1: Status shows that SPIC is transmitting spi_flash_cmd/spi_flash_addr/spi_flash_data to SPI Flash. Suggest not reading DR during this transmitting state. (Check this status can avoid reading wrong data and cause SPIC error.)"]
    #[inline(always)]
    pub fn dcol(&self) -> DcolR {
        DcolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Boot Finish. Set if count waiting cycles (Boot Delay Count) for SPI Flash becomes a stable state after power on (or system reset). 1: Boot Finish Note: Auto_mode would be blocked until boot finish. User cannot set CR until boot finish."]
    #[inline(always)]
    pub fn boot_fin(&self) -> BootFinR {
        BootFinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The previous auto write cmd didn't check the status register (RDSR). User should check the status register of Flash before next user mode transaction."]
    #[inline(always)]
    pub fn atwr_rdsr_n(&self) -> AtwrRdsrNR {
        AtwrRdsrNR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - PDE state. Set if device has entered DPD/Sleep Mode Exists: TAG >= 2.0.7. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_st(&self) -> PdeStR {
        PdeStR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - The previous auto write cmd didn't check the status register (RDSR). User should check the status register of Flash before next user mode transaction."]
    #[inline(always)]
    pub fn atwr_rdsr_n(&mut self) -> AtwrRdsrNW<'_, SrSpec> {
        AtwrRdsrNW::new(self, 8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0x06"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x06;
}
