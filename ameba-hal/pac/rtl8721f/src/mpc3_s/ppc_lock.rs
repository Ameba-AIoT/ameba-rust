#[doc = "Register `PPC_LOCK` reader"]
pub type R = crate::R<PpcLockSpec>;
#[doc = "Register `PPC_LOCK` writer"]
pub type W = crate::W<PpcLockSpec>;
#[doc = "Field `LOCK` reader - PPC lock register. When writing 1'b1 to this nit, PPC will be locked. Note: It can not be written to 1'b0 again until system reset to prevent modification. When PPC is locked, you can not program PPC registers unless system reset."]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - PPC lock register. When writing 1'b1 to this nit, PPC will be locked. Note: It can not be written to 1'b0 again until system reset to prevent modification. When PPC is locked, you can not program PPC registers unless system reset."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPC lock register. When writing 1'b1 to this nit, PPC will be locked. Note: It can not be written to 1'b0 again until system reset to prevent modification. When PPC is locked, you can not program PPC registers unless system reset."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPC lock register. When writing 1'b1 to this nit, PPC will be locked. Note: It can not be written to 1'b0 again until system reset to prevent modification. When PPC is locked, you can not program PPC registers unless system reset."]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, PpcLockSpec> {
        LockW::new(self, 0)
    }
}
#[doc = "PPC Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpcLockSpec;
impl crate::RegisterSpec for PpcLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc_lock::R`](R) reader structure"]
impl crate::Readable for PpcLockSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc_lock::W`](W) writer structure"]
impl crate::Writable for PpcLockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC_LOCK to value 0"]
impl crate::Resettable for PpcLockSpec {}
