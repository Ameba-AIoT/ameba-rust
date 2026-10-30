#[doc = "Register `SPDIO_CPU_H2C_MSG_EXT` reader"]
pub type R = crate::R<SpdioCpuH2cMsgExtSpec>;
#[doc = "Field `CPU_H2C_MSG_EXT` reader - Driver to FW Message"]
pub type CpuH2cMsgExtR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Driver to FW Message"]
    #[inline(always)]
    pub fn cpu_h2c_msg_ext(&self) -> CpuH2cMsgExtR {
        CpuH2cMsgExtR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_cpu_h2c_msg_ext::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCpuH2cMsgExtSpec;
impl crate::RegisterSpec for SpdioCpuH2cMsgExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_cpu_h2c_msg_ext::R`](R) reader structure"]
impl crate::Readable for SpdioCpuH2cMsgExtSpec {}
#[doc = "`reset()` method sets SPDIO_CPU_H2C_MSG_EXT to value 0"]
impl crate::Resettable for SpdioCpuH2cMsgExtSpec {}
