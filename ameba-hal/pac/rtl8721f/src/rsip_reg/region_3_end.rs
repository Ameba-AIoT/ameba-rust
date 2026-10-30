#[doc = "Register `Region_3_end` reader"]
pub type R = crate::R<Region3EndSpec>;
#[doc = "Register `Region_3_end` writer"]
pub type W = crate::W<Region3EndSpec>;
#[doc = "Field `region_3_end` reader - Region 3 end address register."]
pub type Region3EndR = crate::FieldReader<u32>;
#[doc = "Field `region_3_end` writer - Region 3 end address register."]
pub type Region3EndW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 3 end address register."]
    #[inline(always)]
    pub fn region_3_end(&self) -> Region3EndR {
        Region3EndR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 3 end address register."]
    #[inline(always)]
    pub fn region_3_end(&mut self) -> Region3EndW<'_, Region3EndSpec> {
        Region3EndW::new(self, 5)
    }
}
#[doc = "Region 3 end address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_3_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_3_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region3EndSpec;
impl crate::RegisterSpec for Region3EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_3_end::R`](R) reader structure"]
impl crate::Readable for Region3EndSpec {}
#[doc = "`write(|w| ..)` method takes [`region_3_end::W`](W) writer structure"]
impl crate::Writable for Region3EndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_3_end to value 0"]
impl crate::Resettable for Region3EndSpec {}
