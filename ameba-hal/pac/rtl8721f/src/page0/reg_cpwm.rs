#[doc = "Register `REG_CPWM` reader"]
pub type R = crate::R<RegCpwmSpec>;
#[doc = "Register `REG_CPWM` writer"]
pub type W = crate::W<RegCpwmSpec>;
#[doc = "Field `BIT_CPWM_MOD` reader - The current power mode index."]
pub type BitCpwmModR = crate::FieldReader<u16>;
#[doc = "Field `BIT_CPWM_MOD` writer - The current power mode index."]
pub type BitCpwmModW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `BIT_CPWM_TOGGLING` reader - Toggling bit : one bit sequence number bit. When this bit is toggled, interrupt is send to HISR.CPWMINT."]
pub type BitCpwmTogglingR = crate::BitReader;
#[doc = "Field `BIT_CPWM_TOGGLING` writer - Toggling bit : one bit sequence number bit. When this bit is toggled, interrupt is send to HISR.CPWMINT."]
pub type BitCpwmTogglingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - The current power mode index."]
    #[inline(always)]
    pub fn bit_cpwm_mod(&self) -> BitCpwmModR {
        BitCpwmModR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Toggling bit : one bit sequence number bit. When this bit is toggled, interrupt is send to HISR.CPWMINT."]
    #[inline(always)]
    pub fn bit_cpwm_toggling(&self) -> BitCpwmTogglingR {
        BitCpwmTogglingR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - The current power mode index."]
    #[inline(always)]
    pub fn bit_cpwm_mod(&mut self) -> BitCpwmModW<'_, RegCpwmSpec> {
        BitCpwmModW::new(self, 0)
    }
    #[doc = "Bit 15 - Toggling bit : one bit sequence number bit. When this bit is toggled, interrupt is send to HISR.CPWMINT."]
    #[inline(always)]
    pub fn bit_cpwm_toggling(&mut self) -> BitCpwmTogglingW<'_, RegCpwmSpec> {
        BitCpwmTogglingW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cpwm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cpwm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCpwmSpec;
impl crate::RegisterSpec for RegCpwmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cpwm::R`](R) reader structure"]
impl crate::Readable for RegCpwmSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cpwm::W`](W) writer structure"]
impl crate::Writable for RegCpwmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CPWM to value 0"]
impl crate::Resettable for RegCpwmSpec {}
