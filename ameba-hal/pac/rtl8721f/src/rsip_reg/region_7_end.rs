#[doc = "Register `Region_7_end` reader"]
pub type R = crate::R<Region7EndSpec>;
#[doc = "Register `Region_7_end` writer"]
pub type W = crate::W<Region7EndSpec>;
#[doc = "Field `region_7_end` reader - Region 7 end address register."]
pub type Region7EndR = crate::FieldReader<u32>;
#[doc = "Field `region_7_end` writer - Region 7 end address register."]
pub type Region7EndW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 7 end address register."]
    #[inline(always)]
    pub fn region_7_end(&self) -> Region7EndR {
        Region7EndR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 7 end address register."]
    #[inline(always)]
    pub fn region_7_end(&mut self) -> Region7EndW<'_, Region7EndSpec> {
        Region7EndW::new(self, 5)
    }
}
#[doc = "Region 7 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_7_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_7_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region7EndSpec;
impl crate::RegisterSpec for Region7EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_7_end::R`](R) reader structure"]
impl crate::Readable for Region7EndSpec {}
#[doc = "`write(|w| ..)` method takes [`region_7_end::W`](W) writer structure"]
impl crate::Writable for Region7EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_7_end to value 0"]
impl crate::Resettable for Region7EndSpec {}
