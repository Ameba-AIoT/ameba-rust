#[doc = "Register `Region_3_base` reader"]
pub type R = crate::R<Region3BaseSpec>;
#[doc = "Register `Region_3_base` writer"]
pub type W = crate::W<Region3BaseSpec>;
#[doc = "Field `region_3_base` reader - Region 3 base address register."]
pub type Region3BaseR = crate::FieldReader<u32>;
#[doc = "Field `region_3_base` writer - Region 3 base address register."]
pub type Region3BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 3 base address register."]
    #[inline(always)]
    pub fn region_3_base(&self) -> Region3BaseR {
        Region3BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 3 base address register."]
    #[inline(always)]
    pub fn region_3_base(&mut self) -> Region3BaseW<'_, Region3BaseSpec> {
        Region3BaseW::new(self, 5)
    }
}
#[doc = "Region 3 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_3_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_3_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region3BaseSpec;
impl crate::RegisterSpec for Region3BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_3_base::R`](R) reader structure"]
impl crate::Readable for Region3BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`region_3_base::W`](W) writer structure"]
impl crate::Writable for Region3BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_3_base to value 0"]
impl crate::Resettable for Region3BaseSpec {}
