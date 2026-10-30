#[doc = "Register `Region_6_base` reader"]
pub type R = crate::R<Region6BaseSpec>;
#[doc = "Register `Region_6_base` writer"]
pub type W = crate::W<Region6BaseSpec>;
#[doc = "Field `region_6_base` reader - Region 6 base address register."]
pub type Region6BaseR = crate::FieldReader<u32>;
#[doc = "Field `region_6_base` writer - Region 6 base address register."]
pub type Region6BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 6 base address register."]
    #[inline(always)]
    pub fn region_6_base(&self) -> Region6BaseR {
        Region6BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 6 base address register."]
    #[inline(always)]
    pub fn region_6_base(&mut self) -> Region6BaseW<'_, Region6BaseSpec> {
        Region6BaseW::new(self, 5)
    }
}
#[doc = "Region 6 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_6_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_6_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region6BaseSpec;
impl crate::RegisterSpec for Region6BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_6_base::R`](R) reader structure"]
impl crate::Readable for Region6BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`region_6_base::W`](W) writer structure"]
impl crate::Writable for Region6BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_6_base to value 0"]
impl crate::Resettable for Region6BaseSpec {}
