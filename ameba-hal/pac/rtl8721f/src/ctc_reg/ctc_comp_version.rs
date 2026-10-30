#[doc = "Register `CTC_COMP_VERSION` reader"]
pub type R = crate::R<CtcCompVersionSpec>;
#[doc = "Register `CTC_COMP_VERSION` writer"]
pub type W = crate::W<CtcCompVersionSpec>;
#[doc = "Field `comp_version` reader - CTC version number"]
pub type CompVersionR = crate::FieldReader<u32>;
#[doc = "Field `comp_version` writer - CTC version number"]
pub type CompVersionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CTC version number"]
    #[inline(always)]
    pub fn comp_version(&self) -> CompVersionR {
        CompVersionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CTC version number"]
    #[inline(always)]
    pub fn comp_version(&mut self) -> CompVersionW<'_, CtcCompVersionSpec> {
        CompVersionW::new(self, 0)
    }
}
#[doc = "Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_comp_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_comp_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcCompVersionSpec;
impl crate::RegisterSpec for CtcCompVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_comp_version::R`](R) reader structure"]
impl crate::Readable for CtcCompVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_comp_version::W`](W) writer structure"]
impl crate::Writable for CtcCompVersionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_COMP_VERSION to value 0x2018_0123"]
impl crate::Resettable for CtcCompVersionSpec {
    const RESET_VALUE: u32 = 0x2018_0123;
}
