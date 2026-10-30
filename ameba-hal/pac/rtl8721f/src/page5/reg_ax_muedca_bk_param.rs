#[doc = "Register `REG_AX_MUEDCA_BK_PARAM` reader"]
pub type R = crate::R<RegAxMuedcaBkParamSpec>;
#[doc = "Register `REG_AX_MUEDCA_BK_PARAM` writer"]
pub type W = crate::W<RegAxMuedcaBkParamSpec>;
#[doc = "Field `BIT_muedca_bk_param_aifs` reader - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
pub type BitMuedcaBkParamAifsR = crate::FieldReader;
#[doc = "Field `BIT_muedca_bk_param_aifs` writer - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
pub type BitMuedcaBkParamAifsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_muedca_bk_param_cw` reader - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitMuedcaBkParamCwR = crate::FieldReader;
#[doc = "Field `BIT_muedca_bk_param_cw` writer - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitMuedcaBkParamCwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_muedca_bk_param_timer` reader - MU EDCA TIMER, in units of 32 us Note: SPEC: The MU EDCA Timer field (8b) indicates the duration of time, in units of 8 TUs"]
pub type BitMuedcaBkParamTimerR = crate::FieldReader<u16>;
#[doc = "Field `BIT_muedca_bk_param_timer` writer - MU EDCA TIMER, in units of 32 us Note: SPEC: The MU EDCA Timer field (8b) indicates the duration of time, in units of 8 TUs"]
pub type BitMuedcaBkParamTimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
    #[inline(always)]
    pub fn bit_muedca_bk_param_aifs(&self) -> BitMuedcaBkParamAifsR {
        BitMuedcaBkParamAifsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_muedca_bk_param_cw(&self) -> BitMuedcaBkParamCwR {
        BitMuedcaBkParamCwR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - MU EDCA TIMER, in units of 32 us Note: SPEC: The MU EDCA Timer field (8b) indicates the duration of time, in units of 8 TUs"]
    #[inline(always)]
    pub fn bit_muedca_bk_param_timer(&self) -> BitMuedcaBkParamTimerR {
        BitMuedcaBkParamTimerR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
    #[inline(always)]
    pub fn bit_muedca_bk_param_aifs(
        &mut self,
    ) -> BitMuedcaBkParamAifsW<'_, RegAxMuedcaBkParamSpec> {
        BitMuedcaBkParamAifsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_muedca_bk_param_cw(&mut self) -> BitMuedcaBkParamCwW<'_, RegAxMuedcaBkParamSpec> {
        BitMuedcaBkParamCwW::new(self, 8)
    }
    #[doc = "Bits 16:31 - MU EDCA TIMER, in units of 32 us Note: SPEC: The MU EDCA Timer field (8b) indicates the duration of time, in units of 8 TUs"]
    #[inline(always)]
    pub fn bit_muedca_bk_param_timer(
        &mut self,
    ) -> BitMuedcaBkParamTimerW<'_, RegAxMuedcaBkParamSpec> {
        BitMuedcaBkParamTimerW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_muedca_bk_param::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_muedca_bk_param::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxMuedcaBkParamSpec;
impl crate::RegisterSpec for RegAxMuedcaBkParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_muedca_bk_param::R`](R) reader structure"]
impl crate::Readable for RegAxMuedcaBkParamSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_muedca_bk_param::W`](W) writer structure"]
impl crate::Writable for RegAxMuedcaBkParamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_MUEDCA_BK_PARAM to value 0xa549"]
impl crate::Resettable for RegAxMuedcaBkParamSpec {
    const RESET_VALUE: u32 = 0xa549;
}
