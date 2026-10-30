#[doc = "Register `Region_0_base` reader"]
pub type R = crate::R<Region0BaseSpec>;
#[doc = "Register `Region_0_base` writer"]
pub type W = crate::W<Region0BaseSpec>;
#[doc = "Field `region_0_base` reader - Region 0 base address register."]
pub type Region0BaseR = crate::FieldReader<u32>;
#[doc = "Field `region_0_base` writer - Region 0 base address register."]
pub type Region0BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 0 base address register."]
    #[inline(always)]
    pub fn region_0_base(&self) -> Region0BaseR {
        Region0BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 0 base address register."]
    #[inline(always)]
    pub fn region_0_base(&mut self) -> Region0BaseW<'_, Region0BaseSpec> {
        Region0BaseW::new(self, 5)
    }
}
#[doc = "Region 0 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_0_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_0_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region0BaseSpec;
impl crate::RegisterSpec for Region0BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_0_base::R`](R) reader structure"]
impl crate::Readable for Region0BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`region_0_base::W`](W) writer structure"]
impl crate::Writable for Region0BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_0_base to value 0"]
impl crate::Resettable for Region0BaseSpec {}
