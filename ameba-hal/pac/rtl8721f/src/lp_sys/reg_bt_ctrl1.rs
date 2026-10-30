#[doc = "Register `REG_BT_CTRL1` reader"]
pub type R = crate::R<RegBtCtrl1Spec>;
#[doc = "Register `REG_BT_CTRL1` writer"]
pub type W = crate::W<RegBtCtrl1Spec>;
#[doc = "Field `BT_PMC_STATE` reader - indicate bt_misc PMC current state, for debug"]
pub type BtPmcStateR = crate::FieldReader<u16>;
#[doc = "Field `BT_CKSL_CKANA` reader - 1: enable osc4m when BT enter LPS 0: switch bt_ana_clk from osc4m to ana_100k when BT enter LPS"]
pub type BtCkslCkanaR = crate::BitReader;
#[doc = "Field `BT_CKSL_CKANA` writer - 1: enable osc4m when BT enter LPS 0: switch bt_ana_clk from osc4m to ana_100k when BT enter LPS"]
pub type BtCkslCkanaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - indicate bt_misc PMC current state, for debug"]
    #[inline(always)]
    pub fn bt_pmc_state(&self) -> BtPmcStateR {
        BtPmcStateR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - 1: enable osc4m when BT enter LPS 0: switch bt_ana_clk from osc4m to ana_100k when BT enter LPS"]
    #[inline(always)]
    pub fn bt_cksl_ckana(&self) -> BtCkslCkanaR {
        BtCkslCkanaR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - 1: enable osc4m when BT enter LPS 0: switch bt_ana_clk from osc4m to ana_100k when BT enter LPS"]
    #[inline(always)]
    pub fn bt_cksl_ckana(&mut self) -> BtCkslCkanaW<'_, RegBtCtrl1Spec> {
        BtCkslCkanaW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtCtrl1Spec;
impl crate::RegisterSpec for RegBtCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegBtCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegBtCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_CTRL1 to value 0x8000"]
impl crate::Resettable for RegBtCtrl1Spec {
    const RESET_VALUE: u32 = 0x8000;
}
