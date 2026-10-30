#[doc = "Register `Region_4_end` reader"]
pub type R = crate::R<Region4EndSpec>;
#[doc = "Register `Region_4_end` writer"]
pub type W = crate::W<Region4EndSpec>;
#[doc = "Field `region_4_end` reader - Region 4 end address register."]
pub type Region4EndR = crate::FieldReader<u32>;
#[doc = "Field `region_4_end` writer - Region 4 end address register."]
pub type Region4EndW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 4 end address register."]
    #[inline(always)]
    pub fn region_4_end(&self) -> Region4EndR {
        Region4EndR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 4 end address register."]
    #[inline(always)]
    pub fn region_4_end(&mut self) -> Region4EndW<'_, Region4EndSpec> {
        Region4EndW::new(self, 5)
    }
}
#[doc = "Region 4 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_4_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_4_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region4EndSpec;
impl crate::RegisterSpec for Region4EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_4_end::R`](R) reader structure"]
impl crate::Readable for Region4EndSpec {}
#[doc = "`write(|w| ..)` method takes [`region_4_end::W`](W) writer structure"]
impl crate::Writable for Region4EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_4_end to value 0"]
impl crate::Resettable for Region4EndSpec {}
