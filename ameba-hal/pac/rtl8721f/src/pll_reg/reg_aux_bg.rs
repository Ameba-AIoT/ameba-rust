#[doc = "Register `REG_AUX_BG` reader"]
pub type R = crate::R<RegAuxBgSpec>;
#[doc = "Register `REG_AUX_BG` writer"]
pub type W = crate::W<RegAuxBgSpec>;
#[doc = "Field `POW_BG` reader - power on BG"]
pub type PowBgR = crate::BitReader;
#[doc = "Field `POW_BG` writer - power on BG"]
pub type PowBgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - power on BG"]
    #[inline(always)]
    pub fn pow_bg(&self) -> PowBgR {
        PowBgR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - power on BG"]
    #[inline(always)]
    pub fn pow_bg(&mut self) -> PowBgW<'_, RegAuxBgSpec> {
        PowBgW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aux_bg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aux_bg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAuxBgSpec;
impl crate::RegisterSpec for RegAuxBgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aux_bg::R`](R) reader structure"]
impl crate::Readable for RegAuxBgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_aux_bg::W`](W) writer structure"]
impl crate::Writable for RegAuxBgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AUX_BG to value 0"]
impl crate::Resettable for RegAuxBgSpec {}
