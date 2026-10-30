#[doc = "Register `Region_5_end` reader"]
pub type R = crate::R<Region5EndSpec>;
#[doc = "Register `Region_5_end` writer"]
pub type W = crate::W<Region5EndSpec>;
#[doc = "Field `region_5_end` reader - Region 5 end address register."]
pub type Region5EndR = crate::FieldReader<u32>;
#[doc = "Field `region_5_end` writer - Region 5 end address register."]
pub type Region5EndW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 5 end address register."]
    #[inline(always)]
    pub fn region_5_end(&self) -> Region5EndR {
        Region5EndR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 5 end address register."]
    #[inline(always)]
    pub fn region_5_end(&mut self) -> Region5EndW<'_, Region5EndSpec> {
        Region5EndW::new(self, 5)
    }
}
#[doc = "Region 5 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_5_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_5_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region5EndSpec;
impl crate::RegisterSpec for Region5EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_5_end::R`](R) reader structure"]
impl crate::Readable for Region5EndSpec {}
#[doc = "`write(|w| ..)` method takes [`region_5_end::W`](W) writer structure"]
impl crate::Writable for Region5EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_5_end to value 0"]
impl crate::Resettable for Region5EndSpec {}
