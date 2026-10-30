#[doc = "Register `SPDIO_CPU_INT_STATUS` reader"]
pub type R = crate::R<SpdioCpuIntStatusSpec>;
#[doc = "Register `SPDIO_CPU_INT_STATUS` writer"]
pub type W = crate::W<SpdioCpuIntStatusSpec>;
#[doc = "Field `TXBD_H2C_OVF` reader - If there is not enough TX_BD for TX Packet, this bit raise. Driver Write 1 clear"]
pub type TxbdH2cOvfR = crate::BitReader;
#[doc = "Field `TXBD_H2C_OVF` writer - If there is not enough TX_BD for TX Packet, this bit raise. Driver Write 1 clear"]
pub type TxbdH2cOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2C_BUS_RES_FAIL` reader - If SDIO is going to move packet to TX FIFO and fails to get bus resource, this interrupt raise. Driver Write 1 clear"]
pub type H2cBusResFailR = crate::BitReader;
#[doc = "Field `H2C_BUS_RES_FAIL` writer - If SDIO is going to move packet to TX FIFO and fails to get bus resource, this interrupt raise. Driver Write 1 clear"]
pub type H2cBusResFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2C_DMA_OK` reader - Trigger by SDIO to CPU that packet is sent from Host to TXFIFO"]
pub type H2cDmaOkR = crate::BitReader;
#[doc = "Field `H2C_DMA_OK` writer - Trigger by SDIO to CPU that packet is sent from Host to TXFIFO"]
pub type H2cDmaOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2H_DMA_OK` reader - Trigger by SDIO to CPU that packet is sent from RXFIFO to Host"]
pub type C2hDmaOkR = crate::BitReader;
#[doc = "Field `C2H_DMA_OK` writer - Trigger by SDIO to CPU that packet is sent from RXFIFO to Host"]
pub type C2hDmaOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_CS_RDY` reader - this bit will be triggered to inform CPU when sdio changing from standby state to command state(dat bus free)"]
pub type SdioCsRdyR = crate::BitReader;
#[doc = "Field `SDIO_CS_RDY` writer - this bit will be triggered to inform CPU when sdio changing from standby state to command state(dat bus free)"]
pub type SdioCsRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_CS_RST` reader - this bit will be triggered to inform CPU when sdio receiving CMD52 reset"]
pub type SdioCsRstR = crate::BitReader;
#[doc = "Field `SDIO_CS_RST` writer - this bit will be triggered to inform CPU when sdio receiving CMD52 reset"]
pub type SdioCsRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_RST_CMD_INT` reader - Trigger by SDIO to CPU when SDIO is reset. CPU should be able to receive this interrupt even CPU clock is gated; therefore, this interrupt should be connected to System On circuit."]
pub type SdioRstCmdIntR = crate::BitReader;
#[doc = "Field `SDIO_RST_CMD_INT` writer - Trigger by SDIO to CPU when SDIO is reset. CPU should be able to receive this interrupt even CPU clock is gated; therefore, this interrupt should be connected to System On circuit."]
pub type SdioRstCmdIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BD_FLAG_ERR_INT` reader - Trigger by SDIO to CPU when detect RX_BD error"]
pub type RxBdFlagErrIntR = crate::BitReader;
#[doc = "Field `RX_BD_FLAG_ERR_INT` writer - Trigger by SDIO to CPU when detect RX_BD error"]
pub type RxBdFlagErrIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BD_AVAI_INT` reader - If the free RXBD Number become larger than FREE_RXBD_COUNT(0x1D8), trigger this interrupt. This interrupt trigger only once when free RXBD number cross FREE_RXBD_COUNT"]
pub type RxBdAvaiIntR = crate::BitReader;
#[doc = "Field `RX_BD_AVAI_INT` writer - If the free RXBD Number become larger than FREE_RXBD_COUNT(0x1D8), trigger this interrupt. This interrupt trigger only once when free RXBD number cross FREE_RXBD_COUNT"]
pub type RxBdAvaiIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_WAKE_CPU_INT` reader - When Host Send CMD52 wakeup or TRX CMD53 while CPU is not ready (SYSTEM_CPU_RDY_IND=0, or CPU_RDY_IND=0), trigger this interrupt to wake CPU, and then indicate BUSY status to host. CPU should be able to receive this interrupt even CPU clock is gated; therefore, this interrupt should be connected to System On circuit."]
pub type HostWakeCpuIntR = crate::BitReader;
#[doc = "Field `HOST_WAKE_CPU_INT` writer - When Host Send CMD52 wakeup or TRX CMD53 while CPU is not ready (SYSTEM_CPU_RDY_IND=0, or CPU_RDY_IND=0), trigger this interrupt to wake CPU, and then indicate BUSY status to host. CPU should be able to receive this interrupt even CPU clock is gated; therefore, this interrupt should be connected to System On circuit."]
pub type HostWakeCpuIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_CMD11_INT` reader - When Host sends CMD11 to notify that the signal voltage level is going to switch to 1.8v, the HW should issue this interrupt to notify the local CPU to do the LDO power voltage level switch."]
pub type HostCmd11IntR = crate::BitReader;
#[doc = "Field `HOST_CMD11_INT` writer - When Host sends CMD11 to notify that the signal voltage level is going to switch to 1.8v, the HW should issue this interrupt to notify the local CPU to do the LDO power voltage level switch."]
pub type HostCmd11IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2C_SUS_REQ` reader - Host write sdio local reg to trigger suspend request interrupt"]
pub type H2cSusReqR = crate::BitReader;
#[doc = "Field `H2C_SUS_REQ` writer - Host write sdio local reg to trigger suspend request interrupt"]
pub type H2cSusReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCCR_IOE2_SET` reader - this bit will be triggered to inform CPU when host enable cccr fn2 ioe"]
pub type CccrIoe2SetR = crate::BitReader;
#[doc = "Field `CCCR_IOE2_SET` writer - this bit will be triggered to inform CPU when host enable cccr fn2 ioe"]
pub type CccrIoe2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCCR_IOE2_CLR` reader - this bit will be triggered to inform CPU when host disable cccr fn2 ioe"]
pub type CccrIoe2ClrR = crate::BitReader;
#[doc = "Field `CCCR_IOE2_CLR` writer - this bit will be triggered to inform CPU when host disable cccr fn2 ioe"]
pub type CccrIoe2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If there is not enough TX_BD for TX Packet, this bit raise. Driver Write 1 clear"]
    #[inline(always)]
    pub fn txbd_h2c_ovf(&self) -> TxbdH2cOvfR {
        TxbdH2cOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If SDIO is going to move packet to TX FIFO and fails to get bus resource, this interrupt raise. Driver Write 1 clear"]
    #[inline(always)]
    pub fn h2c_bus_res_fail(&self) -> H2cBusResFailR {
        H2cBusResFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger by SDIO to CPU that packet is sent from Host to TXFIFO"]
    #[inline(always)]
    pub fn h2c_dma_ok(&self) -> H2cDmaOkR {
        H2cDmaOkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger by SDIO to CPU that packet is sent from RXFIFO to Host"]
    #[inline(always)]
    pub fn c2h_dma_ok(&self) -> C2hDmaOkR {
        C2hDmaOkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - this bit will be triggered to inform CPU when sdio changing from standby state to command state(dat bus free)"]
    #[inline(always)]
    pub fn sdio_cs_rdy(&self) -> SdioCsRdyR {
        SdioCsRdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - this bit will be triggered to inform CPU when sdio receiving CMD52 reset"]
    #[inline(always)]
    pub fn sdio_cs_rst(&self) -> SdioCsRstR {
        SdioCsRstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Trigger by SDIO to CPU when SDIO is reset. CPU should be able to receive this interrupt even CPU clock is gated; therefore, this interrupt should be connected to System On circuit."]
    #[inline(always)]
    pub fn sdio_rst_cmd_int(&self) -> SdioRstCmdIntR {
        SdioRstCmdIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Trigger by SDIO to CPU when detect RX_BD error"]
    #[inline(always)]
    pub fn rx_bd_flag_err_int(&self) -> RxBdFlagErrIntR {
        RxBdFlagErrIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If the free RXBD Number become larger than FREE_RXBD_COUNT(0x1D8), trigger this interrupt. This interrupt trigger only once when free RXBD number cross FREE_RXBD_COUNT"]
    #[inline(always)]
    pub fn rx_bd_avai_int(&self) -> RxBdAvaiIntR {
        RxBdAvaiIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When Host Send CMD52 wakeup or TRX CMD53 while CPU is not ready (SYSTEM_CPU_RDY_IND=0, or CPU_RDY_IND=0), trigger this interrupt to wake CPU, and then indicate BUSY status to host. CPU should be able to receive this interrupt even CPU clock is gated; therefore, this interrupt should be connected to System On circuit."]
    #[inline(always)]
    pub fn host_wake_cpu_int(&self) -> HostWakeCpuIntR {
        HostWakeCpuIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When Host sends CMD11 to notify that the signal voltage level is going to switch to 1.8v, the HW should issue this interrupt to notify the local CPU to do the LDO power voltage level switch."]
    #[inline(always)]
    pub fn host_cmd11_int(&self) -> HostCmd11IntR {
        HostCmd11IntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Host write sdio local reg to trigger suspend request interrupt"]
    #[inline(always)]
    pub fn h2c_sus_req(&self) -> H2cSusReqR {
        H2cSusReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - this bit will be triggered to inform CPU when host enable cccr fn2 ioe"]
    #[inline(always)]
    pub fn cccr_ioe2_set(&self) -> CccrIoe2SetR {
        CccrIoe2SetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - this bit will be triggered to inform CPU when host disable cccr fn2 ioe"]
    #[inline(always)]
    pub fn cccr_ioe2_clr(&self) -> CccrIoe2ClrR {
        CccrIoe2ClrR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If there is not enough TX_BD for TX Packet, this bit raise. Driver Write 1 clear"]
    #[inline(always)]
    pub fn txbd_h2c_ovf(&mut self) -> TxbdH2cOvfW<'_, SpdioCpuIntStatusSpec> {
        TxbdH2cOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - If SDIO is going to move packet to TX FIFO and fails to get bus resource, this interrupt raise. Driver Write 1 clear"]
    #[inline(always)]
    pub fn h2c_bus_res_fail(&mut self) -> H2cBusResFailW<'_, SpdioCpuIntStatusSpec> {
        H2cBusResFailW::new(self, 1)
    }
    #[doc = "Bit 2 - Trigger by SDIO to CPU that packet is sent from Host to TXFIFO"]
    #[inline(always)]
    pub fn h2c_dma_ok(&mut self) -> H2cDmaOkW<'_, SpdioCpuIntStatusSpec> {
        H2cDmaOkW::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger by SDIO to CPU that packet is sent from RXFIFO to Host"]
    #[inline(always)]
    pub fn c2h_dma_ok(&mut self) -> C2hDmaOkW<'_, SpdioCpuIntStatusSpec> {
        C2hDmaOkW::new(self, 3)
    }
    #[doc = "Bit 5 - this bit will be triggered to inform CPU when sdio changing from standby state to command state(dat bus free)"]
    #[inline(always)]
    pub fn sdio_cs_rdy(&mut self) -> SdioCsRdyW<'_, SpdioCpuIntStatusSpec> {
        SdioCsRdyW::new(self, 5)
    }
    #[doc = "Bit 6 - this bit will be triggered to inform CPU when sdio receiving CMD52 reset"]
    #[inline(always)]
    pub fn sdio_cs_rst(&mut self) -> SdioCsRstW<'_, SpdioCpuIntStatusSpec> {
        SdioCsRstW::new(self, 6)
    }
    #[doc = "Bit 7 - Trigger by SDIO to CPU when SDIO is reset. CPU should be able to receive this interrupt even CPU clock is gated; therefore, this interrupt should be connected to System On circuit."]
    #[inline(always)]
    pub fn sdio_rst_cmd_int(&mut self) -> SdioRstCmdIntW<'_, SpdioCpuIntStatusSpec> {
        SdioRstCmdIntW::new(self, 7)
    }
    #[doc = "Bit 8 - Trigger by SDIO to CPU when detect RX_BD error"]
    #[inline(always)]
    pub fn rx_bd_flag_err_int(&mut self) -> RxBdFlagErrIntW<'_, SpdioCpuIntStatusSpec> {
        RxBdFlagErrIntW::new(self, 8)
    }
    #[doc = "Bit 9 - If the free RXBD Number become larger than FREE_RXBD_COUNT(0x1D8), trigger this interrupt. This interrupt trigger only once when free RXBD number cross FREE_RXBD_COUNT"]
    #[inline(always)]
    pub fn rx_bd_avai_int(&mut self) -> RxBdAvaiIntW<'_, SpdioCpuIntStatusSpec> {
        RxBdAvaiIntW::new(self, 9)
    }
    #[doc = "Bit 10 - When Host Send CMD52 wakeup or TRX CMD53 while CPU is not ready (SYSTEM_CPU_RDY_IND=0, or CPU_RDY_IND=0), trigger this interrupt to wake CPU, and then indicate BUSY status to host. CPU should be able to receive this interrupt even CPU clock is gated; therefore, this interrupt should be connected to System On circuit."]
    #[inline(always)]
    pub fn host_wake_cpu_int(&mut self) -> HostWakeCpuIntW<'_, SpdioCpuIntStatusSpec> {
        HostWakeCpuIntW::new(self, 10)
    }
    #[doc = "Bit 11 - When Host sends CMD11 to notify that the signal voltage level is going to switch to 1.8v, the HW should issue this interrupt to notify the local CPU to do the LDO power voltage level switch."]
    #[inline(always)]
    pub fn host_cmd11_int(&mut self) -> HostCmd11IntW<'_, SpdioCpuIntStatusSpec> {
        HostCmd11IntW::new(self, 11)
    }
    #[doc = "Bit 12 - Host write sdio local reg to trigger suspend request interrupt"]
    #[inline(always)]
    pub fn h2c_sus_req(&mut self) -> H2cSusReqW<'_, SpdioCpuIntStatusSpec> {
        H2cSusReqW::new(self, 12)
    }
    #[doc = "Bit 13 - this bit will be triggered to inform CPU when host enable cccr fn2 ioe"]
    #[inline(always)]
    pub fn cccr_ioe2_set(&mut self) -> CccrIoe2SetW<'_, SpdioCpuIntStatusSpec> {
        CccrIoe2SetW::new(self, 13)
    }
    #[doc = "Bit 14 - this bit will be triggered to inform CPU when host disable cccr fn2 ioe"]
    #[inline(always)]
    pub fn cccr_ioe2_clr(&mut self) -> CccrIoe2ClrW<'_, SpdioCpuIntStatusSpec> {
        CccrIoe2ClrW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_cpu_int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_cpu_int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCpuIntStatusSpec;
impl crate::RegisterSpec for SpdioCpuIntStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdio_cpu_int_status::R`](R) reader structure"]
impl crate::Readable for SpdioCpuIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_cpu_int_status::W`](W) writer structure"]
impl crate::Writable for SpdioCpuIntStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_CPU_INT_STATUS to value 0"]
impl crate::Resettable for SpdioCpuIntStatusSpec {}
