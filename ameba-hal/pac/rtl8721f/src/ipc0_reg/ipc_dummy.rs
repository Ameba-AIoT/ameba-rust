#[doc = "Register `IPC_DUMMY` reader"]
pub type R = crate::R<IpcDummySpec>;
#[doc = "Register `IPC_DUMMY` writer"]
pub type W = crate::W<IpcDummySpec>;
#[doc = "Field `dummy` reader - dummy reg"]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - dummy reg"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - dummy reg"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - dummy reg"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, IpcDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcDummySpec;
impl crate::RegisterSpec for IpcDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_dummy::R`](R) reader structure"]
impl crate::Readable for IpcDummySpec {}
#[doc = "`write(|w| ..)` method takes [`ipc_dummy::W`](W) writer structure"]
impl crate::Writable for IpcDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC_DUMMY to value 0"]
impl crate::Resettable for IpcDummySpec {}
