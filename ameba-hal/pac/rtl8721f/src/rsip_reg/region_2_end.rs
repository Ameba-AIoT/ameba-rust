#[doc = "Register `Region_2_end` reader"]
pub type R = crate::R<Region2EndSpec>;
#[doc = "Register `Region_2_end` writer"]
pub type W = crate::W<Region2EndSpec>;
#[doc = "Field `region_2_end` reader - Region 2 end address register."]
pub type Region2EndR = crate::FieldReader<u32>;
#[doc = "Field `region_2_end` writer - Region 2 end address register."]
pub type Region2EndW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 2 end address register."]
    #[inline(always)]
    pub fn region_2_end(&self) -> Region2EndR {
        Region2EndR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 2 end address register."]
    #[inline(always)]
    pub fn region_2_end(&mut self) -> Region2EndW<'_, Region2EndSpec> {
        Region2EndW::new(self, 5)
    }
}
#[doc = "Region 2 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_2_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_2_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region2EndSpec;
impl crate::RegisterSpec for Region2EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_2_end::R`](R) reader structure"]
impl crate::Readable for Region2EndSpec {}
#[doc = "`write(|w| ..)` method takes [`region_2_end::W`](W) writer structure"]
impl crate::Writable for Region2EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_2_end to value 0"]
impl crate::Resettable for Region2EndSpec {}
