#[doc = "Register `IPC_SEM_CPUID0` reader"]
pub type R = crate::R<IpcSemCpuid0Spec>;
#[doc = "Field `sem_cpuid0_data` reader - it is used to indicate cpuid of this semaphore ."]
pub type SemCpuid0DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - it is used to indicate cpuid of this semaphore ."]
    #[inline(always)]
    pub fn sem_cpuid0_data(&self) -> SemCpuid0DataR {
        SemCpuid0DataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_sem_cpuid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcSemCpuid0Spec;
impl crate::RegisterSpec for IpcSemCpuid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_sem_cpuid0::R`](R) reader structure"]
impl crate::Readable for IpcSemCpuid0Spec {}
#[doc = "`reset()` method sets IPC_SEM_CPUID0 to value 0"]
impl crate::Resettable for IpcSemCpuid0Spec {}
