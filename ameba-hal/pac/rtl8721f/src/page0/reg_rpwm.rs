#[doc = "Register `REG_RPWM` reader"]
pub type R = crate::R<RegRpwmSpec>;
#[doc = "Register `REG_RPWM` writer"]
pub type W = crate::W<RegRpwmSpec>;
#[doc = "Field `BIT_RPWM_MOD` reader - The current power mode index."]
pub type BitRpwmModR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RPWM_MOD` writer - The current power mode index."]
pub type BitRpwmModW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `BIT_RPWM_TOGGLING` reader - Toggling bit : one bit sequence number bit. When this bit is toggled, interrupt is send to FISR.RPWMINT."]
pub type BitRpwmTogglingR = crate::BitReader;
#[doc = "Field `BIT_RPWM_TOGGLING` writer - Toggling bit : one bit sequence number bit. When this bit is toggled, interrupt is send to FISR.RPWMINT."]
pub type BitRpwmTogglingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - The current power mode index."]
    #[inline(always)]
    pub fn bit_rpwm_mod(&self) -> BitRpwmModR {
        BitRpwmModR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Toggling bit : one bit sequence number bit. When this bit is toggled, interrupt is send to FISR.RPWMINT."]
    #[inline(always)]
    pub fn bit_rpwm_toggling(&self) -> BitRpwmTogglingR {
        BitRpwmTogglingR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - The current power mode index."]
    #[inline(always)]
    pub fn bit_rpwm_mod(&mut self) -> BitRpwmModW<'_, RegRpwmSpec> {
        BitRpwmModW::new(self, 0)
    }
    #[doc = "Bit 15 - Toggling bit : one bit sequence number bit. When this bit is toggled, interrupt is send to FISR.RPWMINT."]
    #[inline(always)]
    pub fn bit_rpwm_toggling(&mut self) -> BitRpwmTogglingW<'_, RegRpwmSpec> {
        BitRpwmTogglingW::new(self, 15)
    }
}
#[doc = "register for hrpwm\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rpwm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rpwm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRpwmSpec;
impl crate::RegisterSpec for RegRpwmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rpwm::R`](R) reader structure"]
impl crate::Readable for RegRpwmSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rpwm::W`](W) writer structure"]
impl crate::Writable for RegRpwmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RPWM to value 0"]
impl crate::Resettable for RegRpwmSpec {}
