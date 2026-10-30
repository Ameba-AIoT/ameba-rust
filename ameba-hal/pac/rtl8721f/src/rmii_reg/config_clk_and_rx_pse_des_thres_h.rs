#[doc = "Register `CONFIG_CLK_and_Rx_Pse_Des_Thres_h` reader"]
pub type R = crate::R<ConfigClkAndRxPseDesThresHSpec>;
#[doc = "Register `CONFIG_CLK_and_Rx_Pse_Des_Thres_h` writer"]
pub type W = crate::W<ConfigClkAndRxPseDesThresHSpec>;
#[doc = "Field `Rx_Pse_Des_Thres_off_1` reader - This is the Rx_Pse_Des_Thres_off1\\[11:8\\]."]
pub type RxPseDesThresOff1R = crate::FieldReader;
#[doc = "Field `Rx_Pse_Des_Thres_off_1` writer - This is the Rx_Pse_Des_Thres_off1\\[11:8\\]."]
pub type RxPseDesThresOff1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Rxc_dly_sel` reader - 32-tap delay chain for rxc. This field is valid when CONFIG_CLK_REG.dis_dly_mode is 0."]
pub type RxcDlySelR = crate::FieldReader;
#[doc = "Field `Rxc_dly_sel` writer - 32-tap delay chain for rxc. This field is valid when CONFIG_CLK_REG.dis_dly_mode is 0."]
pub type RxcDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Set_d_txc` reader - Add delay to gmii_txc_out. Used in clk. Rgmii_1000: add 2nS delay. Rgmii_100: add 4nS delay. Rgmii_10: add 4nS delay."]
pub type SetDTxcR = crate::BitReader;
#[doc = "Field `Set_d_txc` writer - Add delay to gmii_txc_out. Used in clk. Rgmii_1000: add 2nS delay. Rgmii_100: add 4nS delay. Rgmii_10: add 4nS delay."]
pub type SetDTxcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - This is the Rx_Pse_Des_Thres_off1\\[11:8\\]."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off_1(&self) -> RxPseDesThresOff1R {
        RxPseDesThresOff1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 32-tap delay chain for rxc. This field is valid when CONFIG_CLK_REG.dis_dly_mode is 0."]
    #[inline(always)]
    pub fn rxc_dly_sel(&self) -> RxcDlySelR {
        RxcDlySelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Add delay to gmii_txc_out. Used in clk. Rgmii_1000: add 2nS delay. Rgmii_100: add 4nS delay. Rgmii_10: add 4nS delay."]
    #[inline(always)]
    pub fn set_d_txc(&self) -> SetDTxcR {
        SetDTxcR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - This is the Rx_Pse_Des_Thres_off1\\[11:8\\]."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off_1(
        &mut self,
    ) -> RxPseDesThresOff1W<'_, ConfigClkAndRxPseDesThresHSpec> {
        RxPseDesThresOff1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 32-tap delay chain for rxc. This field is valid when CONFIG_CLK_REG.dis_dly_mode is 0."]
    #[inline(always)]
    pub fn rxc_dly_sel(&mut self) -> RxcDlySelW<'_, ConfigClkAndRxPseDesThresHSpec> {
        RxcDlySelW::new(self, 16)
    }
    #[doc = "Bit 21 - Add delay to gmii_txc_out. Used in clk. Rgmii_1000: add 2nS delay. Rgmii_100: add 4nS delay. Rgmii_10: add 4nS delay."]
    #[inline(always)]
    pub fn set_d_txc(&mut self) -> SetDTxcW<'_, ConfigClkAndRxPseDesThresHSpec> {
        SetDTxcW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`config_clk_and_rx_pse_des_thres_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config_clk_and_rx_pse_des_thres_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigClkAndRxPseDesThresHSpec;
impl crate::RegisterSpec for ConfigClkAndRxPseDesThresHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_clk_and_rx_pse_des_thres_h::R`](R) reader structure"]
impl crate::Readable for ConfigClkAndRxPseDesThresHSpec {}
#[doc = "`write(|w| ..)` method takes [`config_clk_and_rx_pse_des_thres_h::W`](W) writer structure"]
impl crate::Writable for ConfigClkAndRxPseDesThresHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG_CLK_and_Rx_Pse_Des_Thres_h to value 0"]
impl crate::Resettable for ConfigClkAndRxPseDesThresHSpec {}
