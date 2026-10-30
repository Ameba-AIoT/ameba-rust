#[doc = "Register `IPC_SEM_CPUID3` reader"]
pub type R = crate::R<IpcSemCpuid3Spec>;
#[doc = "Field `sem_cpuid3_data` reader - it is used to indicate cpuid of this semaphore ."]
pub type SemCpuid3DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - it is used to indicate cpuid of this semaphore ."]
    #[inline(always)]
    pub fn sem_cpuid3_data(&self) -> SemCpuid3DataR {
        SemCpuid3DataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_sem_cpuid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcSemCpuid3Spec;
impl crate::RegisterSpec for IpcSemCpuid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_sem_cpuid3::R`](R) reader structure"]
impl crate::Readable for IpcSemCpuid3Spec {}
#[doc = "`reset()` method sets IPC_SEM_CPUID3 to value 0"]
impl crate::Resettable for IpcSemCpuid3Spec {}
