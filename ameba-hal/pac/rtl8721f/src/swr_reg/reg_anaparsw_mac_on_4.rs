#[doc = "Register `REG_ANAPARSW_MAC_ON_4` reader"]
pub type R = crate::R<RegAnaparswMacOn4Spec>;
#[doc = "Register `REG_ANAPARSW_MAC_ON_4` writer"]
pub type W = crate::W<RegAnaparswMacOn4Spec>;
#[doc = "Field `null1` reader - NO use in RL6973"]
pub type Null1R = crate::FieldReader<u32>;
#[doc = "Field `null1` writer - NO use in RL6973"]
pub type Null1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NO use in RL6973"]
    #[inline(always)]
    pub fn null1(&self) -> Null1R {
        Null1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - NO use in RL6973"]
    #[inline(always)]
    pub fn null1(&mut self) -> Null1W<'_, RegAnaparswMacOn4Spec> {
        Null1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anaparsw_mac_on_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anaparsw_mac_on_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparswMacOn4Spec;
impl crate::RegisterSpec for RegAnaparswMacOn4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anaparsw_mac_on_4::R`](R) reader structure"]
impl crate::Readable for RegAnaparswMacOn4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anaparsw_mac_on_4::W`](W) writer structure"]
impl crate::Writable for RegAnaparswMacOn4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPARSW_MAC_ON_4 to value 0x0410"]
impl crate::Resettable for RegAnaparswMacOn4Spec {
    const RESET_VALUE: u32 = 0x0410;
}
