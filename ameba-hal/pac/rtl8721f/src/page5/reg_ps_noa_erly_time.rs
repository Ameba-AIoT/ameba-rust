#[doc = "Register `REG_PS_NOA_ERLY_TIME` reader"]
pub type R = crate::R<RegPsNoaErlyTimeSpec>;
#[doc = "Register `REG_PS_NOA_ERLY_TIME` writer"]
pub type W = crate::W<RegPsNoaErlyTimeSpec>;
#[doc = "Field `BIT_NOA_ON_ERLY_TIME` reader - not valid. NOA on early interval, Time unit is TU. PS: This function has some issue, so it's value must be 0"]
pub type BitNoaOnErlyTimeR = crate::FieldReader;
#[doc = "Field `BIT_NOA_ON_ERLY_TIME` writer - not valid. NOA on early interval, Time unit is TU. PS: This function has some issue, so it's value must be 0"]
pub type BitNoaOnErlyTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_NOA_OFF_ERLY_TIME` reader - NOA off early interval, Time unit is TU."]
pub type BitNoaOffErlyTimeR = crate::FieldReader;
#[doc = "Field `BIT_NOA_OFF_ERLY_TIME` writer - NOA off early interval, Time unit is TU."]
pub type BitNoaOffErlyTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - not valid. NOA on early interval, Time unit is TU. PS: This function has some issue, so it's value must be 0"]
    #[inline(always)]
    pub fn bit_noa_on_erly_time(&self) -> BitNoaOnErlyTimeR {
        BitNoaOnErlyTimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NOA off early interval, Time unit is TU."]
    #[inline(always)]
    pub fn bit_noa_off_erly_time(&self) -> BitNoaOffErlyTimeR {
        BitNoaOffErlyTimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - not valid. NOA on early interval, Time unit is TU. PS: This function has some issue, so it's value must be 0"]
    #[inline(always)]
    pub fn bit_noa_on_erly_time(&mut self) -> BitNoaOnErlyTimeW<'_, RegPsNoaErlyTimeSpec> {
        BitNoaOnErlyTimeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NOA off early interval, Time unit is TU."]
    #[inline(always)]
    pub fn bit_noa_off_erly_time(&mut self) -> BitNoaOffErlyTimeW<'_, RegPsNoaErlyTimeSpec> {
        BitNoaOffErlyTimeW::new(self, 8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegPsNoaErlyTimeSpec> {
        DummyW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ps_noa_erly_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ps_noa_erly_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsNoaErlyTimeSpec;
impl crate::RegisterSpec for RegPsNoaErlyTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ps_noa_erly_time::R`](R) reader structure"]
impl crate::Readable for RegPsNoaErlyTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ps_noa_erly_time::W`](W) writer structure"]
impl crate::Writable for RegPsNoaErlyTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PS_NOA_ERLY_TIME to value 0x0500"]
impl crate::Resettable for RegPsNoaErlyTimeSpec {
    const RESET_VALUE: u32 = 0x0500;
}
