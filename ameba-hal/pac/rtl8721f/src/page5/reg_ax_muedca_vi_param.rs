#[doc = "Register `REG_AX_MUEDCA_VI_PARAM` reader"]
pub type R = crate::R<RegAxMuedcaViParamSpec>;
#[doc = "Register `REG_AX_MUEDCA_VI_PARAM` writer"]
pub type W = crate::W<RegAxMuedcaViParamSpec>;
#[doc = "Field `BIT_muedca_vi_param_aifs` reader - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
pub type BitMuedcaViParamAifsR = crate::FieldReader;
#[doc = "Field `BIT_muedca_vi_param_aifs` writer - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
pub type BitMuedcaViParamAifsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_muedca_vi_param_cw` reader - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitMuedcaViParamCwR = crate::FieldReader;
#[doc = "Field `BIT_muedca_vi_param_cw` writer - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitMuedcaViParamCwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_muedca_vi_param_timer` reader - MU EDCA TIMER, in units of 32 us Note: SPEC: The MU EDCA Timer field (8b) indicates the duration of time, in units of 8 TUs"]
pub type BitMuedcaViParamTimerR = crate::FieldReader<u16>;
#[doc = "Field `BIT_muedca_vi_param_timer` writer - MU EDCA TIMER, in units of 32 us Note: SPEC: The MU EDCA Timer field (8b) indicates the duration of time, in units of 8 TUs"]
pub type BitMuedcaViParamTimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
    #[inline(always)]
    pub fn bit_muedca_vi_param_aifs(&self) -> BitMuedcaViParamAifsR {
        BitMuedcaViParamAifsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_muedca_vi_param_cw(&self) -> BitMuedcaViParamCwR {
        BitMuedcaViParamCwR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - MU EDCA TIMER, in units of 32 us Note: SPEC: The MU EDCA Timer field (8b) indicates the duration of time, in units of 8 TUs"]
    #[inline(always)]
    pub fn bit_muedca_vi_param_timer(&self) -> BitMuedcaViParamTimerR {
        BitMuedcaViParamTimerR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
    #[inline(always)]
    pub fn bit_muedca_vi_param_aifs(
        &mut self,
    ) -> BitMuedcaViParamAifsW<'_, RegAxMuedcaViParamSpec> {
        BitMuedcaViParamAifsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_muedca_vi_param_cw(&mut self) -> BitMuedcaViParamCwW<'_, RegAxMuedcaViParamSpec> {
        BitMuedcaViParamCwW::new(self, 8)
    }
    #[doc = "Bits 16:31 - MU EDCA TIMER, in units of 32 us Note: SPEC: The MU EDCA Timer field (8b) indicates the duration of time, in units of 8 TUs"]
    #[inline(always)]
    pub fn bit_muedca_vi_param_timer(
        &mut self,
    ) -> BitMuedcaViParamTimerW<'_, RegAxMuedcaViParamSpec> {
        BitMuedcaViParamTimerW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_muedca_vi_param::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_muedca_vi_param::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxMuedcaViParamSpec;
impl crate::RegisterSpec for RegAxMuedcaViParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_muedca_vi_param::R`](R) reader structure"]
impl crate::Readable for RegAxMuedcaViParamSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_muedca_vi_param::W`](W) writer structure"]
impl crate::Writable for RegAxMuedcaViParamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_MUEDCA_VI_PARAM to value 0x541c"]
impl crate::Resettable for RegAxMuedcaViParamSpec {
    const RESET_VALUE: u32 = 0x541c;
}
