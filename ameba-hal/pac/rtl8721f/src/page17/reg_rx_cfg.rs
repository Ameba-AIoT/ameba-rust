#[doc = "Register `REG_RX_CFG` reader"]
pub type R = crate::R<RegRxCfgSpec>;
#[doc = "Register `REG_RX_CFG` writer"]
pub type W = crate::W<RegRxCfgSpec>;
#[doc = "Field `r_rxpsf_vht_su_chken` reader - enable VHT SU invalid packet check for mac header"]
pub type RRxpsfVhtSuChkenR = crate::BitReader;
#[doc = "Field `r_rxpsf_vht_su_chken` writer - enable VHT SU invalid packet check for mac header"]
pub type RRxpsfVhtSuChkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxpsf_vht_mu_chken` reader - enable VHT MU invalid packet check for mac header"]
pub type RRxpsfVhtMuChkenR = crate::BitReader;
#[doc = "Field `r_rxpsf_vht_mu_chken` writer - enable VHT MU invalid packet check for mac header"]
pub type RRxpsfVhtMuChkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxpsf_he_su_chken` reader - enable HE SU invalid packet check for mac header"]
pub type RRxpsfHeSuChkenR = crate::BitReader;
#[doc = "Field `r_rxpsf_he_su_chken` writer - enable HE SU invalid packet check for mac header"]
pub type RRxpsfHeSuChkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxpsf_he_ersu_chken` reader - enable HE ER SU invalid packet check for mac header"]
pub type RRxpsfHeErsuChkenR = crate::BitReader;
#[doc = "Field `r_rxpsf_he_ersu_chken` writer - enable HE ER SU invalid packet check for mac header"]
pub type RRxpsfHeErsuChkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxpsf_he_mu_chken` reader - enable HE MU invalid packet check for mac header"]
pub type RRxpsfHeMuChkenR = crate::BitReader;
#[doc = "Field `r_rxpsf_he_mu_chken` writer - enable HE MU invalid packet check for mac header"]
pub type RRxpsfHeMuChkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxpsf_inter_mpdu` reader - enable invalid packet check for inter ppdu, inter ppdu valid when plcp and mac header are inter bss"]
pub type RRxpsfInterMpduR = crate::BitReader;
#[doc = "Field `r_rxpsf_inter_mpdu` writer - enable invalid packet check for inter ppdu, inter ppdu valid when plcp and mac header are inter bss"]
pub type RRxpsfInterMpduW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxpsf_inter_plcp` reader - enable invalid packet check for inter ppdu, inter ppdu valid when plcp is inter bss"]
pub type RRxpsfInterPlcpR = crate::BitReader;
#[doc = "Field `r_rxpsf_inter_plcp` writer - enable invalid packet check for inter ppdu, inter ppdu valid when plcp is inter bss"]
pub type RRxpsfInterPlcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sniffer_tb` reader - enable to receive TB PPDU for debug mode"]
pub type RSnifferTbR = crate::BitReader;
#[doc = "Field `r_sniffer_tb` writer - enable to receive TB PPDU for debug mode"]
pub type RSnifferTbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_en_mach_upd_len` reader - enable macrx_pktfifo_ctl update mpdu length in MACHDR_CHK state"]
pub type REnMachUpdLenR = crate::BitReader;
#[doc = "Field `r_en_mach_upd_len` writer - enable macrx_pktfifo_ctl update mpdu length in MACHDR_CHK state"]
pub type REnMachUpdLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_inter_ppdu_reject` reader - enable reject inter ppdu"]
pub type RInterPpduRejectR = crate::BitReader;
#[doc = "Field `r_inter_ppdu_reject` writer - enable reject inter ppdu"]
pub type RInterPpduRejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sts_chkrx_cond` reader - check macrx condition when receiving ppdu status"]
pub type RStsChkrxCondR = crate::FieldReader;
#[doc = "Field `r_sts_chkrx_cond` writer - check macrx condition when receiving ppdu status"]
pub type RStsChkrxCondW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_dis_sts_gclk` reader - disable ppdu status clock gating"]
pub type RDisStsGclkR = crate::BitReader;
#[doc = "Field `r_dis_sts_gclk` writer - disable ppdu status clock gating"]
pub type RDisStsGclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_chk_ctl_bssid` reader - enable check trigger,vht/hendpa and block ack bssid"]
pub type RChkCtlBssidR = crate::BitReader;
#[doc = "Field `r_chk_ctl_bssid` writer - enable check trigger,vht/hendpa and block ack bssid"]
pub type RChkCtlBssidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_chk_mgnt_bssid` reader - enable check management frame bssid"]
pub type RChkMgntBssidR = crate::BitReader;
#[doc = "Field `r_chk_mgnt_bssid` writer - enable check management frame bssid"]
pub type RChkMgntBssidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dis_chk_hendp_len` reader - disable check HE NDP lsig length value"]
pub type RDisChkHendpLenR = crate::BitReader;
#[doc = "Field `r_dis_chk_hendp_len` writer - disable check HE NDP lsig length value"]
pub type RDisChkHendpLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tim_no_hit_reject` reader - enable reject tim no hit beacon"]
pub type RTimNoHitRejectR = crate::BitReader;
#[doc = "Field `r_tim_no_hit_reject` writer - enable reject tim no hit beacon"]
pub type RTimNoHitRejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rxbcn_tim_no_hit_en` reader - enable rxbcn_ok_int when receiving bcn with tim_no_hit,set 0: rxbcn_ok_int will only assert when tim_hit"]
pub type RRxbcnTimNoHitEnR = crate::BitReader;
#[doc = "Field `r_rxbcn_tim_no_hit_en` writer - enable rxbcn_ok_int when receiving bcn with tim_no_hit,set 0: rxbcn_ok_int will only assert when tim_hit"]
pub type RRxbcnTimNoHitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_mba_rsvd_aid_en` reader - enable parsing resevd aid msta-ba for pre-association"]
pub type RMbaRsvdAidEnR = crate::BitReader;
#[doc = "Field `r_mba_rsvd_aid_en` writer - enable parsing resevd aid msta-ba for pre-association"]
pub type RMbaRsvdAidEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dis_he_sigb_chk` reader - when set, disable check he-mu sigb crc8"]
pub type RDisHeSigbChkR = crate::BitReader;
#[doc = "Field `r_dis_he_sigb_chk` writer - when set, disable check he-mu sigb crc8"]
pub type RDisHeSigbChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_vhtndpa_suaid_0_en` reader - when set, will receive vht/he su ndpa which aid field is set to 0."]
pub type RVhtndpaSuaid0EnR = crate::BitReader;
#[doc = "Field `r_vhtndpa_suaid_0_en` writer - when set, will receive vht/he su ndpa which aid field is set to 0."]
pub type RVhtndpaSuaid0EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable VHT SU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_vht_su_chken(&self) -> RRxpsfVhtSuChkenR {
        RRxpsfVhtSuChkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable VHT MU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_vht_mu_chken(&self) -> RRxpsfVhtMuChkenR {
        RRxpsfVhtMuChkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable HE SU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_he_su_chken(&self) -> RRxpsfHeSuChkenR {
        RRxpsfHeSuChkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable HE ER SU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_he_ersu_chken(&self) -> RRxpsfHeErsuChkenR {
        RRxpsfHeErsuChkenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable HE MU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_he_mu_chken(&self) -> RRxpsfHeMuChkenR {
        RRxpsfHeMuChkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable invalid packet check for inter ppdu, inter ppdu valid when plcp and mac header are inter bss"]
    #[inline(always)]
    pub fn r_rxpsf_inter_mpdu(&self) -> RRxpsfInterMpduR {
        RRxpsfInterMpduR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable invalid packet check for inter ppdu, inter ppdu valid when plcp is inter bss"]
    #[inline(always)]
    pub fn r_rxpsf_inter_plcp(&self) -> RRxpsfInterPlcpR {
        RRxpsfInterPlcpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - enable to receive TB PPDU for debug mode"]
    #[inline(always)]
    pub fn r_sniffer_tb(&self) -> RSnifferTbR {
        RSnifferTbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - enable macrx_pktfifo_ctl update mpdu length in MACHDR_CHK state"]
    #[inline(always)]
    pub fn r_en_mach_upd_len(&self) -> REnMachUpdLenR {
        REnMachUpdLenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - enable reject inter ppdu"]
    #[inline(always)]
    pub fn r_inter_ppdu_reject(&self) -> RInterPpduRejectR {
        RInterPpduRejectR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - check macrx condition when receiving ppdu status"]
    #[inline(always)]
    pub fn r_sts_chkrx_cond(&self) -> RStsChkrxCondR {
        RStsChkrxCondR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - disable ppdu status clock gating"]
    #[inline(always)]
    pub fn r_dis_sts_gclk(&self) -> RDisStsGclkR {
        RDisStsGclkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - enable check trigger,vht/hendpa and block ack bssid"]
    #[inline(always)]
    pub fn r_chk_ctl_bssid(&self) -> RChkCtlBssidR {
        RChkCtlBssidR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable check management frame bssid"]
    #[inline(always)]
    pub fn r_chk_mgnt_bssid(&self) -> RChkMgntBssidR {
        RChkMgntBssidR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - disable check HE NDP lsig length value"]
    #[inline(always)]
    pub fn r_dis_chk_hendp_len(&self) -> RDisChkHendpLenR {
        RDisChkHendpLenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable reject tim no hit beacon"]
    #[inline(always)]
    pub fn r_tim_no_hit_reject(&self) -> RTimNoHitRejectR {
        RTimNoHitRejectR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable rxbcn_ok_int when receiving bcn with tim_no_hit,set 0: rxbcn_ok_int will only assert when tim_hit"]
    #[inline(always)]
    pub fn r_rxbcn_tim_no_hit_en(&self) -> RRxbcnTimNoHitEnR {
        RRxbcnTimNoHitEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable parsing resevd aid msta-ba for pre-association"]
    #[inline(always)]
    pub fn r_mba_rsvd_aid_en(&self) -> RMbaRsvdAidEnR {
        RMbaRsvdAidEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - when set, disable check he-mu sigb crc8"]
    #[inline(always)]
    pub fn r_dis_he_sigb_chk(&self) -> RDisHeSigbChkR {
        RDisHeSigbChkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - when set, will receive vht/he su ndpa which aid field is set to 0."]
    #[inline(always)]
    pub fn r_vhtndpa_suaid_0_en(&self) -> RVhtndpaSuaid0EnR {
        RVhtndpaSuaid0EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable VHT SU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_vht_su_chken(&mut self) -> RRxpsfVhtSuChkenW<'_, RegRxCfgSpec> {
        RRxpsfVhtSuChkenW::new(self, 0)
    }
    #[doc = "Bit 1 - enable VHT MU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_vht_mu_chken(&mut self) -> RRxpsfVhtMuChkenW<'_, RegRxCfgSpec> {
        RRxpsfVhtMuChkenW::new(self, 1)
    }
    #[doc = "Bit 2 - enable HE SU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_he_su_chken(&mut self) -> RRxpsfHeSuChkenW<'_, RegRxCfgSpec> {
        RRxpsfHeSuChkenW::new(self, 2)
    }
    #[doc = "Bit 3 - enable HE ER SU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_he_ersu_chken(&mut self) -> RRxpsfHeErsuChkenW<'_, RegRxCfgSpec> {
        RRxpsfHeErsuChkenW::new(self, 3)
    }
    #[doc = "Bit 4 - enable HE MU invalid packet check for mac header"]
    #[inline(always)]
    pub fn r_rxpsf_he_mu_chken(&mut self) -> RRxpsfHeMuChkenW<'_, RegRxCfgSpec> {
        RRxpsfHeMuChkenW::new(self, 4)
    }
    #[doc = "Bit 5 - enable invalid packet check for inter ppdu, inter ppdu valid when plcp and mac header are inter bss"]
    #[inline(always)]
    pub fn r_rxpsf_inter_mpdu(&mut self) -> RRxpsfInterMpduW<'_, RegRxCfgSpec> {
        RRxpsfInterMpduW::new(self, 5)
    }
    #[doc = "Bit 6 - enable invalid packet check for inter ppdu, inter ppdu valid when plcp is inter bss"]
    #[inline(always)]
    pub fn r_rxpsf_inter_plcp(&mut self) -> RRxpsfInterPlcpW<'_, RegRxCfgSpec> {
        RRxpsfInterPlcpW::new(self, 6)
    }
    #[doc = "Bit 16 - enable to receive TB PPDU for debug mode"]
    #[inline(always)]
    pub fn r_sniffer_tb(&mut self) -> RSnifferTbW<'_, RegRxCfgSpec> {
        RSnifferTbW::new(self, 16)
    }
    #[doc = "Bit 17 - enable macrx_pktfifo_ctl update mpdu length in MACHDR_CHK state"]
    #[inline(always)]
    pub fn r_en_mach_upd_len(&mut self) -> REnMachUpdLenW<'_, RegRxCfgSpec> {
        REnMachUpdLenW::new(self, 17)
    }
    #[doc = "Bit 18 - enable reject inter ppdu"]
    #[inline(always)]
    pub fn r_inter_ppdu_reject(&mut self) -> RInterPpduRejectW<'_, RegRxCfgSpec> {
        RInterPpduRejectW::new(self, 18)
    }
    #[doc = "Bits 19:22 - check macrx condition when receiving ppdu status"]
    #[inline(always)]
    pub fn r_sts_chkrx_cond(&mut self) -> RStsChkrxCondW<'_, RegRxCfgSpec> {
        RStsChkrxCondW::new(self, 19)
    }
    #[doc = "Bit 23 - disable ppdu status clock gating"]
    #[inline(always)]
    pub fn r_dis_sts_gclk(&mut self) -> RDisStsGclkW<'_, RegRxCfgSpec> {
        RDisStsGclkW::new(self, 23)
    }
    #[doc = "Bit 24 - enable check trigger,vht/hendpa and block ack bssid"]
    #[inline(always)]
    pub fn r_chk_ctl_bssid(&mut self) -> RChkCtlBssidW<'_, RegRxCfgSpec> {
        RChkCtlBssidW::new(self, 24)
    }
    #[doc = "Bit 25 - enable check management frame bssid"]
    #[inline(always)]
    pub fn r_chk_mgnt_bssid(&mut self) -> RChkMgntBssidW<'_, RegRxCfgSpec> {
        RChkMgntBssidW::new(self, 25)
    }
    #[doc = "Bit 26 - disable check HE NDP lsig length value"]
    #[inline(always)]
    pub fn r_dis_chk_hendp_len(&mut self) -> RDisChkHendpLenW<'_, RegRxCfgSpec> {
        RDisChkHendpLenW::new(self, 26)
    }
    #[doc = "Bit 27 - enable reject tim no hit beacon"]
    #[inline(always)]
    pub fn r_tim_no_hit_reject(&mut self) -> RTimNoHitRejectW<'_, RegRxCfgSpec> {
        RTimNoHitRejectW::new(self, 27)
    }
    #[doc = "Bit 28 - enable rxbcn_ok_int when receiving bcn with tim_no_hit,set 0: rxbcn_ok_int will only assert when tim_hit"]
    #[inline(always)]
    pub fn r_rxbcn_tim_no_hit_en(&mut self) -> RRxbcnTimNoHitEnW<'_, RegRxCfgSpec> {
        RRxbcnTimNoHitEnW::new(self, 28)
    }
    #[doc = "Bit 29 - enable parsing resevd aid msta-ba for pre-association"]
    #[inline(always)]
    pub fn r_mba_rsvd_aid_en(&mut self) -> RMbaRsvdAidEnW<'_, RegRxCfgSpec> {
        RMbaRsvdAidEnW::new(self, 29)
    }
    #[doc = "Bit 30 - when set, disable check he-mu sigb crc8"]
    #[inline(always)]
    pub fn r_dis_he_sigb_chk(&mut self) -> RDisHeSigbChkW<'_, RegRxCfgSpec> {
        RDisHeSigbChkW::new(self, 30)
    }
    #[doc = "Bit 31 - when set, will receive vht/he su ndpa which aid field is set to 0."]
    #[inline(always)]
    pub fn r_vhtndpa_suaid_0_en(&mut self) -> RVhtndpaSuaid0EnW<'_, RegRxCfgSpec> {
        RVhtndpaSuaid0EnW::new(self, 31)
    }
}
#[doc = "rx configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxCfgSpec;
impl crate::RegisterSpec for RegRxCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_cfg::R`](R) reader structure"]
impl crate::Readable for RegRxCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_cfg::W`](W) writer structure"]
impl crate::Writable for RegRxCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_CFG to value 0"]
impl crate::Resettable for RegRxCfgSpec {}
