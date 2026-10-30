#[doc = "Register `REG_PTA_STBC_CTRL` reader"]
pub type R = crate::R<RegPtaStbcCtrlSpec>;
#[doc = "Register `REG_PTA_STBC_CTRL` writer"]
pub type W = crate::W<RegPtaStbcCtrlSpec>;
#[doc = "Field `BIT_STBC_CFEND` reader - CFEND STBC"]
pub type BitStbcCfendR = crate::FieldReader;
#[doc = "Field `BIT_STBC_CFEND` writer - CFEND STBC"]
pub type BitStbcCfendW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_NESS` reader - NESS"]
pub type BitNessR = crate::FieldReader;
#[doc = "Field `BIT_NESS` writer - NESS"]
pub type BitNessW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_Cdend_txtime_L` reader - Cfend packet txtime bit\\[3:0\\]"]
pub type BitCdendTxtimeLR = crate::FieldReader;
#[doc = "Field `BIT_Cdend_txtime_L` writer - Cfend packet txtime bit\\[3:0\\]"]
pub type BitCdendTxtimeLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_Cdend_txtime_H` reader - Cfend packet txtime bit\\[8:4\\]"]
pub type BitCdendTxtimeHR = crate::FieldReader;
#[doc = "Field `BIT_Cdend_txtime_H` writer - Cfend packet txtime bit\\[8:4\\]"]
pub type BitCdendTxtimeHW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_ACQ_MODE_SEL` reader - 1: Enable 88C 4AC Mode 0: Enable 8723 ACQ0~7 Mode"]
pub type BitAcqModeSelR = crate::BitReader;
#[doc = "Field `BIT_ACQ_MODE_SEL` writer - 1: Enable 88C 4AC Mode 0: Enable 8723 ACQ0~7 Mode"]
pub type BitAcqModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TRI_PKT_PKTIN` reader - If trigger frame is transmitted into txpktbuf, HW generates interrupt 1: Enable 0: Disable"]
pub type BitTriPktPktinR = crate::BitReader;
#[doc = "Field `BIT_TRI_PKT_PKTIN` writer - If trigger frame is transmitted into txpktbuf, HW generates interrupt 1: Enable 0: Disable"]
pub type BitTriPktPktinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TRI_PKT_STATUS` reader - "]
pub type BitTriPktStatusR = crate::BitReader;
#[doc = "Field `BIT_TRI_PKT_STATUS` writer - "]
pub type BitTriPktStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_use_data_bw` reader - 1 : Use des2_data_bw for this packet 0 : when des1_cca_rts = 2'b10, update_bw depend on ch_bw_status"]
pub type BitRUseDataBwR = crate::BitReader;
#[doc = "Field `BIT_r_use_data_bw` writer - 1 : Use des2_data_bw for this packet 0 : when des1_cca_rts = 2'b10, update_bw depend on ch_bw_status"]
pub type BitRUseDataBwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_wl_tx_en` reader - enable WL PTA (should be set in init MAC)"]
pub type BitPtaWlTxEnR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_tx_en` writer - enable WL PTA (should be set in init MAC)"]
pub type BitPtaWlTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_edcca_en` reader - 0 : wl_tx signal to PTA does not consider EDCCA 1 : wl_tx signal to PTA consider EDCCA"]
pub type BitPtaEdccaEnR = crate::BitReader;
#[doc = "Field `BIT_pta_edcca_en` writer - 0 : wl_tx signal to PTA does not consider EDCCA 1 : wl_tx signal to PTA consider EDCCA"]
pub type BitPtaEdccaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FORCE_RND_PRI` reader - queue select priority control 1:old round robin method 0:depend on last selected queue"]
pub type BitForceRndPriR = crate::BitReader;
#[doc = "Field `BIT_FORCE_RND_PRI` writer - queue select priority control 1:old round robin method 0:depend on last selected queue"]
pub type BitForceRndPriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_SINGLE_APMDU` reader - If this bit is set, Enable single sub-frame APMDU."]
pub type BitEnSingleApmduR = crate::BitReader;
#[doc = "Field `BIT_EN_SINGLE_APMDU` writer - If this bit is set, Enable single sub-frame APMDU."]
pub type BitEnSingleApmduW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CFEND STBC"]
    #[inline(always)]
    pub fn bit_stbc_cfend(&self) -> BitStbcCfendR {
        BitStbcCfendR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - NESS"]
    #[inline(always)]
    pub fn bit_ness(&self) -> BitNessR {
        BitNessR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Cfend packet txtime bit\\[3:0\\]"]
    #[inline(always)]
    pub fn bit_cdend_txtime_l(&self) -> BitCdendTxtimeLR {
        BitCdendTxtimeLR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Cfend packet txtime bit\\[8:4\\]"]
    #[inline(always)]
    pub fn bit_cdend_txtime_h(&self) -> BitCdendTxtimeHR {
        BitCdendTxtimeHR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 1: Enable 88C 4AC Mode 0: Enable 8723 ACQ0~7 Mode"]
    #[inline(always)]
    pub fn bit_acq_mode_sel(&self) -> BitAcqModeSelR {
        BitAcqModeSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If trigger frame is transmitted into txpktbuf, HW generates interrupt 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_tri_pkt_pktin(&self) -> BitTriPktPktinR {
        BitTriPktPktinR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bit_tri_pkt_status(&self) -> BitTriPktStatusR {
        BitTriPktStatusR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1 : Use des2_data_bw for this packet 0 : when des1_cca_rts = 2'b10, update_bw depend on ch_bw_status"]
    #[inline(always)]
    pub fn bit_r_use_data_bw(&self) -> BitRUseDataBwR {
        BitRUseDataBwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enable WL PTA (should be set in init MAC)"]
    #[inline(always)]
    pub fn bit_pta_wl_tx_en(&self) -> BitPtaWlTxEnR {
        BitPtaWlTxEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 0 : wl_tx signal to PTA does not consider EDCCA 1 : wl_tx signal to PTA consider EDCCA"]
    #[inline(always)]
    pub fn bit_pta_edcca_en(&self) -> BitPtaEdccaEnR {
        BitPtaEdccaEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - queue select priority control 1:old round robin method 0:depend on last selected queue"]
    #[inline(always)]
    pub fn bit_force_rnd_pri(&self) -> BitForceRndPriR {
        BitForceRndPriR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - If this bit is set, Enable single sub-frame APMDU."]
    #[inline(always)]
    pub fn bit_en_single_apmdu(&self) -> BitEnSingleApmduR {
        BitEnSingleApmduR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CFEND STBC"]
    #[inline(always)]
    pub fn bit_stbc_cfend(&mut self) -> BitStbcCfendW<'_, RegPtaStbcCtrlSpec> {
        BitStbcCfendW::new(self, 0)
    }
    #[doc = "Bits 2:3 - NESS"]
    #[inline(always)]
    pub fn bit_ness(&mut self) -> BitNessW<'_, RegPtaStbcCtrlSpec> {
        BitNessW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Cfend packet txtime bit\\[3:0\\]"]
    #[inline(always)]
    pub fn bit_cdend_txtime_l(&mut self) -> BitCdendTxtimeLW<'_, RegPtaStbcCtrlSpec> {
        BitCdendTxtimeLW::new(self, 4)
    }
    #[doc = "Bits 8:12 - Cfend packet txtime bit\\[8:4\\]"]
    #[inline(always)]
    pub fn bit_cdend_txtime_h(&mut self) -> BitCdendTxtimeHW<'_, RegPtaStbcCtrlSpec> {
        BitCdendTxtimeHW::new(self, 8)
    }
    #[doc = "Bit 16 - 1: Enable 88C 4AC Mode 0: Enable 8723 ACQ0~7 Mode"]
    #[inline(always)]
    pub fn bit_acq_mode_sel(&mut self) -> BitAcqModeSelW<'_, RegPtaStbcCtrlSpec> {
        BitAcqModeSelW::new(self, 16)
    }
    #[doc = "Bit 17 - If trigger frame is transmitted into txpktbuf, HW generates interrupt 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_tri_pkt_pktin(&mut self) -> BitTriPktPktinW<'_, RegPtaStbcCtrlSpec> {
        BitTriPktPktinW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bit_tri_pkt_status(&mut self) -> BitTriPktStatusW<'_, RegPtaStbcCtrlSpec> {
        BitTriPktStatusW::new(self, 18)
    }
    #[doc = "Bit 19 - 1 : Use des2_data_bw for this packet 0 : when des1_cca_rts = 2'b10, update_bw depend on ch_bw_status"]
    #[inline(always)]
    pub fn bit_r_use_data_bw(&mut self) -> BitRUseDataBwW<'_, RegPtaStbcCtrlSpec> {
        BitRUseDataBwW::new(self, 19)
    }
    #[doc = "Bit 20 - enable WL PTA (should be set in init MAC)"]
    #[inline(always)]
    pub fn bit_pta_wl_tx_en(&mut self) -> BitPtaWlTxEnW<'_, RegPtaStbcCtrlSpec> {
        BitPtaWlTxEnW::new(self, 20)
    }
    #[doc = "Bit 21 - 0 : wl_tx signal to PTA does not consider EDCCA 1 : wl_tx signal to PTA consider EDCCA"]
    #[inline(always)]
    pub fn bit_pta_edcca_en(&mut self) -> BitPtaEdccaEnW<'_, RegPtaStbcCtrlSpec> {
        BitPtaEdccaEnW::new(self, 21)
    }
    #[doc = "Bit 22 - queue select priority control 1:old round robin method 0:depend on last selected queue"]
    #[inline(always)]
    pub fn bit_force_rnd_pri(&mut self) -> BitForceRndPriW<'_, RegPtaStbcCtrlSpec> {
        BitForceRndPriW::new(self, 22)
    }
    #[doc = "Bit 31 - If this bit is set, Enable single sub-frame APMDU."]
    #[inline(always)]
    pub fn bit_en_single_apmdu(&mut self) -> BitEnSingleApmduW<'_, RegPtaStbcCtrlSpec> {
        BitEnSingleApmduW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pta_stbc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pta_stbc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPtaStbcCtrlSpec;
impl crate::RegisterSpec for RegPtaStbcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pta_stbc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPtaStbcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pta_stbc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPtaStbcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PTA_STBC_CTRL to value 0x0004_0340"]
impl crate::Resettable for RegPtaStbcCtrlSpec {
    const RESET_VALUE: u32 = 0x0004_0340;
}
