#[doc = "Register `REG_GID_POS2` reader"]
pub type R = crate::R<RegGidPos2Spec>;
#[doc = "Register `REG_GID_POS2` writer"]
pub type W = crate::W<RegGidPos2Spec>;
#[doc = "Field `r_gid_position2` reader - VHT group id position2"]
pub type RGidPosition2R = crate::FieldReader<u32>;
#[doc = "Field `r_gid_position2` writer - VHT group id position2"]
pub type RGidPosition2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VHT group id position2"]
    #[inline(always)]
    pub fn r_gid_position2(&self) -> RGidPosition2R {
        RGidPosition2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VHT group id position2"]
    #[inline(always)]
    pub fn r_gid_position2(&mut self) -> RGidPosition2W<'_, RegGidPos2Spec> {
        RGidPosition2W::new(self, 0)
    }
}
#[doc = "VHT group id position2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gid_pos2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_gid_pos2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGidPos2Spec;
impl crate::RegisterSpec for RegGidPos2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gid_pos2::R`](R) reader structure"]
impl crate::Readable for RegGidPos2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_gid_pos2::W`](W) writer structure"]
impl crate::Writable for RegGidPos2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GID_POS2 to value 0"]
impl crate::Resettable for RegGidPos2Spec {}
