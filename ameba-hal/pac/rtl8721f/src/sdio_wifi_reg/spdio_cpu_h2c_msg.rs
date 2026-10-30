#[doc = "Register `SPDIO_CPU_H2C_MSG` reader"]
pub type R = crate::R<SpdioCpuH2cMsgSpec>;
#[doc = "Field `CPU_H2C_MSG` reader - Driver to FW Message"]
pub type CpuH2cMsgR = crate::FieldReader<u32>;
#[doc = "Field `CPU_H2C_MSG_PS` reader - host write 1 to trigger interrupt to device"]
pub type CpuH2cMsgPsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Driver to FW Message"]
    #[inline(always)]
    pub fn cpu_h2c_msg(&self) -> CpuH2cMsgR {
        CpuH2cMsgR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - host write 1 to trigger interrupt to device"]
    #[inline(always)]
    pub fn cpu_h2c_msg_ps(&self) -> CpuH2cMsgPsR {
        CpuH2cMsgPsR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_cpu_h2c_msg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCpuH2cMsgSpec;
impl crate::RegisterSpec for SpdioCpuH2cMsgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_cpu_h2c_msg::R`](R) reader structure"]
impl crate::Readable for SpdioCpuH2cMsgSpec {}
#[doc = "`reset()` method sets SPDIO_CPU_H2C_MSG to value 0"]
impl crate::Resettable for SpdioCpuH2cMsgSpec {}
