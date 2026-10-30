#[doc = "Register `REG_BOUNDARY_CHK` reader"]
pub type R = crate::R<RegBoundaryChkSpec>;
#[doc = "Register `REG_BOUNDARY_CHK` writer"]
pub type W = crate::W<RegBoundaryChkSpec>;
#[doc = "Field `dummy` reader - HW used only . Used for access boundary check, AON is 256B which is 000-FF"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - HW used only . Used for access boundary check, AON is 256B which is 000-FF"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW used only . Used for access boundary check, AON is 256B which is 000-FF"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW used only . Used for access boundary check, AON is 256B which is 000-FF"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegBoundaryChkSpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_boundary_chk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_boundary_chk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBoundaryChkSpec;
impl crate::RegisterSpec for RegBoundaryChkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_boundary_chk::R`](R) reader structure"]
impl crate::Readable for RegBoundaryChkSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_boundary_chk::W`](W) writer structure"]
impl crate::Writable for RegBoundaryChkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BOUNDARY_CHK to value 0xbdc0_0512"]
impl crate::Resettable for RegBoundaryChkSpec {
    const RESET_VALUE: u32 = 0xbdc0_0512;
}
