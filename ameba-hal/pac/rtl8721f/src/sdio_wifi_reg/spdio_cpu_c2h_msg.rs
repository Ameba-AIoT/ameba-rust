#[doc = "Register `SPDIO_CPU_C2H_MSG` reader"]
pub type R = crate::R<SpdioCpuC2hMsgSpec>;
#[doc = "Register `SPDIO_CPU_C2H_MSG` writer"]
pub type W = crate::W<SpdioCpuC2hMsgSpec>;
#[doc = "Field `CPU_C2H_MSG` reader - FW to Driver Message"]
pub type CpuC2hMsgR = crate::FieldReader<u32>;
#[doc = "Field `CPU_C2H_MSG` writer - FW to Driver Message"]
pub type CpuC2hMsgW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `CPU_C2H_MSG_PS` reader - device write 1 to trigger interrupt to host"]
pub type CpuC2hMsgPsR = crate::BitReader;
#[doc = "Field `CPU_C2H_MSG_PS` writer - device write 1 to trigger interrupt to host"]
pub type CpuC2hMsgPsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - FW to Driver Message"]
    #[inline(always)]
    pub fn cpu_c2h_msg(&self) -> CpuC2hMsgR {
        CpuC2hMsgR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - device write 1 to trigger interrupt to host"]
    #[inline(always)]
    pub fn cpu_c2h_msg_ps(&self) -> CpuC2hMsgPsR {
        CpuC2hMsgPsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - FW to Driver Message"]
    #[inline(always)]
    pub fn cpu_c2h_msg(&mut self) -> CpuC2hMsgW<'_, SpdioCpuC2hMsgSpec> {
        CpuC2hMsgW::new(self, 0)
    }
    #[doc = "Bit 31 - device write 1 to trigger interrupt to host"]
    #[inline(always)]
    pub fn cpu_c2h_msg_ps(&mut self) -> CpuC2hMsgPsW<'_, SpdioCpuC2hMsgSpec> {
        CpuC2hMsgPsW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_cpu_c2h_msg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_cpu_c2h_msg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCpuC2hMsgSpec;
impl crate::RegisterSpec for SpdioCpuC2hMsgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_cpu_c2h_msg::R`](R) reader structure"]
impl crate::Readable for SpdioCpuC2hMsgSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_cpu_c2h_msg::W`](W) writer structure"]
impl crate::Writable for SpdioCpuC2hMsgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_CPU_C2H_MSG to value 0"]
impl crate::Resettable for SpdioCpuC2hMsgSpec {}
