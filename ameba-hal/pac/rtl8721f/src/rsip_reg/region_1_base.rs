#[doc = "Register `Region_1_base` reader"]
pub type R = crate::R<Region1BaseSpec>;
#[doc = "Register `Region_1_base` writer"]
pub type W = crate::W<Region1BaseSpec>;
#[doc = "Field `region_1_base` reader - Region 1 base address register."]
pub type Region1BaseR = crate::FieldReader<u32>;
#[doc = "Field `region_1_base` writer - Region 1 base address register."]
pub type Region1BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 1 base address register."]
    #[inline(always)]
    pub fn region_1_base(&self) -> Region1BaseR {
        Region1BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 1 base address register."]
    #[inline(always)]
    pub fn region_1_base(&mut self) -> Region1BaseW<'_, Region1BaseSpec> {
        Region1BaseW::new(self, 5)
    }
}
#[doc = "Region 1 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_1_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_1_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region1BaseSpec;
impl crate::RegisterSpec for Region1BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_1_base::R`](R) reader structure"]
impl crate::Readable for Region1BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`region_1_base::W`](W) writer structure"]
impl crate::Writable for Region1BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_1_base to value 0"]
impl crate::Resettable for Region1BaseSpec {}
