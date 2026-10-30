#[doc = "Register `REG_RESP_RATE_RRSR_1` reader"]
pub type R = crate::R<RegRespRateRrsr1Spec>;
#[doc = "Register `REG_RESP_RATE_RRSR_1` writer"]
pub type W = crate::W<RegRespRateRrsr1Spec>;
#[doc = "Field `BIT_RESP_RATE_EN` reader - 0\\]:enable respnse rate of CCK and OFDM \\[1\\]:enable respnse rate of HT \\[2\\]:enable respnse rate of VHT \\[3\\]:enable respnse rate of HE"]
pub type BitRespRateEnR = crate::FieldReader;
#[doc = "Field `BIT_RESP_RATE_EN` writer - 0\\]:enable respnse rate of CCK and OFDM \\[1\\]:enable respnse rate of HT \\[2\\]:enable respnse rate of VHT \\[3\\]:enable respnse rate of HE"]
pub type BitRespRateEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_RESP_RRSR_CCK` reader - {1M, 2M, 5.5M, 11M }"]
pub type BitRespRrsrCckR = crate::FieldReader;
#[doc = "Field `BIT_RESP_RRSR_CCK` writer - {1M, 2M, 5.5M, 11M }"]
pub type BitRespRrsrCckW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0\\]:enable respnse rate of CCK and OFDM \\[1\\]:enable respnse rate of HT \\[2\\]:enable respnse rate of VHT \\[3\\]:enable respnse rate of HE"]
    #[inline(always)]
    pub fn bit_resp_rate_en(&self) -> BitRespRateEnR {
        BitRespRateEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - {1M, 2M, 5.5M, 11M }"]
    #[inline(always)]
    pub fn bit_resp_rrsr_cck(&self) -> BitRespRrsrCckR {
        BitRespRrsrCckR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0\\]:enable respnse rate of CCK and OFDM \\[1\\]:enable respnse rate of HT \\[2\\]:enable respnse rate of VHT \\[3\\]:enable respnse rate of HE"]
    #[inline(always)]
    pub fn bit_resp_rate_en(&mut self) -> BitRespRateEnW<'_, RegRespRateRrsr1Spec> {
        BitRespRateEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - {1M, 2M, 5.5M, 11M }"]
    #[inline(always)]
    pub fn bit_resp_rrsr_cck(&mut self) -> BitRespRrsrCckW<'_, RegRespRateRrsr1Spec> {
        BitRespRrsrCckW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_resp_rate_rrsr_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_resp_rate_rrsr_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRespRateRrsr1Spec;
impl crate::RegisterSpec for RegRespRateRrsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_resp_rate_rrsr_1::R`](R) reader structure"]
impl crate::Readable for RegRespRateRrsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_resp_rate_rrsr_1::W`](W) writer structure"]
impl crate::Writable for RegRespRateRrsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RESP_RATE_RRSR_1 to value 0x3f"]
impl crate::Resettable for RegRespRateRrsr1Spec {
    const RESET_VALUE: u32 = 0x3f;
}
