#[doc = "Register `CH0_NOISE_TH` reader"]
pub type R = crate::R<Ch0NoiseThSpec>;
#[doc = "Register `CH0_NOISE_TH` writer"]
pub type W = crate::W<Ch0NoiseThSpec>;
#[doc = "Field `ch0_p_ent` reader - The environmental positive noise threshold: the positive maximum capacitance change of raw data that is still considered an environment change."]
pub type Ch0PEntR = crate::FieldReader<u16>;
#[doc = "Field `ch0_p_ent` writer - The environmental positive noise threshold: the positive maximum capacitance change of raw data that is still considered an environment change."]
pub type Ch0PEntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch0_n_ent` reader - The environmental negative noise threshold: the negative maximum capacitance change of raw data that is still considered an environment change."]
pub type Ch0NEntR = crate::FieldReader<u16>;
#[doc = "Field `ch0_n_ent` writer - The environmental negative noise threshold: the negative maximum capacitance change of raw data that is still considered an environment change."]
pub type Ch0NEntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The environmental positive noise threshold: the positive maximum capacitance change of raw data that is still considered an environment change."]
    #[inline(always)]
    pub fn ch0_p_ent(&self) -> Ch0PEntR {
        Ch0PEntR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - The environmental negative noise threshold: the negative maximum capacitance change of raw data that is still considered an environment change."]
    #[inline(always)]
    pub fn ch0_n_ent(&self) -> Ch0NEntR {
        Ch0NEntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The environmental positive noise threshold: the positive maximum capacitance change of raw data that is still considered an environment change."]
    #[inline(always)]
    pub fn ch0_p_ent(&mut self) -> Ch0PEntW<'_, Ch0NoiseThSpec> {
        Ch0PEntW::new(self, 0)
    }
    #[doc = "Bits 16:27 - The environmental negative noise threshold: the negative maximum capacitance change of raw data that is still considered an environment change."]
    #[inline(always)]
    pub fn ch0_n_ent(&mut self) -> Ch0NEntW<'_, Ch0NoiseThSpec> {
        Ch0NEntW::new(self, 16)
    }
}
#[doc = "Channel 0 Noise Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_noise_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_noise_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0NoiseThSpec;
impl crate::RegisterSpec for Ch0NoiseThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_noise_th::R`](R) reader structure"]
impl crate::Readable for Ch0NoiseThSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0_noise_th::W`](W) writer structure"]
impl crate::Writable for Ch0NoiseThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0_NOISE_TH to value 0x0028_0028"]
impl crate::Resettable for Ch0NoiseThSpec {
    const RESET_VALUE: u32 = 0x0028_0028;
}
