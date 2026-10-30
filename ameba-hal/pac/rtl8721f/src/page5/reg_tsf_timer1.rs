#[doc = "Register `REG_TSF_TIMER1` reader"]
pub type R = crate::R<RegTsfTimer1Spec>;
#[doc = "Register `REG_TSF_TIMER1` writer"]
pub type W = crate::W<RegTsfTimer1Spec>;
#[doc = "Field `BIT_TSTTIMER1` reader - When the lower 4 bytes of TSFTR matches the content of this register, the FW tsftime_int\\[0\\] interrupt is issued."]
pub type BitTsttimer1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_TSTTIMER1` writer - When the lower 4 bytes of TSFTR matches the content of this register, the FW tsftime_int\\[0\\] interrupt is issued."]
pub type BitTsttimer1W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - When the lower 4 bytes of TSFTR matches the content of this register, the FW tsftime_int\\[0\\] interrupt is issued."]
    #[inline(always)]
    pub fn bit_tsttimer1(&self) -> BitTsttimer1R {
        BitTsttimer1R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - When the lower 4 bytes of TSFTR matches the content of this register, the FW tsftime_int\\[0\\] interrupt is issued."]
    #[inline(always)]
    pub fn bit_tsttimer1(&mut self) -> BitTsttimer1W<'_, RegTsfTimer1Spec> {
        BitTsttimer1W::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tsf_timer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tsf_timer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTsfTimer1Spec;
impl crate::RegisterSpec for RegTsfTimer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tsf_timer1::R`](R) reader structure"]
impl crate::Readable for RegTsfTimer1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tsf_timer1::W`](W) writer structure"]
impl crate::Writable for RegTsfTimer1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TSF_TIMER1 to value 0"]
impl crate::Resettable for RegTsfTimer1Spec {}
