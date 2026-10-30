#[doc = "Register `SPDIO_SYS_CPU_INDICATION` reader"]
pub type R = crate::R<SpdioSysCpuIndicationSpec>;
#[doc = "Register `SPDIO_SYS_CPU_INDICATION` writer"]
pub type W = crate::W<SpdioSysCpuIndicationSpec>;
#[doc = "Field `SYSTEM_CPU_RDY_IND` reader - Used to indicate that CPU is ready for TRX. Default: 0 This bit is synchronized to CPU_RDY_IND(Offset: 0x87)"]
pub type SystemCpuRdyIndR = crate::BitReader;
#[doc = "Field `SYSTEM_CPU_RDY_IND` writer - Used to indicate that CPU is ready for TRX. Default: 0 This bit is synchronized to CPU_RDY_IND(Offset: 0x87)"]
pub type SystemCpuRdyIndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Used to indicate that CPU is ready for TRX. Default: 0 This bit is synchronized to CPU_RDY_IND(Offset: 0x87)"]
    #[inline(always)]
    pub fn system_cpu_rdy_ind(&self) -> SystemCpuRdyIndR {
        SystemCpuRdyIndR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to indicate that CPU is ready for TRX. Default: 0 This bit is synchronized to CPU_RDY_IND(Offset: 0x87)"]
    #[inline(always)]
    pub fn system_cpu_rdy_ind(&mut self) -> SystemCpuRdyIndW<'_, SpdioSysCpuIndicationSpec> {
        SystemCpuRdyIndW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_sys_cpu_indication::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_sys_cpu_indication::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioSysCpuIndicationSpec;
impl crate::RegisterSpec for SpdioSysCpuIndicationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_sys_cpu_indication::R`](R) reader structure"]
impl crate::Readable for SpdioSysCpuIndicationSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_sys_cpu_indication::W`](W) writer structure"]
impl crate::Writable for SpdioSysCpuIndicationSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_SYS_CPU_INDICATION to value 0"]
impl crate::Resettable for SpdioSysCpuIndicationSpec {}
