#[doc = "Register `Region_1_end` reader"]
pub type R = crate::R<Region1EndSpec>;
#[doc = "Register `Region_1_end` writer"]
pub type W = crate::W<Region1EndSpec>;
#[doc = "Field `region_1_end` reader - Region 1 end address register."]
pub type Region1EndR = crate::FieldReader<u32>;
#[doc = "Field `region_1_end` writer - Region 1 end address register."]
pub type Region1EndW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 1 end address register."]
    #[inline(always)]
    pub fn region_1_end(&self) -> Region1EndR {
        Region1EndR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 1 end address register."]
    #[inline(always)]
    pub fn region_1_end(&mut self) -> Region1EndW<'_, Region1EndSpec> {
        Region1EndW::new(self, 5)
    }
}
#[doc = "Region 1 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_1_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_1_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region1EndSpec;
impl crate::RegisterSpec for Region1EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_1_end::R`](R) reader structure"]
impl crate::Readable for Region1EndSpec {}
#[doc = "`write(|w| ..)` method takes [`region_1_end::W`](W) writer structure"]
impl crate::Writable for Region1EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_1_end to value 0"]
impl crate::Resettable for Region1EndSpec {}
