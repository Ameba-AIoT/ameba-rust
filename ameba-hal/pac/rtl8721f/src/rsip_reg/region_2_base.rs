#[doc = "Register `Region_2_base` reader"]
pub type R = crate::R<Region2BaseSpec>;
#[doc = "Register `Region_2_base` writer"]
pub type W = crate::W<Region2BaseSpec>;
#[doc = "Field `region_2_base` reader - Region 2 base address register."]
pub type Region2BaseR = crate::FieldReader<u32>;
#[doc = "Field `region_2_base` writer - Region 2 base address register."]
pub type Region2BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 2 base address register."]
    #[inline(always)]
    pub fn region_2_base(&self) -> Region2BaseR {
        Region2BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 2 base address register."]
    #[inline(always)]
    pub fn region_2_base(&mut self) -> Region2BaseW<'_, Region2BaseSpec> {
        Region2BaseW::new(self, 5)
    }
}
#[doc = "Region 2 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_2_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_2_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region2BaseSpec;
impl crate::RegisterSpec for Region2BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_2_base::R`](R) reader structure"]
impl crate::Readable for Region2BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`region_2_base::W`](W) writer structure"]
impl crate::Writable for Region2BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_2_base to value 0"]
impl crate::Resettable for Region2BaseSpec {}
