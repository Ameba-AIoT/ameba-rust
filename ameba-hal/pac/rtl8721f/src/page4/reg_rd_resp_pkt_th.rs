#[doc = "Register `REG_RD_RESP_PKT_TH` reader"]
pub type R = crate::R<RegRdRespPktThSpec>;
#[doc = "Register `REG_RD_RESP_PKT_TH` writer"]
pub type W = crate::W<RegRdRespPktThSpec>;
#[doc = "Field `BIT_RD_RESP_PKT_TH` reader - When RD_CTRL responder enables, and the granted AC queue packet number is beyond this threshold, then responder will carry More PPDU =1 and piggyback data frames back."]
pub type BitRdRespPktThR = crate::FieldReader;
#[doc = "Field `BIT_RD_RESP_PKT_TH` writer - When RD_CTRL responder enables, and the granted AC queue packet number is beyond this threshold, then responder will carry More PPDU =1 and piggyback data frames back."]
pub type BitRdRespPktThW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 24:29 - When RD_CTRL responder enables, and the granted AC queue packet number is beyond this threshold, then responder will carry More PPDU =1 and piggyback data frames back."]
    #[inline(always)]
    pub fn bit_rd_resp_pkt_th(&self) -> BitRdRespPktThR {
        BitRdRespPktThR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - When RD_CTRL responder enables, and the granted AC queue packet number is beyond this threshold, then responder will carry More PPDU =1 and piggyback data frames back."]
    #[inline(always)]
    pub fn bit_rd_resp_pkt_th(&mut self) -> BitRdRespPktThW<'_, RegRdRespPktThSpec> {
        BitRdRespPktThW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rd_resp_pkt_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rd_resp_pkt_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRdRespPktThSpec;
impl crate::RegisterSpec for RegRdRespPktThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rd_resp_pkt_th::R`](R) reader structure"]
impl crate::Readable for RegRdRespPktThSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rd_resp_pkt_th::W`](W) writer structure"]
impl crate::Writable for RegRdRespPktThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RD_RESP_PKT_TH to value 0x0300_0000"]
impl crate::Resettable for RegRdRespPktThSpec {
    const RESET_VALUE: u32 = 0x0300_0000;
}
