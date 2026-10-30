#[doc = "Register `Region_7_base` reader"]
pub type R = crate::R<Region7BaseSpec>;
#[doc = "Register `Region_7_base` writer"]
pub type W = crate::W<Region7BaseSpec>;
#[doc = "Field `region_7_base` reader - Region 7 base address register."]
pub type Region7BaseR = crate::FieldReader<u32>;
#[doc = "Field `region_7_base` writer - Region 7 base address register."]
pub type Region7BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Region 7 base address register."]
    #[inline(always)]
    pub fn region_7_base(&self) -> Region7BaseR {
        Region7BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Region 7 base address register."]
    #[inline(always)]
    pub fn region_7_base(&mut self) -> Region7BaseW<'_, Region7BaseSpec> {
        Region7BaseW::new(self, 5)
    }
}
#[doc = "Region 7 base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_7_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_7_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region7BaseSpec;
impl crate::RegisterSpec for Region7BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_7_base::R`](R) reader structure"]
impl crate::Readable for Region7BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`region_7_base::W`](W) writer structure"]
impl crate::Writable for Region7BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_7_base to value 0"]
impl crate::Resettable for Region7BaseSpec {}
