#[doc = "Register `REG_EDCA_BE_PARAM` reader"]
pub type R = crate::R<RegEdcaBeParamSpec>;
#[doc = "Register `REG_EDCA_BE_PARAM` writer"]
pub type W = crate::W<RegEdcaBeParamSpec>;
#[doc = "Field `BIT_AIFS_BE` reader - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
pub type BitAifsBeR = crate::FieldReader;
#[doc = "Field `BIT_AIFS_BE` writer - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
pub type BitAifsBeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_CW_BE` reader - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitCwBeR = crate::FieldReader;
#[doc = "Field `BIT_CW_BE` writer - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitCwBeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_TXOPLIMIT_BE` reader - TXOP Limit An unsigned integer in units of 32 microseconds. Value of zero indicates that a single MSDU or MMPDU in addition to a protection frame exchange can be transmitted at any rate. Default: BK = 0h; BE = 0h; VI = 5Eh; VO = 2Fh."]
pub type BitTxoplimitBeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TXOPLIMIT_BE` writer - TXOP Limit An unsigned integer in units of 32 microseconds. Value of zero indicates that a single MSDU or MMPDU in addition to a protection frame exchange can be transmitted at any rate. Default: BK = 0h; BE = 0h; VI = 5Eh; VO = 2Fh."]
pub type BitTxoplimitBeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
    #[inline(always)]
    pub fn bit_aifs_be(&self) -> BitAifsBeR {
        BitAifsBeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_cw_be(&self) -> BitCwBeR {
        BitCwBeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:26 - TXOP Limit An unsigned integer in units of 32 microseconds. Value of zero indicates that a single MSDU or MMPDU in addition to a protection frame exchange can be transmitted at any rate. Default: BK = 0h; BE = 0h; VI = 5Eh; VO = 2Fh."]
    #[inline(always)]
    pub fn bit_txoplimit_be(&self) -> BitTxoplimitBeR {
        BitTxoplimitBeR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch."]
    #[inline(always)]
    pub fn bit_aifs_be(&mut self) -> BitAifsBeW<'_, RegEdcaBeParamSpec> {
        BitAifsBeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_cw_be(&mut self) -> BitCwBeW<'_, RegEdcaBeParamSpec> {
        BitCwBeW::new(self, 8)
    }
    #[doc = "Bits 16:26 - TXOP Limit An unsigned integer in units of 32 microseconds. Value of zero indicates that a single MSDU or MMPDU in addition to a protection frame exchange can be transmitted at any rate. Default: BK = 0h; BE = 0h; VI = 5Eh; VO = 2Fh."]
    #[inline(always)]
    pub fn bit_txoplimit_be(&mut self) -> BitTxoplimitBeW<'_, RegEdcaBeParamSpec> {
        BitTxoplimitBeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_edca_be_param::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_edca_be_param::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegEdcaBeParamSpec;
impl crate::RegisterSpec for RegEdcaBeParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_edca_be_param::R`](R) reader structure"]
impl crate::Readable for RegEdcaBeParamSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_edca_be_param::W`](W) writer structure"]
impl crate::Writable for RegEdcaBeParamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_EDCA_BE_PARAM to value 0xa42b"]
impl crate::Resettable for RegEdcaBeParamSpec {
    const RESET_VALUE: u32 = 0xa42b;
}
