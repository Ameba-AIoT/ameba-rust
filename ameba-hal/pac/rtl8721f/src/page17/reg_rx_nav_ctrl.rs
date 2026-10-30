#[doc = "Register `REG_RX_NAV_CTRL` reader"]
pub type R = crate::R<RegRxNavCtrlSpec>;
#[doc = "Register `REG_RX_NAV_CTRL` writer"]
pub type W = crate::W<RegRxNavCtrlSpec>;
#[doc = "Field `BIT_WMAC_BASICNAV_INTXOP` reader - enable updating basic nav in my txop"]
pub type BitWmacBasicnavIntxopR = crate::BitReader;
#[doc = "Field `BIT_WMAC_BASICNAV_INTXOP` writer - enable updating basic nav in my txop"]
pub type BitWmacBasicnavIntxopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_INTRANAV_INTXOP` reader - enable updating intra nav in my txop"]
pub type BitWmacIntranavIntxopR = crate::BitReader;
#[doc = "Field `BIT_WMAC_INTRANAV_INTXOP` writer - enable updating intra nav in my txop"]
pub type BitWmacIntranavIntxopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SRCTRL_PLCP_EN` reader - 1:use txop_dur as sr_tx_timer"]
pub type BitSrctrlPlcpEnR = crate::BitReader;
#[doc = "Field `BIT_SRCTRL_PLCP_EN` writer - 1:use txop_dur as sr_tx_timer"]
pub type BitSrctrlPlcpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TF_UP_NAV_EN` reader - 1:enable updating intra nav when receiving trigger frame"]
pub type BitWmacTfUpNavEnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TF_UP_NAV_EN` writer - 1:enable updating intra nav when receiving trigger frame"]
pub type BitWmacTfUpNavEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_PLCP_UP_NAV_EN` reader - 1:enable using txop_dur in plcp header to update nav"]
pub type BitWmacPlcpUpNavEnR = crate::BitReader;
#[doc = "Field `BIT_WMAC_PLCP_UP_NAV_EN` writer - 1:enable using txop_dur in plcp header to update nav"]
pub type BitWmacPlcpUpNavEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable updating basic nav in my txop"]
    #[inline(always)]
    pub fn bit_wmac_basicnav_intxop(&self) -> BitWmacBasicnavIntxopR {
        BitWmacBasicnavIntxopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable updating intra nav in my txop"]
    #[inline(always)]
    pub fn bit_wmac_intranav_intxop(&self) -> BitWmacIntranavIntxopR {
        BitWmacIntranavIntxopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:use txop_dur as sr_tx_timer"]
    #[inline(always)]
    pub fn bit_srctrl_plcp_en(&self) -> BitSrctrlPlcpEnR {
        BitSrctrlPlcpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1:enable updating intra nav when receiving trigger frame"]
    #[inline(always)]
    pub fn bit_wmac_tf_up_nav_en(&self) -> BitWmacTfUpNavEnR {
        BitWmacTfUpNavEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:enable using txop_dur in plcp header to update nav"]
    #[inline(always)]
    pub fn bit_wmac_plcp_up_nav_en(&self) -> BitWmacPlcpUpNavEnR {
        BitWmacPlcpUpNavEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable updating basic nav in my txop"]
    #[inline(always)]
    pub fn bit_wmac_basicnav_intxop(&mut self) -> BitWmacBasicnavIntxopW<'_, RegRxNavCtrlSpec> {
        BitWmacBasicnavIntxopW::new(self, 0)
    }
    #[doc = "Bit 1 - enable updating intra nav in my txop"]
    #[inline(always)]
    pub fn bit_wmac_intranav_intxop(&mut self) -> BitWmacIntranavIntxopW<'_, RegRxNavCtrlSpec> {
        BitWmacIntranavIntxopW::new(self, 1)
    }
    #[doc = "Bit 2 - 1:use txop_dur as sr_tx_timer"]
    #[inline(always)]
    pub fn bit_srctrl_plcp_en(&mut self) -> BitSrctrlPlcpEnW<'_, RegRxNavCtrlSpec> {
        BitSrctrlPlcpEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 1:enable updating intra nav when receiving trigger frame"]
    #[inline(always)]
    pub fn bit_wmac_tf_up_nav_en(&mut self) -> BitWmacTfUpNavEnW<'_, RegRxNavCtrlSpec> {
        BitWmacTfUpNavEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:enable using txop_dur in plcp header to update nav"]
    #[inline(always)]
    pub fn bit_wmac_plcp_up_nav_en(&mut self) -> BitWmacPlcpUpNavEnW<'_, RegRxNavCtrlSpec> {
        BitWmacPlcpUpNavEnW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_nav_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_nav_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxNavCtrlSpec;
impl crate::RegisterSpec for RegRxNavCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`reg_rx_nav_ctrl::R`](R) reader structure"]
impl crate::Readable for RegRxNavCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_nav_ctrl::W`](W) writer structure"]
impl crate::Writable for RegRxNavCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_NAV_CTRL to value 0x0b"]
impl crate::Resettable for RegRxNavCtrlSpec {
    const RESET_VALUE: u8 = 0x0b;
}
