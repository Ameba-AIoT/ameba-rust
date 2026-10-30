#[doc = "Register `REG_ACKTO` reader"]
pub type R = crate::R<RegAcktoSpec>;
#[doc = "Register `REG_ACKTO` writer"]
pub type W = crate::W<RegAcktoSpec>;
#[doc = "Field `BIT_CTS2TO` reader - This register sets the CTS2 time out value after CTS1 in Dual CTS sequence. It is in units of us."]
pub type BitCts2toR = crate::FieldReader<u16>;
#[doc = "Field `BIT_CTS2TO` writer - This register sets the CTS2 time out value after CTS1 in Dual CTS sequence. It is in units of us."]
pub type BitCts2toW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_resp_dcm_disuse_rx` reader - 0:rx dcm will response tx dcm when ctrl info is valid and inf0_dcm=1"]
pub type RRespDcmDisuseRxR = crate::BitReader;
#[doc = "Field `r_resp_dcm_disuse_rx` writer - 0:rx dcm will response tx dcm when ctrl info is valid and inf0_dcm=1"]
pub type RRespDcmDisuseRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_resp_ersu_disuse_rx` reader - 0:rx er su will response tx er su"]
pub type RRespErsuDisuseRxR = crate::BitReader;
#[doc = "Field `r_resp_ersu_disuse_rx` writer - 0:rx er su will response tx er su"]
pub type RRespErsuDisuseRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EIFS` reader - This register sets the EIFS value mentioned in 802.11 specification. It is in units of us. To receive 1Mbps ACK frame, EIFS = SIFS + 144us (preamble length) + 48us (PLCP header length) + 112us (14 bytes of data) = 314us."]
pub type BitEifsR = crate::FieldReader<u16>;
#[doc = "Field `BIT_EIFS` writer - This register sets the EIFS value mentioned in 802.11 specification. It is in units of us. To receive 1Mbps ACK frame, EIFS = SIFS + 144us (preamble length) + 48us (PLCP header length) + 112us (14 bytes of data) = 314us."]
pub type BitEifsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - This register sets the CTS2 time out value after CTS1 in Dual CTS sequence. It is in units of us."]
    #[inline(always)]
    pub fn bit_cts2to(&self) -> BitCts2toR {
        BitCts2toR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - 0:rx dcm will response tx dcm when ctrl info is valid and inf0_dcm=1"]
    #[inline(always)]
    pub fn r_resp_dcm_disuse_rx(&self) -> RRespDcmDisuseRxR {
        RRespDcmDisuseRxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0:rx er su will response tx er su"]
    #[inline(always)]
    pub fn r_resp_ersu_disuse_rx(&self) -> RRespErsuDisuseRxR {
        RRespErsuDisuseRxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:31 - This register sets the EIFS value mentioned in 802.11 specification. It is in units of us. To receive 1Mbps ACK frame, EIFS = SIFS + 144us (preamble length) + 48us (PLCP header length) + 112us (14 bytes of data) = 314us."]
    #[inline(always)]
    pub fn bit_eifs(&self) -> BitEifsR {
        BitEifsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register sets the CTS2 time out value after CTS1 in Dual CTS sequence. It is in units of us."]
    #[inline(always)]
    pub fn bit_cts2to(&mut self) -> BitCts2toW<'_, RegAcktoSpec> {
        BitCts2toW::new(self, 0)
    }
    #[doc = "Bit 12 - 0:rx dcm will response tx dcm when ctrl info is valid and inf0_dcm=1"]
    #[inline(always)]
    pub fn r_resp_dcm_disuse_rx(&mut self) -> RRespDcmDisuseRxW<'_, RegAcktoSpec> {
        RRespDcmDisuseRxW::new(self, 12)
    }
    #[doc = "Bit 13 - 0:rx er su will response tx er su"]
    #[inline(always)]
    pub fn r_resp_ersu_disuse_rx(&mut self) -> RRespErsuDisuseRxW<'_, RegAcktoSpec> {
        RRespErsuDisuseRxW::new(self, 13)
    }
    #[doc = "Bits 16:31 - This register sets the EIFS value mentioned in 802.11 specification. It is in units of us. To receive 1Mbps ACK frame, EIFS = SIFS + 144us (preamble length) + 48us (PLCP header length) + 112us (14 bytes of data) = 314us."]
    #[inline(always)]
    pub fn bit_eifs(&mut self) -> BitEifsW<'_, RegAcktoSpec> {
        BitEifsW::new(self, 16)
    }
}
#[doc = "ACK Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ackto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ackto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAcktoSpec;
impl crate::RegisterSpec for RegAcktoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ackto::R`](R) reader structure"]
impl crate::Readable for RegAcktoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ackto::W`](W) writer structure"]
impl crate::Writable for RegAcktoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ACKTO to value 0x013a_002d"]
impl crate::Resettable for RegAcktoSpec {
    const RESET_VALUE: u32 = 0x013a_002d;
}
