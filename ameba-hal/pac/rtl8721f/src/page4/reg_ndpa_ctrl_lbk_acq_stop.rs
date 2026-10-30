#[doc = "Register `REG_NDPA_CTRL_LBK_ACQ_STOP` reader"]
pub type R = crate::R<RegNdpaCtrlLbkAcqStopSpec>;
#[doc = "Register `REG_NDPA_CTRL_LBK_ACQ_STOP` writer"]
pub type W = crate::W<RegNdpaCtrlLbkAcqStopSpec>;
#[doc = "Field `BIT_AC0Q_STOP` reader - When this bit is raised, AC0 queue is stop. Report by MAC."]
pub type BitAc0qStopR = crate::BitReader;
#[doc = "Field `BIT_AC1Q_STOP` reader - When this bit is raised, AC1 queue is stop. Report by MAC."]
pub type BitAc1qStopR = crate::BitReader;
#[doc = "Field `BIT_AC2Q_STOP` reader - When this bit is raised, AC2 queue is stop. Report by MAC."]
pub type BitAc2qStopR = crate::BitReader;
#[doc = "Field `BIT_AC3Q_STOP` reader - When this bit is raised, AC3 queue is stop. Report by MAC."]
pub type BitAc3qStopR = crate::BitReader;
#[doc = "Field `BIT_AC4Q_STOP` reader - When this bit is raised, AC4 queue is stop. Report by MAC."]
pub type BitAc4qStopR = crate::BitReader;
#[doc = "Field `BIT_AC5Q_STOP` reader - When this bit is raised, AC5 queue is stop. Report by MAC."]
pub type BitAc5qStopR = crate::BitReader;
#[doc = "Field `BIT_AC6Q_STOP` reader - When this bit is raised, AC6 queue is stop. Report by MAC."]
pub type BitAc6qStopR = crate::BitReader;
#[doc = "Field `BIT_AC7Q_STOP` reader - When this bit is raised, AC7 queue is stop. Report by MAC."]
pub type BitAc7qStopR = crate::BitReader;
#[doc = "Field `BIT_WMAC_LBK_BUF_HEAD` reader - In delay MAC LBK mode, this register indicate the LBK BUF head page. When this buffer size is not enough for LBK packet length, LBK test may fail. In ARP/NS mode, this register indicate the ARP/NS response packet head page."]
pub type BitWmacLbkBufHeadR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_LBK_BUF_HEAD` writer - In delay MAC LBK mode, this register indicate the LBK BUF head page. When this buffer size is not enough for LBK packet length, LBK test may fail. In ARP/NS mode, this register indicate the ARP/NS response packet head page."]
pub type BitWmacLbkBufHeadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_Search_Queue_en` reader - Search queue request enable in ptcl idle:0 disable; 1 enable"]
pub type BitSearchQueueEnR = crate::BitReader;
#[doc = "Field `BIT_Search_Queue_en` writer - Search queue request enable in ptcl idle:0 disable; 1 enable"]
pub type BitSearchQueueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_OQT_BLOCK` reader - If this bit is set, the OQT will not block by all AC queues occupied by other MACID."]
pub type BitDisOqtBlockR = crate::BitReader;
#[doc = "Field `BIT_DIS_OQT_BLOCK` writer - If this bit is set, the OQT will not block by all AC queues occupied by other MACID."]
pub type BitDisOqtBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_RELEASE_RETRY` reader - when release one packet for a macid, retry should happen on the same ACQ. Set 1 to remove this requirement"]
pub type BitDisReleaseRetryR = crate::BitReader;
#[doc = "Field `BIT_DIS_RELEASE_RETRY` writer - when release one packet for a macid, retry should happen on the same ACQ. Set 1 to remove this requirement"]
pub type BitDisReleaseRetryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_en_gnt_bt_awake` reader - Enable pollute bit function.(C card ECO item, modify hardware circuit and add two options) 0: Disable 1: Enable When (GNT_BT option/LTE_RX option/(GNT_WL==0)) == 1, retry count can't increase if Tx fail. One or more packets will record in polluted bit field of txreport when txreport generates in Tx ok or tx drop."]
pub type BitREnGntBtAwakeR = crate::BitReader;
#[doc = "Field `BIT_r_en_gnt_bt_awake` writer - Enable pollute bit function.(C card ECO item, modify hardware circuit and add two options) 0: Disable 1: Enable When (GNT_BT option/LTE_RX option/(GNT_WL==0)) == 1, retry count can't increase if Tx fail. One or more packets will record in polluted bit field of txreport when txreport generates in Tx ok or tx drop."]
pub type BitREnGntBtAwakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_EOF_EN` reader - when set to 1, enable eof packet qos packet,but also need set agg_en to 0 in tx_desc"]
pub type BitREofEnR = crate::BitReader;
#[doc = "Field `BIT_r_EOF_EN` writer - when set to 1, enable eof packet qos packet,but also need set agg_en to 0 in tx_desc"]
pub type BitREofEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_SIGTA_BAR` reader - Enable BAR signaling TA in duplicate BW"]
pub type BitRSigtaBarR = crate::BitReader;
#[doc = "Field `BIT_r_SIGTA_BAR` writer - Enable BAR signaling TA in duplicate BW"]
pub type BitRSigtaBarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_SIGTA_BW` reader - Enable signaling TA in duplicate BW, valid if EN_SIGTA = 1 in TX DESC"]
pub type BitRSigtaBwR = crate::FieldReader;
#[doc = "Field `BIT_r_SIGTA_BW` writer - Enable signaling TA in duplicate BW, valid if EN_SIGTA = 1 in TX DESC"]
pub type BitRSigtaBwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_r_ndpa_bw` reader - NDPA bandwidth setting 3: 160M 2: 80M 1: 40M 0: 20M"]
pub type BitRNdpaBwR = crate::FieldReader;
#[doc = "Field `BIT_r_ndpa_bw` writer - NDPA bandwidth setting 3: 160M 2: 80M 1: 40M 0: 20M"]
pub type BitRNdpaBwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - When this bit is raised, AC0 queue is stop. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac0q_stop(&self) -> BitAc0qStopR {
        BitAc0qStopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is raised, AC1 queue is stop. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac1q_stop(&self) -> BitAc1qStopR {
        BitAc1qStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is raised, AC2 queue is stop. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac2q_stop(&self) -> BitAc2qStopR {
        BitAc2qStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is raised, AC3 queue is stop. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac3q_stop(&self) -> BitAc3qStopR {
        BitAc3qStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When this bit is raised, AC4 queue is stop. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac4q_stop(&self) -> BitAc4qStopR {
        BitAc4qStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When this bit is raised, AC5 queue is stop. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac5q_stop(&self) -> BitAc5qStopR {
        BitAc5qStopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When this bit is raised, AC6 queue is stop. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac6q_stop(&self) -> BitAc6qStopR {
        BitAc6qStopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit is raised, AC7 queue is stop. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac7q_stop(&self) -> BitAc7qStopR {
        BitAc7qStopR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - In delay MAC LBK mode, this register indicate the LBK BUF head page. When this buffer size is not enough for LBK packet length, LBK test may fail. In ARP/NS mode, this register indicate the ARP/NS response packet head page."]
    #[inline(always)]
    pub fn bit_wmac_lbk_buf_head(&self) -> BitWmacLbkBufHeadR {
        BitWmacLbkBufHeadR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Search queue request enable in ptcl idle:0 disable; 1 enable"]
    #[inline(always)]
    pub fn bit_search_queue_en(&self) -> BitSearchQueueEnR {
        BitSearchQueueEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If this bit is set, the OQT will not block by all AC queues occupied by other MACID."]
    #[inline(always)]
    pub fn bit_dis_oqt_block(&self) -> BitDisOqtBlockR {
        BitDisOqtBlockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - when release one packet for a macid, retry should happen on the same ACQ. Set 1 to remove this requirement"]
    #[inline(always)]
    pub fn bit_dis_release_retry(&self) -> BitDisReleaseRetryR {
        BitDisReleaseRetryR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable pollute bit function.(C card ECO item, modify hardware circuit and add two options) 0: Disable 1: Enable When (GNT_BT option/LTE_RX option/(GNT_WL==0)) == 1, retry count can't increase if Tx fail. One or more packets will record in polluted bit field of txreport when txreport generates in Tx ok or tx drop."]
    #[inline(always)]
    pub fn bit_r_en_gnt_bt_awake(&self) -> BitREnGntBtAwakeR {
        BitREnGntBtAwakeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - when set to 1, enable eof packet qos packet,but also need set agg_en to 0 in tx_desc"]
    #[inline(always)]
    pub fn bit_r_eof_en(&self) -> BitREofEnR {
        BitREofEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable BAR signaling TA in duplicate BW"]
    #[inline(always)]
    pub fn bit_r_sigta_bar(&self) -> BitRSigtaBarR {
        BitRSigtaBarR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Enable signaling TA in duplicate BW, valid if EN_SIGTA = 1 in TX DESC"]
    #[inline(always)]
    pub fn bit_r_sigta_bw(&self) -> BitRSigtaBwR {
        BitRSigtaBwR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - NDPA bandwidth setting 3: 160M 2: 80M 1: 40M 0: 20M"]
    #[inline(always)]
    pub fn bit_r_ndpa_bw(&self) -> BitRNdpaBwR {
        BitRNdpaBwR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - In delay MAC LBK mode, this register indicate the LBK BUF head page. When this buffer size is not enough for LBK packet length, LBK test may fail. In ARP/NS mode, this register indicate the ARP/NS response packet head page."]
    #[inline(always)]
    pub fn bit_wmac_lbk_buf_head(&mut self) -> BitWmacLbkBufHeadW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitWmacLbkBufHeadW::new(self, 8)
    }
    #[doc = "Bit 16 - Search queue request enable in ptcl idle:0 disable; 1 enable"]
    #[inline(always)]
    pub fn bit_search_queue_en(&mut self) -> BitSearchQueueEnW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitSearchQueueEnW::new(self, 16)
    }
    #[doc = "Bit 17 - If this bit is set, the OQT will not block by all AC queues occupied by other MACID."]
    #[inline(always)]
    pub fn bit_dis_oqt_block(&mut self) -> BitDisOqtBlockW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitDisOqtBlockW::new(self, 17)
    }
    #[doc = "Bit 18 - when release one packet for a macid, retry should happen on the same ACQ. Set 1 to remove this requirement"]
    #[inline(always)]
    pub fn bit_dis_release_retry(&mut self) -> BitDisReleaseRetryW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitDisReleaseRetryW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable pollute bit function.(C card ECO item, modify hardware circuit and add two options) 0: Disable 1: Enable When (GNT_BT option/LTE_RX option/(GNT_WL==0)) == 1, retry count can't increase if Tx fail. One or more packets will record in polluted bit field of txreport when txreport generates in Tx ok or tx drop."]
    #[inline(always)]
    pub fn bit_r_en_gnt_bt_awake(&mut self) -> BitREnGntBtAwakeW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitREnGntBtAwakeW::new(self, 19)
    }
    #[doc = "Bit 20 - when set to 1, enable eof packet qos packet,but also need set agg_en to 0 in tx_desc"]
    #[inline(always)]
    pub fn bit_r_eof_en(&mut self) -> BitREofEnW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitREofEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable BAR signaling TA in duplicate BW"]
    #[inline(always)]
    pub fn bit_r_sigta_bar(&mut self) -> BitRSigtaBarW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitRSigtaBarW::new(self, 21)
    }
    #[doc = "Bits 22:23 - Enable signaling TA in duplicate BW, valid if EN_SIGTA = 1 in TX DESC"]
    #[inline(always)]
    pub fn bit_r_sigta_bw(&mut self) -> BitRSigtaBwW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitRSigtaBwW::new(self, 22)
    }
    #[doc = "Bits 24:25 - NDPA bandwidth setting 3: 160M 2: 80M 1: 40M 0: 20M"]
    #[inline(always)]
    pub fn bit_r_ndpa_bw(&mut self) -> BitRNdpaBwW<'_, RegNdpaCtrlLbkAcqStopSpec> {
        BitRNdpaBwW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ndpa_ctrl_lbk_acq_stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ndpa_ctrl_lbk_acq_stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNdpaCtrlLbkAcqStopSpec;
impl crate::RegisterSpec for RegNdpaCtrlLbkAcqStopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ndpa_ctrl_lbk_acq_stop::R`](R) reader structure"]
impl crate::Readable for RegNdpaCtrlLbkAcqStopSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ndpa_ctrl_lbk_acq_stop::W`](W) writer structure"]
impl crate::Writable for RegNdpaCtrlLbkAcqStopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NDPA_CTRL_LBK_ACQ_STOP to value 0x0008_f600"]
impl crate::Resettable for RegNdpaCtrlLbkAcqStopSpec {
    const RESET_VALUE: u32 = 0x0008_f600;
}
