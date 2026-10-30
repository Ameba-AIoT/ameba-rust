#[doc = "Register `REG_TSF_TIMER2` reader"]
pub type R = crate::R<RegTsfTimer2Spec>;
#[doc = "Register `REG_TSF_TIMER2` writer"]
pub type W = crate::W<RegTsfTimer2Spec>;
#[doc = "Field `BIT_TSTTIMER2` reader - When the lower 4 bytes of TSFTR matches the content of this register, the FW tsftime_int\\[1\\] interrupt is issued."]
pub type BitTsttimer2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_TSTTIMER2` writer - When the lower 4 bytes of TSFTR matches the content of this register, the FW tsftime_int\\[1\\] interrupt is issued."]
pub type BitTsttimer2W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - When the lower 4 bytes of TSFTR matches the content of this register, the FW tsftime_int\\[1\\] interrupt is issued."]
    #[inline(always)]
    pub fn bit_tsttimer2(&self) -> BitTsttimer2R {
        BitTsttimer2R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - When the lower 4 bytes of TSFTR matches the content of this register, the FW tsftime_int\\[1\\] interrupt is issued."]
    #[inline(always)]
    pub fn bit_tsttimer2(&mut self) -> BitTsttimer2W<'_, RegTsfTimer2Spec> {
        BitTsttimer2W::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tsf_timer2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tsf_timer2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTsfTimer2Spec;
impl crate::RegisterSpec for RegTsfTimer2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tsf_timer2::R`](R) reader structure"]
impl crate::Readable for RegTsfTimer2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tsf_timer2::W`](W) writer structure"]
impl crate::Writable for RegTsfTimer2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TSF_TIMER2 to value 0"]
impl crate::Resettable for RegTsfTimer2Spec {}
