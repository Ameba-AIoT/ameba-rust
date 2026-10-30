#[doc = "Register `IPC_SEM_1` reader"]
pub type R = crate::R<IpcSem1Spec>;
#[doc = "Field `sem_1_data` reader - it is used to indicate who owns this semaphore ."]
pub type Sem1DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - it is used to indicate who owns this semaphore ."]
    #[inline(always)]
    pub fn sem_1_data(&self) -> Sem1DataR {
        Sem1DataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_sem_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcSem1Spec;
impl crate::RegisterSpec for IpcSem1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_sem_1::R`](R) reader structure"]
impl crate::Readable for IpcSem1Spec {}
#[doc = "`reset()` method sets IPC_SEM_1 to value 0"]
impl crate::Resettable for IpcSem1Spec {}
