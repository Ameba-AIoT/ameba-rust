#[doc = "Register `IPC_SEM_CPUID2` reader"]
pub type R = crate::R<IpcSemCpuid2Spec>;
#[doc = "Field `sem_cpuid2_data` reader - it is used to indicate cpuid of this semaphore ."]
pub type SemCpuid2DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - it is used to indicate cpuid of this semaphore ."]
    #[inline(always)]
    pub fn sem_cpuid2_data(&self) -> SemCpuid2DataR {
        SemCpuid2DataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_sem_cpuid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcSemCpuid2Spec;
impl crate::RegisterSpec for IpcSemCpuid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_sem_cpuid2::R`](R) reader structure"]
impl crate::Readable for IpcSemCpuid2Spec {}
#[doc = "`reset()` method sets IPC_SEM_CPUID2 to value 0"]
impl crate::Resettable for IpcSemCpuid2Spec {}
