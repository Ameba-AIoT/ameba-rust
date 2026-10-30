#[doc = "Register `reg_boot_reason` reader"]
pub type R = crate::R<RegBootReasonSpec>;
#[doc = "Register `reg_boot_reason` writer"]
pub type W = crate::W<RegBootReasonSpec>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegBootReasonSpec> {
        RsvdW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_boot_reason::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_boot_reason::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBootReasonSpec;
impl crate::RegisterSpec for RegBootReasonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_boot_reason::R`](R) reader structure"]
impl crate::Readable for RegBootReasonSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_boot_reason::W`](W) writer structure"]
impl crate::Writable for RegBootReasonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets reg_boot_reason to value 0"]
impl crate::Resettable for RegBootReasonSpec {}
