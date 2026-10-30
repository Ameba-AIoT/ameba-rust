#[doc = "Register `IPC_SEM_0` reader"]
pub type R = crate::R<IpcSem0Spec>;
#[doc = "Field `sem_0_data` reader - it is used to indicate who owns this semaphore ."]
pub type Sem0DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - it is used to indicate who owns this semaphore ."]
    #[inline(always)]
    pub fn sem_0_data(&self) -> Sem0DataR {
        Sem0DataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_sem_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcSem0Spec;
impl crate::RegisterSpec for IpcSem0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_sem_0::R`](R) reader structure"]
impl crate::Readable for IpcSem0Spec {}
#[doc = "`reset()` method sets IPC_SEM_0 to value 0"]
impl crate::Resettable for IpcSem0Spec {}
