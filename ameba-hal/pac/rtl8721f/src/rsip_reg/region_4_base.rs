#[doc = "Register `Region_4_base` reader"]
pub type R = crate::R<Region4BaseSpec>;
#[doc = "Register `Region_4_base` writer"]
pub type W = crate::W<Region4BaseSpec>;
#[doc = "Field `region_4_base` reader - Region 4 base address register."]
pub type Region4BaseR = crate::FieldReader<u32>;
#[doc = "Field `region_4_base` writer - Region 4 base address register."]
pub type Region4BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 4 base address register."]
    #[inline(always)]
    pub fn region_4_base(&self) -> Region4BaseR {
        Region4BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 4 base address register."]
    #[inline(always)]
    pub fn region_4_base(&mut self) -> Region4BaseW<'_, Region4BaseSpec> {
        Region4BaseW::new(self, 5)
    }
}
#[doc = "Region 4 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_4_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_4_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region4BaseSpec;
impl crate::RegisterSpec for Region4BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_4_base::R`](R) reader structure"]
impl crate::Readable for Region4BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`region_4_base::W`](W) writer structure"]
impl crate::Writable for Region4BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_4_base to value 0"]
impl crate::Resettable for Region4BaseSpec {}
