#[doc = "Register `REG_ANAPARSW_MAC_ON_5` reader"]
pub type R = crate::R<RegAnaparswMacOn5Spec>;
#[doc = "Register `REG_ANAPARSW_MAC_ON_5` writer"]
pub type W = crate::W<RegAnaparswMacOn5Spec>;
#[doc = "Field `null2` reader - NO use in RL6973"]
pub type Null2R = crate::FieldReader;
#[doc = "Field `null2` writer - NO use in RL6973"]
pub type Null2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NO use in RL6973"]
    #[inline(always)]
    pub fn null2(&self) -> Null2R {
        Null2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NO use in RL6973"]
    #[inline(always)]
    pub fn null2(&mut self) -> Null2W<'_, RegAnaparswMacOn5Spec> {
        Null2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anaparsw_mac_on_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anaparsw_mac_on_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparswMacOn5Spec;
impl crate::RegisterSpec for RegAnaparswMacOn5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anaparsw_mac_on_5::R`](R) reader structure"]
impl crate::Readable for RegAnaparswMacOn5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anaparsw_mac_on_5::W`](W) writer structure"]
impl crate::Writable for RegAnaparswMacOn5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPARSW_MAC_ON_5 to value 0xff"]
impl crate::Resettable for RegAnaparswMacOn5Spec {
    const RESET_VALUE: u32 = 0xff;
}
