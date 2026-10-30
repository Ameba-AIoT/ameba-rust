#[doc = "Register `Region_5_base` reader"]
pub type R = crate::R<Region5BaseSpec>;
#[doc = "Register `Region_5_base` writer"]
pub type W = crate::W<Region5BaseSpec>;
#[doc = "Field `region_5_base` reader - Region 5 base address register."]
pub type Region5BaseR = crate::FieldReader<u32>;
#[doc = "Field `region_5_base` writer - Region 5 base address register."]
pub type Region5BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 5 base address register."]
    #[inline(always)]
    pub fn region_5_base(&self) -> Region5BaseR {
        Region5BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 5 base address register."]
    #[inline(always)]
    pub fn region_5_base(&mut self) -> Region5BaseW<'_, Region5BaseSpec> {
        Region5BaseW::new(self, 5)
    }
}
#[doc = "Region 5 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_5_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_5_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region5BaseSpec;
impl crate::RegisterSpec for Region5BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_5_base::R`](R) reader structure"]
impl crate::Readable for Region5BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`region_5_base::W`](W) writer structure"]
impl crate::Writable for Region5BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_5_base to value 0"]
impl crate::Resettable for Region5BaseSpec {}
