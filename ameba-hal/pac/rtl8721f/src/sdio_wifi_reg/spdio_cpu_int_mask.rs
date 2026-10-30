#[doc = "Register `SPDIO_CPU_INT_MASK` reader"]
pub type R = crate::R<SpdioCpuIntMaskSpec>;
#[doc = "Register `SPDIO_CPU_INT_MASK` writer"]
pub type W = crate::W<SpdioCpuIntMaskSpec>;
#[doc = "Field `TXBD_H2C_OVF__MSK` reader - This bit is clear, and TXBD_H2C_OVF is masked"]
pub type TxbdH2cOvf_MskR = crate::BitReader;
#[doc = "Field `TXBD_H2C_OVF__MSK` writer - This bit is clear, and TXBD_H2C_OVF is masked"]
pub type TxbdH2cOvf_MskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2C_BUS_RES_FAIL_MSK` reader - This bit is clear, and H2C_BUS_RES_FAIL_INT is masked"]
pub type H2cBusResFailMskR = crate::BitReader;
#[doc = "Field `H2C_BUS_RES_FAIL_MSK` writer - This bit is clear, and H2C_BUS_RES_FAIL_INT is masked"]
pub type H2cBusResFailMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2C_DMA_OK_MSK` reader - This bit is clear, and H2C_DMA_OK _INT is masked"]
pub type H2cDmaOkMskR = crate::BitReader;
#[doc = "Field `H2C_DMA_OK_MSK` writer - This bit is clear, and H2C_DMA_OK _INT is masked"]
pub type H2cDmaOkMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2H_DMA_OK_MSK` reader - This bit is clear, and C2H_DMA_OK _INT is masked"]
pub type C2hDmaOkMskR = crate::BitReader;
#[doc = "Field `C2H_DMA_OK_MSK` writer - This bit is clear, and C2H_DMA_OK _INT is masked"]
pub type C2hDmaOkMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2C_MSG_INT_MSK` reader - This bit is clear, and H2C_MSG_INT is masked"]
pub type H2cMsgIntMskR = crate::BitReader;
#[doc = "Field `H2C_MSG_INT_MSK` writer - This bit is clear, and H2C_MSG_INT is masked"]
pub type H2cMsgIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPWM_INT_MSK` reader - This bit is clear, and RPWM_INT is masked"]
pub type RpwmIntMskR = crate::BitReader;
#[doc = "Field `RPWM_INT_MSK` writer - This bit is clear, and RPWM_INT is masked"]
pub type RpwmIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPWM2_INT_MSK` reader - This bit is clear, and RPWM2_INT is masked"]
pub type Rpwm2IntMskR = crate::BitReader;
#[doc = "Field `RPWM2_INT_MSK` writer - This bit is clear, and RPWM2_INT is masked"]
pub type Rpwm2IntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_RST_CMD_INT_MSK` reader - This bit is clear, and SDIO_RST_CMD_INT is masked"]
pub type SdioRstCmdIntMskR = crate::BitReader;
#[doc = "Field `SDIO_RST_CMD_INT_MSK` writer - This bit is clear, and SDIO_RST_CMD_INT is masked"]
pub type SdioRstCmdIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BD_FLAG_ERR_INT_MSK` reader - This bit is clear, and RX_BD_FLAG_ERR_INT is masked"]
pub type RxBdFlagErrIntMskR = crate::BitReader;
#[doc = "Field `RX_BD_FLAG_ERR_INT_MSK` writer - This bit is clear, and RX_BD_FLAG_ERR_INT is masked"]
pub type RxBdFlagErrIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BD_AVAI_INT_MSK` reader - This bit is clear, and RX_BD_AVAI_INT is masked"]
pub type RxBdAvaiIntMskR = crate::BitReader;
#[doc = "Field `RX_BD_AVAI_INT_MSK` writer - This bit is clear, and RX_BD_AVAI_INT is masked"]
pub type RxBdAvaiIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_WAKE_CPU_INT_MSK` reader - This bit is clear, and HOST_WAKE_CPU_INT is masked"]
pub type HostWakeCpuIntMskR = crate::BitReader;
#[doc = "Field `HOST_WAKE_CPU_INT_MSK` writer - This bit is clear, and HOST_WAKE_CPU_INT is masked"]
pub type HostWakeCpuIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_CMD11_INT_MSK` reader - This bit is clear, and HOST_CMD11_INT is masked"]
pub type HostCmd11IntMskR = crate::BitReader;
#[doc = "Field `HOST_CMD11_INT_MSK` writer - This bit is clear, and HOST_CMD11_INT is masked"]
pub type HostCmd11IntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H2C_SUS_REQ_MSK` reader - This bit is clear, and H2C_SUS_REQ is masked"]
pub type H2cSusReqMskR = crate::BitReader;
#[doc = "Field `H2C_SUS_REQ_MSK` writer - This bit is clear, and H2C_SUS_REQ is masked"]
pub type H2cSusReqMskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is clear, and TXBD_H2C_OVF is masked"]
    #[inline(always)]
    pub fn txbd_h2c_ovf__msk(&self) -> TxbdH2cOvf_MskR {
        TxbdH2cOvf_MskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is clear, and H2C_BUS_RES_FAIL_INT is masked"]
    #[inline(always)]
    pub fn h2c_bus_res_fail_msk(&self) -> H2cBusResFailMskR {
        H2cBusResFailMskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is clear, and H2C_DMA_OK _INT is masked"]
    #[inline(always)]
    pub fn h2c_dma_ok_msk(&self) -> H2cDmaOkMskR {
        H2cDmaOkMskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is clear, and C2H_DMA_OK _INT is masked"]
    #[inline(always)]
    pub fn c2h_dma_ok_msk(&self) -> C2hDmaOkMskR {
        C2hDmaOkMskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is clear, and H2C_MSG_INT is masked"]
    #[inline(always)]
    pub fn h2c_msg_int_msk(&self) -> H2cMsgIntMskR {
        H2cMsgIntMskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is clear, and RPWM_INT is masked"]
    #[inline(always)]
    pub fn rpwm_int_msk(&self) -> RpwmIntMskR {
        RpwmIntMskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is clear, and RPWM2_INT is masked"]
    #[inline(always)]
    pub fn rpwm2_int_msk(&self) -> Rpwm2IntMskR {
        Rpwm2IntMskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is clear, and SDIO_RST_CMD_INT is masked"]
    #[inline(always)]
    pub fn sdio_rst_cmd_int_msk(&self) -> SdioRstCmdIntMskR {
        SdioRstCmdIntMskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit is clear, and RX_BD_FLAG_ERR_INT is masked"]
    #[inline(always)]
    pub fn rx_bd_flag_err_int_msk(&self) -> RxBdFlagErrIntMskR {
        RxBdFlagErrIntMskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit is clear, and RX_BD_AVAI_INT is masked"]
    #[inline(always)]
    pub fn rx_bd_avai_int_msk(&self) -> RxBdAvaiIntMskR {
        RxBdAvaiIntMskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit is clear, and HOST_WAKE_CPU_INT is masked"]
    #[inline(always)]
    pub fn host_wake_cpu_int_msk(&self) -> HostWakeCpuIntMskR {
        HostWakeCpuIntMskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is clear, and HOST_CMD11_INT is masked"]
    #[inline(always)]
    pub fn host_cmd11_int_msk(&self) -> HostCmd11IntMskR {
        HostCmd11IntMskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is clear, and H2C_SUS_REQ is masked"]
    #[inline(always)]
    pub fn h2c_sus_req_msk(&self) -> H2cSusReqMskR {
        H2cSusReqMskR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is clear, and TXBD_H2C_OVF is masked"]
    #[inline(always)]
    pub fn txbd_h2c_ovf__msk(&mut self) -> TxbdH2cOvf_MskW<'_, SpdioCpuIntMaskSpec> {
        TxbdH2cOvf_MskW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is clear, and H2C_BUS_RES_FAIL_INT is masked"]
    #[inline(always)]
    pub fn h2c_bus_res_fail_msk(&mut self) -> H2cBusResFailMskW<'_, SpdioCpuIntMaskSpec> {
        H2cBusResFailMskW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is clear, and H2C_DMA_OK _INT is masked"]
    #[inline(always)]
    pub fn h2c_dma_ok_msk(&mut self) -> H2cDmaOkMskW<'_, SpdioCpuIntMaskSpec> {
        H2cDmaOkMskW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is clear, and C2H_DMA_OK _INT is masked"]
    #[inline(always)]
    pub fn c2h_dma_ok_msk(&mut self) -> C2hDmaOkMskW<'_, SpdioCpuIntMaskSpec> {
        C2hDmaOkMskW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is clear, and H2C_MSG_INT is masked"]
    #[inline(always)]
    pub fn h2c_msg_int_msk(&mut self) -> H2cMsgIntMskW<'_, SpdioCpuIntMaskSpec> {
        H2cMsgIntMskW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is clear, and RPWM_INT is masked"]
    #[inline(always)]
    pub fn rpwm_int_msk(&mut self) -> RpwmIntMskW<'_, SpdioCpuIntMaskSpec> {
        RpwmIntMskW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is clear, and RPWM2_INT is masked"]
    #[inline(always)]
    pub fn rpwm2_int_msk(&mut self) -> Rpwm2IntMskW<'_, SpdioCpuIntMaskSpec> {
        Rpwm2IntMskW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit is clear, and SDIO_RST_CMD_INT is masked"]
    #[inline(always)]
    pub fn sdio_rst_cmd_int_msk(&mut self) -> SdioRstCmdIntMskW<'_, SpdioCpuIntMaskSpec> {
        SdioRstCmdIntMskW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit is clear, and RX_BD_FLAG_ERR_INT is masked"]
    #[inline(always)]
    pub fn rx_bd_flag_err_int_msk(&mut self) -> RxBdFlagErrIntMskW<'_, SpdioCpuIntMaskSpec> {
        RxBdFlagErrIntMskW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit is clear, and RX_BD_AVAI_INT is masked"]
    #[inline(always)]
    pub fn rx_bd_avai_int_msk(&mut self) -> RxBdAvaiIntMskW<'_, SpdioCpuIntMaskSpec> {
        RxBdAvaiIntMskW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit is clear, and HOST_WAKE_CPU_INT is masked"]
    #[inline(always)]
    pub fn host_wake_cpu_int_msk(&mut self) -> HostWakeCpuIntMskW<'_, SpdioCpuIntMaskSpec> {
        HostWakeCpuIntMskW::new(self, 10)
    }
    #[doc = "Bit 11 - This bit is clear, and HOST_CMD11_INT is masked"]
    #[inline(always)]
    pub fn host_cmd11_int_msk(&mut self) -> HostCmd11IntMskW<'_, SpdioCpuIntMaskSpec> {
        HostCmd11IntMskW::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is clear, and H2C_SUS_REQ is masked"]
    #[inline(always)]
    pub fn h2c_sus_req_msk(&mut self) -> H2cSusReqMskW<'_, SpdioCpuIntMaskSpec> {
        H2cSusReqMskW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_cpu_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_cpu_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCpuIntMaskSpec;
impl crate::RegisterSpec for SpdioCpuIntMaskSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdio_cpu_int_mask::R`](R) reader structure"]
impl crate::Readable for SpdioCpuIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_cpu_int_mask::W`](W) writer structure"]
impl crate::Writable for SpdioCpuIntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_CPU_INT_MASK to value 0"]
impl crate::Resettable for SpdioCpuIntMaskSpec {}
