#[doc = "Register `REG_TB_CTRL` reader"]
pub type R = crate::R<RegTbCtrlSpec>;
#[doc = "Register `REG_TB_CTRL` writer"]
pub type W = crate::W<RegTbCtrlSpec>;
#[doc = "Field `BIT_TB_PKT_LIFE_TIME` reader - LIFETIME Threshold: Unit = 256us. When TB PPDU is triggered, If BIT_TB_EN_LIFETIME = 1, count down. If BIT_TB_EN_LIFETIME = 0, do nothing."]
pub type BitTbPktLifeTimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TB_PKT_LIFE_TIME` writer - LIFETIME Threshold: Unit = 256us. When TB PPDU is triggered, If BIT_TB_EN_LIFETIME = 1, count down. If BIT_TB_EN_LIFETIME = 0, do nothing."]
pub type BitTbPktLifeTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_TB_EN_LIFETIME` reader - When this bit is set, enable the lifetime function of TB PPDU."]
pub type BitTbEnLifetimeR = crate::BitReader;
#[doc = "Field `BIT_TB_EN_LIFETIME` writer - When this bit is set, enable the lifetime function of TB PPDU."]
pub type BitTbEnLifetimeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TB_DEL_EN` reader - Enable TB PPDU packet delete: 1) pkt_ageout; 2) macid_delete"]
pub type BitTbDelEnR = crate::BitReader;
#[doc = "Field `BIT_TB_DEL_EN` writer - Enable TB PPDU packet delete: 1) pkt_ageout; 2) macid_delete"]
pub type BitTbDelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TB_BYPASS_TXPOWER` reader - 1: NOT abort TB PPDU even the txpwr_abort_tx_idx(from txpwr_ctrl) is illegal 0: enable tx power abort TB"]
pub type BitTbBypassTxpowerR = crate::BitReader;
#[doc = "Field `BIT_TB_BYPASS_TXPOWER` writer - 1: NOT abort TB PPDU even the txpwr_abort_tx_idx(from txpwr_ctrl) is illegal 0: enable tx power abort TB"]
pub type BitTbBypassTxpowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HW_SSN_EN_TB` reader - HW update the SSN \\[9\\] for TB PPDU"]
pub type BitHwSsnEnTbR = crate::BitReader;
#[doc = "Field `BIT_HW_SSN_EN_TB` writer - HW update the SSN \\[9\\] for TB PPDU"]
pub type BitHwSsnEnTbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TB_SIFS_BK` reader - If sifs_timeout_tb_ref (from trxptcl) asserted, break the aggr process of TB PPDU"]
pub type BitEnTbSifsBkR = crate::BitReader;
#[doc = "Field `BIT_EN_TB_SIFS_BK` writer - If sifs_timeout_tb_ref (from trxptcl) asserted, break the aggr process of TB PPDU"]
pub type BitEnTbSifsBkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_UPD_TB_RETRY` reader - enable write retrybit in MH and add retrycnt when TB PPDU TX fail."]
pub type BitEnUpdTbRetryR = crate::BitReader;
#[doc = "Field `BIT_EN_UPD_TB_RETRY` writer - enable write retrybit in MH and add retrycnt when TB PPDU TX fail."]
pub type BitEnUpdTbRetryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TB_PRETX` reader - When this bit is set, enable pretx function of TB PPDU, assert txen before gen_cmd_rdy."]
pub type BitEnTbPretxR = crate::BitReader;
#[doc = "Field `BIT_EN_TB_PRETX` writer - When this bit is set, enable pretx function of TB PPDU, assert txen before gen_cmd_rdy."]
pub type BitEnTbPretxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_CHK_TB_AC` reader - When this bit is set, don't care the AC of DATA"]
pub type BitDisChkTbAcR = crate::BitReader;
#[doc = "Field `BIT_DIS_CHK_TB_AC` writer - When this bit is set, don't care the AC of DATA"]
pub type BitDisChkTbAcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TB_TXRPT` reader - When this bit is set, enable generate report for TB PPDU"]
pub type BitEnTbTxrptR = crate::BitReader;
#[doc = "Field `BIT_EN_TB_TXRPT` writer - When this bit is set, enable generate report for TB PPDU"]
pub type BitEnTbTxrptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:15 - LIFETIME Threshold: Unit = 256us. When TB PPDU is triggered, If BIT_TB_EN_LIFETIME = 1, count down. If BIT_TB_EN_LIFETIME = 0, do nothing."]
    #[inline(always)]
    pub fn bit_tb_pkt_life_time(&self) -> BitTbPktLifeTimeR {
        BitTbPktLifeTimeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - When this bit is set, enable the lifetime function of TB PPDU."]
    #[inline(always)]
    pub fn bit_tb_en_lifetime(&self) -> BitTbEnLifetimeR {
        BitTbEnLifetimeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable TB PPDU packet delete: 1) pkt_ageout; 2) macid_delete"]
    #[inline(always)]
    pub fn bit_tb_del_en(&self) -> BitTbDelEnR {
        BitTbDelEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: NOT abort TB PPDU even the txpwr_abort_tx_idx(from txpwr_ctrl) is illegal 0: enable tx power abort TB"]
    #[inline(always)]
    pub fn bit_tb_bypass_txpower(&self) -> BitTbBypassTxpowerR {
        BitTbBypassTxpowerR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HW update the SSN \\[9\\] for TB PPDU"]
    #[inline(always)]
    pub fn bit_hw_ssn_en_tb(&self) -> BitHwSsnEnTbR {
        BitHwSsnEnTbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If sifs_timeout_tb_ref (from trxptcl) asserted, break the aggr process of TB PPDU"]
    #[inline(always)]
    pub fn bit_en_tb_sifs_bk(&self) -> BitEnTbSifsBkR {
        BitEnTbSifsBkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - enable write retrybit in MH and add retrycnt when TB PPDU TX fail."]
    #[inline(always)]
    pub fn bit_en_upd_tb_retry(&self) -> BitEnUpdTbRetryR {
        BitEnUpdTbRetryR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When this bit is set, enable pretx function of TB PPDU, assert txen before gen_cmd_rdy."]
    #[inline(always)]
    pub fn bit_en_tb_pretx(&self) -> BitEnTbPretxR {
        BitEnTbPretxR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When this bit is set, don't care the AC of DATA"]
    #[inline(always)]
    pub fn bit_dis_chk_tb_ac(&self) -> BitDisChkTbAcR {
        BitDisChkTbAcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When this bit is set, enable generate report for TB PPDU"]
    #[inline(always)]
    pub fn bit_en_tb_txrpt(&self) -> BitEnTbTxrptR {
        BitEnTbTxrptR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - LIFETIME Threshold: Unit = 256us. When TB PPDU is triggered, If BIT_TB_EN_LIFETIME = 1, count down. If BIT_TB_EN_LIFETIME = 0, do nothing."]
    #[inline(always)]
    pub fn bit_tb_pkt_life_time(&mut self) -> BitTbPktLifeTimeW<'_, RegTbCtrlSpec> {
        BitTbPktLifeTimeW::new(self, 0)
    }
    #[doc = "Bit 16 - When this bit is set, enable the lifetime function of TB PPDU."]
    #[inline(always)]
    pub fn bit_tb_en_lifetime(&mut self) -> BitTbEnLifetimeW<'_, RegTbCtrlSpec> {
        BitTbEnLifetimeW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable TB PPDU packet delete: 1) pkt_ageout; 2) macid_delete"]
    #[inline(always)]
    pub fn bit_tb_del_en(&mut self) -> BitTbDelEnW<'_, RegTbCtrlSpec> {
        BitTbDelEnW::new(self, 17)
    }
    #[doc = "Bit 18 - 1: NOT abort TB PPDU even the txpwr_abort_tx_idx(from txpwr_ctrl) is illegal 0: enable tx power abort TB"]
    #[inline(always)]
    pub fn bit_tb_bypass_txpower(&mut self) -> BitTbBypassTxpowerW<'_, RegTbCtrlSpec> {
        BitTbBypassTxpowerW::new(self, 18)
    }
    #[doc = "Bit 19 - HW update the SSN \\[9\\] for TB PPDU"]
    #[inline(always)]
    pub fn bit_hw_ssn_en_tb(&mut self) -> BitHwSsnEnTbW<'_, RegTbCtrlSpec> {
        BitHwSsnEnTbW::new(self, 19)
    }
    #[doc = "Bit 20 - If sifs_timeout_tb_ref (from trxptcl) asserted, break the aggr process of TB PPDU"]
    #[inline(always)]
    pub fn bit_en_tb_sifs_bk(&mut self) -> BitEnTbSifsBkW<'_, RegTbCtrlSpec> {
        BitEnTbSifsBkW::new(self, 20)
    }
    #[doc = "Bit 21 - enable write retrybit in MH and add retrycnt when TB PPDU TX fail."]
    #[inline(always)]
    pub fn bit_en_upd_tb_retry(&mut self) -> BitEnUpdTbRetryW<'_, RegTbCtrlSpec> {
        BitEnUpdTbRetryW::new(self, 21)
    }
    #[doc = "Bit 22 - When this bit is set, enable pretx function of TB PPDU, assert txen before gen_cmd_rdy."]
    #[inline(always)]
    pub fn bit_en_tb_pretx(&mut self) -> BitEnTbPretxW<'_, RegTbCtrlSpec> {
        BitEnTbPretxW::new(self, 22)
    }
    #[doc = "Bit 23 - When this bit is set, don't care the AC of DATA"]
    #[inline(always)]
    pub fn bit_dis_chk_tb_ac(&mut self) -> BitDisChkTbAcW<'_, RegTbCtrlSpec> {
        BitDisChkTbAcW::new(self, 23)
    }
    #[doc = "Bit 24 - When this bit is set, enable generate report for TB PPDU"]
    #[inline(always)]
    pub fn bit_en_tb_txrpt(&mut self) -> BitEnTbTxrptW<'_, RegTbCtrlSpec> {
        BitEnTbTxrptW::new(self, 24)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegTbCtrlSpec> {
        DummyW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTbCtrlSpec;
impl crate::RegisterSpec for RegTbCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tb_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTbCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tb_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTbCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TB_CTRL to value 0x01f0_0000"]
impl crate::Resettable for RegTbCtrlSpec {
    const RESET_VALUE: u32 = 0x01f0_0000;
}
