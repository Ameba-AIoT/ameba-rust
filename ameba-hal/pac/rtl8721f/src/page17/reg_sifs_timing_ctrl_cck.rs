#[doc = "Register `REG_SIFS_TIMING_CTRL_CCK` reader"]
pub type R = crate::R<RegSifsTimingCtrlCckSpec>;
#[doc = "Register `REG_SIFS_TIMING_CTRL_CCK` writer"]
pub type W = crate::W<RegSifsTimingCtrlCckSpec>;
#[doc = "Field `BIT_T2T_SIFS_CCK` reader - This register sets the SIFS time required for consecutive TX events for example CTS-Data, or broadcast bursting. It is in unit of 100ns.(default:7us)"]
pub type BitT2tSifsCckR = crate::FieldReader<u16>;
#[doc = "Field `BIT_T2T_SIFS_CCK` writer - This register sets the SIFS time required for consecutive TX events for example CTS-Data, or broadcast bursting. It is in unit of 100ns.(default:7us)"]
pub type BitT2tSifsCckW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_R2T_SIFS_CCK` reader - This register sets the SIFS time required for any immediate response for example Data-Ack. It is in unit of 100ns.(default:7us)"]
pub type BitR2tSifsCckR = crate::FieldReader<u16>;
#[doc = "Field `BIT_R2T_SIFS_CCK` writer - This register sets the SIFS time required for any immediate response for example Data-Ack. It is in unit of 100ns.(default:7us)"]
pub type BitR2tSifsCckW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_WMAC_100NS_TIME` reader - Sets the 100ns time unit used by MAC clock.(DEFAULT:40M CLK,eg:20M clk should set this value to 2)"]
pub type BitWmac100nsTimeR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_100NS_TIME` writer - Sets the 100ns time unit used by MAC clock.(DEFAULT:40M CLK,eg:20M clk should set this value to 2)"]
pub type BitWmac100nsTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - This register sets the SIFS time required for consecutive TX events for example CTS-Data, or broadcast bursting. It is in unit of 100ns.(default:7us)"]
    #[inline(always)]
    pub fn bit_t2t_sifs_cck(&self) -> BitT2tSifsCckR {
        BitT2tSifsCckR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - This register sets the SIFS time required for any immediate response for example Data-Ack. It is in unit of 100ns.(default:7us)"]
    #[inline(always)]
    pub fn bit_r2t_sifs_cck(&self) -> BitR2tSifsCckR {
        BitR2tSifsCckR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - Sets the 100ns time unit used by MAC clock.(DEFAULT:40M CLK,eg:20M clk should set this value to 2)"]
    #[inline(always)]
    pub fn bit_wmac_100ns_time(&self) -> BitWmac100nsTimeR {
        BitWmac100nsTimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register sets the SIFS time required for consecutive TX events for example CTS-Data, or broadcast bursting. It is in unit of 100ns.(default:7us)"]
    #[inline(always)]
    pub fn bit_t2t_sifs_cck(&mut self) -> BitT2tSifsCckW<'_, RegSifsTimingCtrlCckSpec> {
        BitT2tSifsCckW::new(self, 0)
    }
    #[doc = "Bits 12:23 - This register sets the SIFS time required for any immediate response for example Data-Ack. It is in unit of 100ns.(default:7us)"]
    #[inline(always)]
    pub fn bit_r2t_sifs_cck(&mut self) -> BitR2tSifsCckW<'_, RegSifsTimingCtrlCckSpec> {
        BitR2tSifsCckW::new(self, 12)
    }
    #[doc = "Bits 24:31 - Sets the 100ns time unit used by MAC clock.(DEFAULT:40M CLK,eg:20M clk should set this value to 2)"]
    #[inline(always)]
    pub fn bit_wmac_100ns_time(&mut self) -> BitWmac100nsTimeW<'_, RegSifsTimingCtrlCckSpec> {
        BitWmac100nsTimeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sifs_timing_ctrl_cck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sifs_timing_ctrl_cck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSifsTimingCtrlCckSpec;
impl crate::RegisterSpec for RegSifsTimingCtrlCckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sifs_timing_ctrl_cck::R`](R) reader structure"]
impl crate::Readable for RegSifsTimingCtrlCckSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sifs_timing_ctrl_cck::W`](W) writer structure"]
impl crate::Writable for RegSifsTimingCtrlCckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SIFS_TIMING_CTRL_CCK to value 0x0404_6046"]
impl crate::Resettable for RegSifsTimingCtrlCckSpec {
    const RESET_VALUE: u32 = 0x0404_6046;
}
