#[doc = "Register `REG_ADDRESS_CAM_CTL` reader"]
pub type R = crate::R<RegAddressCamCtlSpec>;
#[doc = "Register `REG_ADDRESS_CAM_CTL` writer"]
pub type W = crate::W<RegAddressCamCtlSpec>;
#[doc = "Field `r_addrcam_num` reader - address filter cam number, up to 16 entries"]
pub type RAddrcamNumR = crate::FieldReader;
#[doc = "Field `r_addrcam_num` writer - address filter cam number, up to 16 entries"]
pub type RAddrcamNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_dis_addrcam` reader - when set, disable address cam filter function"]
pub type RDisAddrcamR = crate::BitReader;
#[doc = "Field `r_dis_addrcam` writer - when set, disable address cam filter function"]
pub type RDisAddrcamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_relay_mode` reader - when set, enable relay mesh function"]
pub type RRelayModeR = crate::BitReader;
#[doc = "Field `r_relay_mode` writer - when set, enable relay mesh function"]
pub type RRelayModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_macid_static_sel` reader - when set, statistic only A1 match packet;otherwise statistic A1 match and my bc/mc packet for macid_match_no_fcs"]
pub type RMacidStaticSelR = crate::BitReader;
#[doc = "Field `r_macid_static_sel` writer - when set, statistic only A1 match packet;otherwise statistic A1 match and my bc/mc packet for macid_match_no_fcs"]
pub type RMacidStaticSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_idunmatch_cnt_clr` writer - when set, generate one pulse and reset cca_idunmatch_cnt"]
pub type RIdunmatchCntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cca_idunmatch_cnt` reader - macid no match counter for mpdu"]
pub type CcaIdunmatchCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - address filter cam number, up to 16 entries"]
    #[inline(always)]
    pub fn r_addrcam_num(&self) -> RAddrcamNumR {
        RAddrcamNumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - when set, disable address cam filter function"]
    #[inline(always)]
    pub fn r_dis_addrcam(&self) -> RDisAddrcamR {
        RDisAddrcamR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when set, enable relay mesh function"]
    #[inline(always)]
    pub fn r_relay_mode(&self) -> RRelayModeR {
        RRelayModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - when set, statistic only A1 match packet;otherwise statistic A1 match and my bc/mc packet for macid_match_no_fcs"]
    #[inline(always)]
    pub fn r_macid_static_sel(&self) -> RMacidStaticSelR {
        RMacidStaticSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - macid no match counter for mpdu"]
    #[inline(always)]
    pub fn cca_idunmatch_cnt(&self) -> CcaIdunmatchCntR {
        CcaIdunmatchCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - address filter cam number, up to 16 entries"]
    #[inline(always)]
    pub fn r_addrcam_num(&mut self) -> RAddrcamNumW<'_, RegAddressCamCtlSpec> {
        RAddrcamNumW::new(self, 0)
    }
    #[doc = "Bit 4 - when set, disable address cam filter function"]
    #[inline(always)]
    pub fn r_dis_addrcam(&mut self) -> RDisAddrcamW<'_, RegAddressCamCtlSpec> {
        RDisAddrcamW::new(self, 4)
    }
    #[doc = "Bit 5 - when set, enable relay mesh function"]
    #[inline(always)]
    pub fn r_relay_mode(&mut self) -> RRelayModeW<'_, RegAddressCamCtlSpec> {
        RRelayModeW::new(self, 5)
    }
    #[doc = "Bit 6 - when set, statistic only A1 match packet;otherwise statistic A1 match and my bc/mc packet for macid_match_no_fcs"]
    #[inline(always)]
    pub fn r_macid_static_sel(&mut self) -> RMacidStaticSelW<'_, RegAddressCamCtlSpec> {
        RMacidStaticSelW::new(self, 6)
    }
    #[doc = "Bit 7 - when set, generate one pulse and reset cca_idunmatch_cnt"]
    #[inline(always)]
    pub fn r_idunmatch_cnt_clr(&mut self) -> RIdunmatchCntClrW<'_, RegAddressCamCtlSpec> {
        RIdunmatchCntClrW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_address_cam_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_address_cam_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddressCamCtlSpec;
impl crate::RegisterSpec for RegAddressCamCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_address_cam_ctl::R`](R) reader structure"]
impl crate::Readable for RegAddressCamCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_address_cam_ctl::W`](W) writer structure"]
impl crate::Writable for RegAddressCamCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESS_CAM_CTL to value 0x1f"]
impl crate::Resettable for RegAddressCamCtlSpec {
    const RESET_VALUE: u32 = 0x1f;
}
