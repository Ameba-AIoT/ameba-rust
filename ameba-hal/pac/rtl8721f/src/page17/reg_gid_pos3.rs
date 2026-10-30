#[doc = "Register `REG_GID_POS3` reader"]
pub type R = crate::R<RegGidPos3Spec>;
#[doc = "Register `REG_GID_POS3` writer"]
pub type W = crate::W<RegGidPos3Spec>;
#[doc = "Field `r_gid_position3` reader - VHT group id position3"]
pub type RGidPosition3R = crate::FieldReader<u32>;
#[doc = "Field `r_gid_position3` writer - VHT group id position3"]
pub type RGidPosition3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VHT group id position3"]
    #[inline(always)]
    pub fn r_gid_position3(&self) -> RGidPosition3R {
        RGidPosition3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VHT group id position3"]
    #[inline(always)]
    pub fn r_gid_position3(&mut self) -> RGidPosition3W<'_, RegGidPos3Spec> {
        RGidPosition3W::new(self, 0)
    }
}
#[doc = "VHT group id position3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gid_pos3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_gid_pos3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGidPos3Spec;
impl crate::RegisterSpec for RegGidPos3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gid_pos3::R`](R) reader structure"]
impl crate::Readable for RegGidPos3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_gid_pos3::W`](W) writer structure"]
impl crate::Writable for RegGidPos3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GID_POS3 to value 0"]
impl crate::Resettable for RegGidPos3Spec {}
