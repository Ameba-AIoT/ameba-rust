#[doc = "Register `REG_BOOT_REASON_SW` reader"]
pub type R = crate::R<RegBootReasonSwSpec>;
#[doc = "Register `REG_BOOT_REASON_SW` writer"]
pub type W = crate::W<RegBootReasonSwSpec>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegBootReasonSwSpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_boot_reason_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_boot_reason_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBootReasonSwSpec;
impl crate::RegisterSpec for RegBootReasonSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_boot_reason_sw::R`](R) reader structure"]
impl crate::Readable for RegBootReasonSwSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_boot_reason_sw::W`](W) writer structure"]
impl crate::Writable for RegBootReasonSwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BOOT_REASON_SW to value 0"]
impl crate::Resettable for RegBootReasonSwSpec {}
