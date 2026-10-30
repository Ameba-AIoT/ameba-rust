#[doc = "Register `IPC_SEM_CPUID1` reader"]
pub type R = crate::R<IpcSemCpuid1Spec>;
#[doc = "Field `sem_cpuid1_data` reader - it is used to indicate cpuid of this semaphore ."]
pub type SemCpuid1DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - it is used to indicate cpuid of this semaphore ."]
    #[inline(always)]
    pub fn sem_cpuid1_data(&self) -> SemCpuid1DataR {
        SemCpuid1DataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_sem_cpuid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcSemCpuid1Spec;
impl crate::RegisterSpec for IpcSemCpuid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_sem_cpuid1::R`](R) reader structure"]
impl crate::Readable for IpcSemCpuid1Spec {}
#[doc = "`reset()` method sets IPC_SEM_CPUID1 to value 0"]
impl crate::Resettable for IpcSemCpuid1Spec {}
