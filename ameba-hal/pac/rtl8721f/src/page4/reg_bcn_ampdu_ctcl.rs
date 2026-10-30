#[doc = "Register `REG_BCN_AMPDU_CTCL` reader"]
pub type R = crate::R<RegBcnAmpduCtclSpec>;
#[doc = "Register `REG_BCN_AMPDU_CTCL` writer"]
pub type W = crate::W<RegBcnAmpduCtclSpec>;
#[doc = "Field `BIT_EN_MACID_RELEASE` reader - 1: Enable MACID Release function. (same as MACID DROP) 0: Disable MACID Release function."]
pub type BitEnMacidReleaseR = crate::BitReader;
#[doc = "Field `BIT_EN_MACID_RELEASE` writer - 1: Enable MACID Release function. (same as MACID DROP) 0: Disable MACID Release function."]
pub type BitEnMacidReleaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_CLEAR_MACID_RELEASE` reader - 1: HW No clear MACID Release bit after release the pkt. 0: HW will clear MACID Release bit after release the pkt."]
pub type BitDisClearMacidReleaseR = crate::BitReader;
#[doc = "Field `BIT_DIS_CLEAR_MACID_RELEASE` writer - 1: HW No clear MACID Release bit after release the pkt. 0: HW will clear MACID Release bit after release the pkt."]
pub type BitDisClearMacidReleaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_SET_MOREDATA` reader - 1: Enable HW set the more data bit of the PKT that MACID release function tx. 0: Disable HW set the more data bit of the PKT that MACID release function tx."]
pub type BitEnSetMoredataR = crate::BitReader;
#[doc = "Field `BIT_EN_SET_MOREDATA` writer - 1: Enable HW set the more data bit of the PKT that MACID release function tx. 0: Disable HW set the more data bit of the PKT that MACID release function tx."]
pub type BitEnSetMoredataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_CLR_CMD_REL_BCN_PKT` reader - If this bit is set, the BCN pkt will be set to non-valid when scheduler clear TX BCN CMD even non-TX."]
pub type BitEnClrCmdRelBcnPktR = crate::BitReader;
#[doc = "Field `BIT_EN_CLR_CMD_REL_BCN_PKT` writer - If this bit is set, the BCN pkt will be set to non-valid when scheduler clear TX BCN CMD even non-TX."]
pub type BitEnClrCmdRelBcnPktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MOREDATA_BYPASS` reader - If this bit is set, the moredata bit will controlled by DES, otherwise controlled by PTCL according TXPKTBUF status."]
pub type BitMoredataBypassR = crate::BitReader;
#[doc = "Field `BIT_MOREDATA_BYPASS` writer - If this bit is set, the moredata bit will controlled by DES, otherwise controlled by PTCL according TXPKTBUF status."]
pub type BitMoredataBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCN_PORT_SEL` reader - This bit use to select BCN on port 1 or port 0. This just useful in USB/SDIO interface."]
pub type BitBcnPortSelR = crate::BitReader;
#[doc = "Field `BIT_BCN_PORT_SEL` writer - This bit use to select BCN on port 1 or port 0. This just useful in USB/SDIO interface."]
pub type BitBcnPortSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BCN_PKT_REL` reader - If this bit is set, the BCN pkt will be set to non-valid after BCN TX."]
pub type BitEnBcnPktRelR = crate::BitReader;
#[doc = "Field `BIT_EN_BCN_PKT_REL` writer - If this bit is set, the BCN pkt will be set to non-valid after BCN TX."]
pub type BitEnBcnPktRelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHECK_CCK_EN` reader - If this bit is set, HW check Tx pkt's data rate. When data rate is CCK rate, modify the rate to NON_CCK_RATE."]
pub type BitCheckCckEnR = crate::BitReader;
#[doc = "Field `BIT_CHECK_CCK_EN` writer - If this bit is set, HW check Tx pkt's data rate. When data rate is CCK rate, modify the rate to NON_CCK_RATE."]
pub type BitCheckCckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCNQ2_HEAD` reader - TXDMA will pull BCNQ2 Pkt in this address. Port2, NAN"]
pub type BitBcnq2HeadR = crate::FieldReader;
#[doc = "Field `BIT_BCNQ2_HEAD` writer - TXDMA will pull BCNQ2 Pkt in this address. Port2, NAN"]
pub type BitBcnq2HeadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_AMPDU_MAX_TIME` reader - AMPDU duration limit in the unit of 32us."]
pub type BitAmpduMaxTimeR = crate::FieldReader;
#[doc = "Field `BIT_AMPDU_MAX_TIME` writer - AMPDU duration limit in the unit of 32us."]
pub type BitAmpduMaxTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCNQ1_HEAD` reader - TXDMA will pull BCNQ1 Pkt in this address. Port1 & VAP 1/3/5/7 of port0"]
pub type BitBcnq1HeadR = crate::FieldReader;
#[doc = "Field `BIT_BCNQ1_HEAD` writer - TXDMA will pull BCNQ1 Pkt in this address. Port1 & VAP 1/3/5/7 of port0"]
pub type BitBcnq1HeadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 1: Enable MACID Release function. (same as MACID DROP) 0: Disable MACID Release function."]
    #[inline(always)]
    pub fn bit_en_macid_release(&self) -> BitEnMacidReleaseR {
        BitEnMacidReleaseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: HW No clear MACID Release bit after release the pkt. 0: HW will clear MACID Release bit after release the pkt."]
    #[inline(always)]
    pub fn bit_dis_clear_macid_release(&self) -> BitDisClearMacidReleaseR {
        BitDisClearMacidReleaseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable HW set the more data bit of the PKT that MACID release function tx. 0: Disable HW set the more data bit of the PKT that MACID release function tx."]
    #[inline(always)]
    pub fn bit_en_set_moredata(&self) -> BitEnSetMoredataR {
        BitEnSetMoredataR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set, the BCN pkt will be set to non-valid when scheduler clear TX BCN CMD even non-TX."]
    #[inline(always)]
    pub fn bit_en_clr_cmd_rel_bcn_pkt(&self) -> BitEnClrCmdRelBcnPktR {
        BitEnClrCmdRelBcnPktR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set, the moredata bit will controlled by DES, otherwise controlled by PTCL according TXPKTBUF status."]
    #[inline(always)]
    pub fn bit_moredata_bypass(&self) -> BitMoredataBypassR {
        BitMoredataBypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit use to select BCN on port 1 or port 0. This just useful in USB/SDIO interface."]
    #[inline(always)]
    pub fn bit_bcn_port_sel(&self) -> BitBcnPortSelR {
        BitBcnPortSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is set, the BCN pkt will be set to non-valid after BCN TX."]
    #[inline(always)]
    pub fn bit_en_bcn_pkt_rel(&self) -> BitEnBcnPktRelR {
        BitEnBcnPktRelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If this bit is set, HW check Tx pkt's data rate. When data rate is CCK rate, modify the rate to NON_CCK_RATE."]
    #[inline(always)]
    pub fn bit_check_cck_en(&self) -> BitCheckCckEnR {
        BitCheckCckEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - TXDMA will pull BCNQ2 Pkt in this address. Port2, NAN"]
    #[inline(always)]
    pub fn bit_bcnq2_head(&self) -> BitBcnq2HeadR {
        BitBcnq2HeadR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AMPDU duration limit in the unit of 32us."]
    #[inline(always)]
    pub fn bit_ampdu_max_time(&self) -> BitAmpduMaxTimeR {
        BitAmpduMaxTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TXDMA will pull BCNQ1 Pkt in this address. Port1 & VAP 1/3/5/7 of port0"]
    #[inline(always)]
    pub fn bit_bcnq1_head(&self) -> BitBcnq1HeadR {
        BitBcnq1HeadR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable MACID Release function. (same as MACID DROP) 0: Disable MACID Release function."]
    #[inline(always)]
    pub fn bit_en_macid_release(&mut self) -> BitEnMacidReleaseW<'_, RegBcnAmpduCtclSpec> {
        BitEnMacidReleaseW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: HW No clear MACID Release bit after release the pkt. 0: HW will clear MACID Release bit after release the pkt."]
    #[inline(always)]
    pub fn bit_dis_clear_macid_release(
        &mut self,
    ) -> BitDisClearMacidReleaseW<'_, RegBcnAmpduCtclSpec> {
        BitDisClearMacidReleaseW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Enable HW set the more data bit of the PKT that MACID release function tx. 0: Disable HW set the more data bit of the PKT that MACID release function tx."]
    #[inline(always)]
    pub fn bit_en_set_moredata(&mut self) -> BitEnSetMoredataW<'_, RegBcnAmpduCtclSpec> {
        BitEnSetMoredataW::new(self, 2)
    }
    #[doc = "Bit 3 - If this bit is set, the BCN pkt will be set to non-valid when scheduler clear TX BCN CMD even non-TX."]
    #[inline(always)]
    pub fn bit_en_clr_cmd_rel_bcn_pkt(&mut self) -> BitEnClrCmdRelBcnPktW<'_, RegBcnAmpduCtclSpec> {
        BitEnClrCmdRelBcnPktW::new(self, 3)
    }
    #[doc = "Bit 4 - If this bit is set, the moredata bit will controlled by DES, otherwise controlled by PTCL according TXPKTBUF status."]
    #[inline(always)]
    pub fn bit_moredata_bypass(&mut self) -> BitMoredataBypassW<'_, RegBcnAmpduCtclSpec> {
        BitMoredataBypassW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit use to select BCN on port 1 or port 0. This just useful in USB/SDIO interface."]
    #[inline(always)]
    pub fn bit_bcn_port_sel(&mut self) -> BitBcnPortSelW<'_, RegBcnAmpduCtclSpec> {
        BitBcnPortSelW::new(self, 5)
    }
    #[doc = "Bit 6 - If this bit is set, the BCN pkt will be set to non-valid after BCN TX."]
    #[inline(always)]
    pub fn bit_en_bcn_pkt_rel(&mut self) -> BitEnBcnPktRelW<'_, RegBcnAmpduCtclSpec> {
        BitEnBcnPktRelW::new(self, 6)
    }
    #[doc = "Bit 7 - If this bit is set, HW check Tx pkt's data rate. When data rate is CCK rate, modify the rate to NON_CCK_RATE."]
    #[inline(always)]
    pub fn bit_check_cck_en(&mut self) -> BitCheckCckEnW<'_, RegBcnAmpduCtclSpec> {
        BitCheckCckEnW::new(self, 7)
    }
    #[doc = "Bits 8:15 - TXDMA will pull BCNQ2 Pkt in this address. Port2, NAN"]
    #[inline(always)]
    pub fn bit_bcnq2_head(&mut self) -> BitBcnq2HeadW<'_, RegBcnAmpduCtclSpec> {
        BitBcnq2HeadW::new(self, 8)
    }
    #[doc = "Bits 16:23 - AMPDU duration limit in the unit of 32us."]
    #[inline(always)]
    pub fn bit_ampdu_max_time(&mut self) -> BitAmpduMaxTimeW<'_, RegBcnAmpduCtclSpec> {
        BitAmpduMaxTimeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - TXDMA will pull BCNQ1 Pkt in this address. Port1 & VAP 1/3/5/7 of port0"]
    #[inline(always)]
    pub fn bit_bcnq1_head(&mut self) -> BitBcnq1HeadW<'_, RegBcnAmpduCtclSpec> {
        BitBcnq1HeadW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_ampdu_ctcl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_ampdu_ctcl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnAmpduCtclSpec;
impl crate::RegisterSpec for RegBcnAmpduCtclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_ampdu_ctcl::R`](R) reader structure"]
impl crate::Readable for RegBcnAmpduCtclSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_ampdu_ctcl::W`](W) writer structure"]
impl crate::Writable for RegBcnAmpduCtclSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_AMPDU_CTCL to value 0xf664_f625"]
impl crate::Resettable for RegBcnAmpduCtclSpec {
    const RESET_VALUE: u32 = 0xf664_f625;
}
