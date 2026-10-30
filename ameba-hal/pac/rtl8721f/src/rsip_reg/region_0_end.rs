#[doc = "Register `Region_0_end` reader"]
pub type R = crate::R<Region0EndSpec>;
#[doc = "Register `Region_0_end` writer"]
pub type W = crate::W<Region0EndSpec>;
#[doc = "Field `region_0_end` reader - Region 0 end address register."]
pub type Region0EndR = crate::FieldReader<u32>;
#[doc = "Field `region_0_end` writer - Region 0 end address register."]
pub type Region0EndW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 0 end address register."]
    #[inline(always)]
    pub fn region_0_end(&self) -> Region0EndR {
        Region0EndR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 0 end address register."]
    #[inline(always)]
    pub fn region_0_end(&mut self) -> Region0EndW<'_, Region0EndSpec> {
        Region0EndW::new(self, 5)
    }
}
#[doc = "Region 0 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_0_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_0_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region0EndSpec;
impl crate::RegisterSpec for Region0EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_0_end::R`](R) reader structure"]
impl crate::Readable for Region0EndSpec {}
#[doc = "`write(|w| ..)` method takes [`region_0_end::W`](W) writer structure"]
impl crate::Writable for Region0EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_0_end to value 0"]
impl crate::Resettable for Region0EndSpec {}
