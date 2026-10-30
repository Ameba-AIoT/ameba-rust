#[doc = "Register `REG_RESP_SIFS_CCK` reader"]
pub type R = crate::R<RegRespSifsCckSpec>;
#[doc = "Register `REG_RESP_SIFS_CCK` writer"]
pub type W = crate::W<RegRespSifsCckSpec>;
#[doc = "Field `BIT_ACKTO_OFDM` reader - This register sets the Ack time out value after Tx unicast packet. It is in units of us."]
pub type BitAcktoOfdmR = crate::FieldReader<u16>;
#[doc = "Field `BIT_ACKTO_OFDM` writer - This register sets the Ack time out value after Tx unicast packet. It is in units of us."]
pub type BitAcktoOfdmW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_ACKTO_CCK` reader - This register sets the Ack time out value after Tx unicast packet with CCK rate. It is in units of us."]
pub type BitAcktoCckR = crate::FieldReader<u16>;
#[doc = "Field `BIT_ACKTO_CCK` writer - This register sets the Ack time out value after Tx unicast packet with CCK rate. It is in units of us."]
pub type BitAcktoCckW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - This register sets the Ack time out value after Tx unicast packet. It is in units of us."]
    #[inline(always)]
    pub fn bit_ackto_ofdm(&self) -> BitAcktoOfdmR {
        BitAcktoOfdmR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - This register sets the Ack time out value after Tx unicast packet with CCK rate. It is in units of us."]
    #[inline(always)]
    pub fn bit_ackto_cck(&self) -> BitAcktoCckR {
        BitAcktoCckR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register sets the Ack time out value after Tx unicast packet. It is in units of us."]
    #[inline(always)]
    pub fn bit_ackto_ofdm(&mut self) -> BitAcktoOfdmW<'_, RegRespSifsCckSpec> {
        BitAcktoOfdmW::new(self, 0)
    }
    #[doc = "Bits 12:23 - This register sets the Ack time out value after Tx unicast packet with CCK rate. It is in units of us."]
    #[inline(always)]
    pub fn bit_ackto_cck(&mut self) -> BitAcktoCckW<'_, RegRespSifsCckSpec> {
        BitAcktoCckW::new(self, 12)
    }
}
#[doc = "Response SIFS for CCK Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_resp_sifs_cck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_resp_sifs_cck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRespSifsCckSpec;
impl crate::RegisterSpec for RegRespSifsCckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_resp_sifs_cck::R`](R) reader structure"]
impl crate::Readable for RegRespSifsCckSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_resp_sifs_cck::W`](W) writer structure"]
impl crate::Writable for RegRespSifsCckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RESP_SIFS_CCK to value 0x0007_e02d"]
impl crate::Resettable for RegRespSifsCckSpec {
    const RESET_VALUE: u32 = 0x0007_e02d;
}
