#[doc = "Register `Region_6_end` reader"]
pub type R = crate::R<Region6EndSpec>;
#[doc = "Register `Region_6_end` writer"]
pub type W = crate::W<Region6EndSpec>;
#[doc = "Field `region_6_end` reader - Region 6 end address register."]
pub type Region6EndR = crate::FieldReader<u32>;
#[doc = "Field `region_6_end` writer - Region 6 end address register."]
pub type Region6EndW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 6 end address register."]
    #[inline(always)]
    pub fn region_6_end(&self) -> Region6EndR {
        Region6EndR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 6 end address register."]
    #[inline(always)]
    pub fn region_6_end(&mut self) -> Region6EndW<'_, Region6EndSpec> {
        Region6EndW::new(self, 5)
    }
}
#[doc = "Region 6 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_6_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_6_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region6EndSpec;
impl crate::RegisterSpec for Region6EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_6_end::R`](R) reader structure"]
impl crate::Readable for Region6EndSpec {}
#[doc = "`write(|w| ..)` method takes [`region_6_end::W`](W) writer structure"]
impl crate::Writable for Region6EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_6_end to value 0"]
impl crate::Resettable for Region6EndSpec {}
