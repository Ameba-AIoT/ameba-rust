#[doc = "Register `REG_GID_POS_EN_l` reader"]
pub type R = crate::R<RegGidPosEnLSpec>;
#[doc = "Register `REG_GID_POS_EN_l` writer"]
pub type W = crate::W<RegGidPosEnLSpec>;
#[doc = "Field `r_gid_position_en_l` reader - VHT group id position enable"]
pub type RGidPositionEnLR = crate::FieldReader<u32>;
#[doc = "Field `r_gid_position_en_l` writer - VHT group id position enable"]
pub type RGidPositionEnLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VHT group id position enable"]
    #[inline(always)]
    pub fn r_gid_position_en_l(&self) -> RGidPositionEnLR {
        RGidPositionEnLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VHT group id position enable"]
    #[inline(always)]
    pub fn r_gid_position_en_l(&mut self) -> RGidPositionEnLW<'_, RegGidPosEnLSpec> {
        RGidPositionEnLW::new(self, 0)
    }
}
#[doc = "VHT group id position enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gid_pos_en_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_gid_pos_en_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGidPosEnLSpec;
impl crate::RegisterSpec for RegGidPosEnLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gid_pos_en_l::R`](R) reader structure"]
impl crate::Readable for RegGidPosEnLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_gid_pos_en_l::W`](W) writer structure"]
impl crate::Writable for RegGidPosEnLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GID_POS_EN_l to value 0"]
impl crate::Resettable for RegGidPosEnLSpec {}
