#[doc = "Register `SPDIO_CPU_C2H_MSG_EXT` reader"]
pub type R = crate::R<SpdioCpuC2hMsgExtSpec>;
#[doc = "Register `SPDIO_CPU_C2H_MSG_EXT` writer"]
pub type W = crate::W<SpdioCpuC2hMsgExtSpec>;
#[doc = "Field `CPU_C2H_MSG_EXT` reader - FW to driver Message"]
pub type CpuC2hMsgExtR = crate::FieldReader<u32>;
#[doc = "Field `CPU_C2H_MSG_EXT` writer - FW to driver Message"]
pub type CpuC2hMsgExtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FW to driver Message"]
    #[inline(always)]
    pub fn cpu_c2h_msg_ext(&self) -> CpuC2hMsgExtR {
        CpuC2hMsgExtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FW to driver Message"]
    #[inline(always)]
    pub fn cpu_c2h_msg_ext(&mut self) -> CpuC2hMsgExtW<'_, SpdioCpuC2hMsgExtSpec> {
        CpuC2hMsgExtW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_cpu_c2h_msg_ext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_cpu_c2h_msg_ext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCpuC2hMsgExtSpec;
impl crate::RegisterSpec for SpdioCpuC2hMsgExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_cpu_c2h_msg_ext::R`](R) reader structure"]
impl crate::Readable for SpdioCpuC2hMsgExtSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_cpu_c2h_msg_ext::W`](W) writer structure"]
impl crate::Writable for SpdioCpuC2hMsgExtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_CPU_C2H_MSG_EXT to value 0"]
impl crate::Resettable for SpdioCpuC2hMsgExtSpec {}
