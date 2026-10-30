#[doc = "Register `REG_EDCA_VO_PARAM` reader"]
pub type R = crate::R<RegEdcaVoParamSpec>;
#[doc = "Register `REG_EDCA_VO_PARAM` writer"]
pub type W = crate::W<RegEdcaVoParamSpec>;
#[doc = "Field `BIT_AIFS_VO` reader - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
pub type BitAifsVoR = crate::FieldReader;
#[doc = "Field `BIT_AIFS_VO` writer - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
pub type BitAifsVoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_CW_VO` reader - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitCwVoR = crate::FieldReader;
#[doc = "Field `BIT_CW_VO` writer - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitCwVoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_TXOPLIMIT_VO` reader - TXOP Limit An unsigned integer in units of 32 microseconds. Value of zero indicates that a single MSDU or MMPDU in addition to a protection frame exchange can be transmitted at any rate. Default: BK = 0h; BE = 0h; VI = 5Eh; VO = 2Fh."]
pub type BitTxoplimitVoR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TXOPLIMIT_VO` writer - TXOP Limit An unsigned integer in units of 32 microseconds. Value of zero indicates that a single MSDU or MMPDU in addition to a protection frame exchange can be transmitted at any rate. Default: BK = 0h; BE = 0h; VI = 5Eh; VO = 2Fh."]
pub type BitTxoplimitVoW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
    #[inline(always)]
    pub fn bit_aifs_vo(&self) -> BitAifsVoR {
        BitAifsVoR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_cw_vo(&self) -> BitCwVoR {
        BitCwVoR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:26 - TXOP Limit An unsigned integer in units of 32 microseconds. Value of zero indicates that a single MSDU or MMPDU in addition to a protection frame exchange can be transmitted at any rate. Default: BK = 0h; BE = 0h; VI = 5Eh; VO = 2Fh."]
    #[inline(always)]
    pub fn bit_txoplimit_vo(&self) -> BitTxoplimitVoR {
        BitTxoplimitVoR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
    #[inline(always)]
    pub fn bit_aifs_vo(&mut self) -> BitAifsVoW<'_, RegEdcaVoParamSpec> {
        BitAifsVoW::new(self, 0)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_cw_vo(&mut self) -> BitCwVoW<'_, RegEdcaVoParamSpec> {
        BitCwVoW::new(self, 8)
    }
    #[doc = "Bits 16:26 - TXOP Limit An unsigned integer in units of 32 microseconds. Value of zero indicates that a single MSDU or MMPDU in addition to a protection frame exchange can be transmitted at any rate. Default: BK = 0h; BE = 0h; VI = 5Eh; VO = 2Fh."]
    #[inline(always)]
    pub fn bit_txoplimit_vo(&mut self) -> BitTxoplimitVoW<'_, RegEdcaVoParamSpec> {
        BitTxoplimitVoW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_edca_vo_param::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_edca_vo_param::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegEdcaVoParamSpec;
impl crate::RegisterSpec for RegEdcaVoParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_edca_vo_param::R`](R) reader structure"]
impl crate::Readable for RegEdcaVoParamSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_edca_vo_param::W`](W) writer structure"]
impl crate::Writable for RegEdcaVoParamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_EDCA_VO_PARAM to value 0x002f_3224"]
impl crate::Resettable for RegEdcaVoParamSpec {
    const RESET_VALUE: u32 = 0x002f_3224;
}
