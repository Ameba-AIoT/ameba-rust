#[doc = "Register `SPDIO_SYS_CPU_INDICATION` reader"]
pub type R = crate::R<SpdioSysCpuIndicationSpec>;
#[doc = "Register `SPDIO_SYS_CPU_INDICATION` writer"]
pub type W = crate::W<SpdioSysCpuIndicationSpec>;
#[doc = "Field `SYSTEM_CPU_RDY_IND` reader - Used to indicate that CPU is ready for TRX. Default: 0 This bit is synchronized to CPU_RDY_IND(Offset: 0x87)"]
pub type SystemCpuRdyIndR = crate::BitReader;
#[doc = "Field `SYSTEM_CPU_RDY_IND` writer - Used to indicate that CPU is ready for TRX. Default: 0 This bit is synchronized to CPU_RDY_IND(Offset: 0x87)"]
pub type SystemCpuRdyIndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READEE_DONE` reader - After cccr reg is configured, write this bit to 1"]
pub type ReadeeDoneR = crate::BitReader;
#[doc = "Field `READEE_DONE` writer - After cccr reg is configured, write this bit to 1"]
pub type ReadeeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FW_DL_DONE` reader - Host write firmware update done, sync to this bit"]
pub type FwDlDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Used to indicate that CPU is ready for TRX. Default: 0 This bit is synchronized to CPU_RDY_IND(Offset: 0x87)"]
    #[inline(always)]
    pub fn system_cpu_rdy_ind(&self) -> SystemCpuRdyIndR {
        SystemCpuRdyIndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - After cccr reg is configured, write this bit to 1"]
    #[inline(always)]
    pub fn readee_done(&self) -> ReadeeDoneR {
        ReadeeDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host write firmware update done, sync to this bit"]
    #[inline(always)]
    pub fn fw_dl_done(&self) -> FwDlDoneR {
        FwDlDoneR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to indicate that CPU is ready for TRX. Default: 0 This bit is synchronized to CPU_RDY_IND(Offset: 0x87)"]
    #[inline(always)]
    pub fn system_cpu_rdy_ind(&mut self) -> SystemCpuRdyIndW<'_, SpdioSysCpuIndicationSpec> {
        SystemCpuRdyIndW::new(self, 0)
    }
    #[doc = "Bit 1 - After cccr reg is configured, write this bit to 1"]
    #[inline(always)]
    pub fn readee_done(&mut self) -> ReadeeDoneW<'_, SpdioSysCpuIndicationSpec> {
        ReadeeDoneW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_sys_cpu_indication::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_sys_cpu_indication::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioSysCpuIndicationSpec;
impl crate::RegisterSpec for SpdioSysCpuIndicationSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spdio_sys_cpu_indication::R`](R) reader structure"]
impl crate::Readable for SpdioSysCpuIndicationSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_sys_cpu_indication::W`](W) writer structure"]
impl crate::Writable for SpdioSysCpuIndicationSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_SYS_CPU_INDICATION to value 0"]
impl crate::Resettable for SpdioSysCpuIndicationSpec {}
