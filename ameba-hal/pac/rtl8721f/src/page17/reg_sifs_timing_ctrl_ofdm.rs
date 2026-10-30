#[doc = "Register `REG_SIFS_TIMING_CTRL_OFDM` reader"]
pub type R = crate::R<RegSifsTimingCtrlOfdmSpec>;
#[doc = "Register `REG_SIFS_TIMING_CTRL_OFDM` writer"]
pub type W = crate::W<RegSifsTimingCtrlOfdmSpec>;
#[doc = "Field `BIT_T2T_SIFS_OFDM` reader - This register sets the SIFS time required for consecutive TX events for example CTS-Data, or broadcast bursting. It is in unit of 100ns.(default:13us)"]
pub type BitT2tSifsOfdmR = crate::FieldReader<u16>;
#[doc = "Field `BIT_T2T_SIFS_OFDM` writer - This register sets the SIFS time required for consecutive TX events for example CTS-Data, or broadcast bursting. It is in unit of 100ns.(default:13us)"]
pub type BitT2tSifsOfdmW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_R2T_SIFS_OFDM` reader - This register sets the SIFS time required for any immediate response for example Data-Ack. It is in unit of 100ns.(default:13us)"]
pub type BitR2tSifsOfdmR = crate::FieldReader<u16>;
#[doc = "Field `BIT_R2T_SIFS_OFDM` writer - This register sets the SIFS time required for any immediate response for example Data-Ack. It is in unit of 100ns.(default:13us)"]
pub type BitR2tSifsOfdmW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_RX_AIR_END_TIME` reader - This register sets the time from cca fall to 6us. It is in unit of 100ns.(default:6.1us)"]
pub type BitRxAirEndTimeR = crate::FieldReader;
#[doc = "Field `BIT_RX_AIR_END_TIME` writer - This register sets the time from cca fall to 6us. It is in unit of 100ns.(default:6.1us)"]
pub type BitRxAirEndTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - This register sets the SIFS time required for consecutive TX events for example CTS-Data, or broadcast bursting. It is in unit of 100ns.(default:13us)"]
    #[inline(always)]
    pub fn bit_t2t_sifs_ofdm(&self) -> BitT2tSifsOfdmR {
        BitT2tSifsOfdmR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - This register sets the SIFS time required for any immediate response for example Data-Ack. It is in unit of 100ns.(default:13us)"]
    #[inline(always)]
    pub fn bit_r2t_sifs_ofdm(&self) -> BitR2tSifsOfdmR {
        BitR2tSifsOfdmR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - This register sets the time from cca fall to 6us. It is in unit of 100ns.(default:6.1us)"]
    #[inline(always)]
    pub fn bit_rx_air_end_time(&self) -> BitRxAirEndTimeR {
        BitRxAirEndTimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register sets the SIFS time required for consecutive TX events for example CTS-Data, or broadcast bursting. It is in unit of 100ns.(default:13us)"]
    #[inline(always)]
    pub fn bit_t2t_sifs_ofdm(&mut self) -> BitT2tSifsOfdmW<'_, RegSifsTimingCtrlOfdmSpec> {
        BitT2tSifsOfdmW::new(self, 0)
    }
    #[doc = "Bits 12:23 - This register sets the SIFS time required for any immediate response for example Data-Ack. It is in unit of 100ns.(default:13us)"]
    #[inline(always)]
    pub fn bit_r2t_sifs_ofdm(&mut self) -> BitR2tSifsOfdmW<'_, RegSifsTimingCtrlOfdmSpec> {
        BitR2tSifsOfdmW::new(self, 12)
    }
    #[doc = "Bits 24:31 - This register sets the time from cca fall to 6us. It is in unit of 100ns.(default:6.1us)"]
    #[inline(always)]
    pub fn bit_rx_air_end_time(&mut self) -> BitRxAirEndTimeW<'_, RegSifsTimingCtrlOfdmSpec> {
        BitRxAirEndTimeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sifs_timing_ctrl_ofdm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sifs_timing_ctrl_ofdm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSifsTimingCtrlOfdmSpec;
impl crate::RegisterSpec for RegSifsTimingCtrlOfdmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sifs_timing_ctrl_ofdm::R`](R) reader structure"]
impl crate::Readable for RegSifsTimingCtrlOfdmSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sifs_timing_ctrl_ofdm::W`](W) writer structure"]
impl crate::Writable for RegSifsTimingCtrlOfdmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SIFS_TIMING_CTRL_OFDM to value 0x3d08_2082"]
impl crate::Resettable for RegSifsTimingCtrlOfdmSpec {
    const RESET_VALUE: u32 = 0x3d08_2082;
}
