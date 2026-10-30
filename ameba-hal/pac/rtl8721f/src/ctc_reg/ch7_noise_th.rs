#[doc = "Register `CH7_NOISE_TH` reader"]
pub type R = crate::R<Ch7NoiseThSpec>;
#[doc = "Register `CH7_NOISE_TH` writer"]
pub type W = crate::W<Ch7NoiseThSpec>;
#[doc = "Field `ch7_p_ent` reader - The environmental positive noise threshold: the positive maximum capacitance change of raw data that is still considered an environment change."]
pub type Ch7PEntR = crate::FieldReader<u16>;
#[doc = "Field `ch7_p_ent` writer - The environmental positive noise threshold: the positive maximum capacitance change of raw data that is still considered an environment change."]
pub type Ch7PEntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch7_n_ent` reader - The environmental negative noise threshold: the negative maximum capacitance change of raw data that is still considered an environment change."]
pub type Ch7NEntR = crate::FieldReader<u16>;
#[doc = "Field `ch7_n_ent` writer - The environmental negative noise threshold: the negative maximum capacitance change of raw data that is still considered an environment change."]
pub type Ch7NEntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The environmental positive noise threshold: the positive maximum capacitance change of raw data that is still considered an environment change."]
    #[inline(always)]
    pub fn ch7_p_ent(&self) -> Ch7PEntR {
        Ch7PEntR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - The environmental negative noise threshold: the negative maximum capacitance change of raw data that is still considered an environment change."]
    #[inline(always)]
    pub fn ch7_n_ent(&self) -> Ch7NEntR {
        Ch7NEntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The environmental positive noise threshold: the positive maximum capacitance change of raw data that is still considered an environment change."]
    #[inline(always)]
    pub fn ch7_p_ent(&mut self) -> Ch7PEntW<'_, Ch7NoiseThSpec> {
        Ch7PEntW::new(self, 0)
    }
    #[doc = "Bits 16:27 - The environmental negative noise threshold: the negative maximum capacitance change of raw data that is still considered an environment change."]
    #[inline(always)]
    pub fn ch7_n_ent(&mut self) -> Ch7NEntW<'_, Ch7NoiseThSpec> {
        Ch7NEntW::new(self, 16)
    }
}
#[doc = "Channel 7 Noise Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_noise_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_noise_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7NoiseThSpec;
impl crate::RegisterSpec for Ch7NoiseThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_noise_th::R`](R) reader structure"]
impl crate::Readable for Ch7NoiseThSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_noise_th::W`](W) writer structure"]
impl crate::Writable for Ch7NoiseThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_NOISE_TH to value 0x0028_0028"]
impl crate::Resettable for Ch7NoiseThSpec {
    const RESET_VALUE: u32 = 0x0028_0028;
}
