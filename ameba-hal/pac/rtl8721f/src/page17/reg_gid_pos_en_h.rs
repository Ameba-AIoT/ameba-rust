#[doc = "Register `REG_GID_POS_EN_H` reader"]
pub type R = crate::R<RegGidPosEnHSpec>;
#[doc = "Register `REG_GID_POS_EN_H` writer"]
pub type W = crate::W<RegGidPosEnHSpec>;
#[doc = "Field `r_gid_position_en_h` reader - VHT group id position enable"]
pub type RGidPositionEnHR = crate::FieldReader<u32>;
#[doc = "Field `r_gid_position_en_h` writer - VHT group id position enable"]
pub type RGidPositionEnHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VHT group id position enable"]
    #[inline(always)]
    pub fn r_gid_position_en_h(&self) -> RGidPositionEnHR {
        RGidPositionEnHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VHT group id position enable"]
    #[inline(always)]
    pub fn r_gid_position_en_h(&mut self) -> RGidPositionEnHW<'_, RegGidPosEnHSpec> {
        RGidPositionEnHW::new(self, 0)
    }
}
#[doc = "VHT group id position enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gid_pos_en_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_gid_pos_en_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGidPosEnHSpec;
impl crate::RegisterSpec for RegGidPosEnHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gid_pos_en_h::R`](R) reader structure"]
impl crate::Readable for RegGidPosEnHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_gid_pos_en_h::W`](W) writer structure"]
impl crate::Writable for RegGidPosEnHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GID_POS_EN_H to value 0"]
impl crate::Resettable for RegGidPosEnHSpec {}
