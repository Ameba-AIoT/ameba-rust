#[doc = "Register `SP_DIRECT_CTRL3` reader"]
pub type R = crate::R<SpDirectCtrl3Spec>;
#[doc = "Register `SP_DIRECT_CTRL3` writer"]
pub type W = crate::W<SpDirectCtrl3Spec>;
#[doc = "Field `rx_fifo_0_reg_0_l_sel` reader - (Ibid.)"]
pub type RxFifo0Reg0LSelR = crate::FieldReader;
#[doc = "Field `rx_fifo_0_reg_0_l_sel` writer - (Ibid.)"]
pub type RxFifo0Reg0LSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUMMY11` reader - "]
pub type Dummy11R = crate::FieldReader;
#[doc = "Field `DUMMY11` writer - "]
pub type Dummy11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rx_fifo_0_reg_0_r_sel` reader - (Ibid.)"]
pub type RxFifo0Reg0RSelR = crate::FieldReader;
#[doc = "Field `rx_fifo_0_reg_0_r_sel` writer - (Ibid.)"]
pub type RxFifo0Reg0RSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUMMY10` reader - "]
pub type Dummy10R = crate::FieldReader;
#[doc = "Field `DUMMY10` writer - "]
pub type Dummy10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rx_fifo_0_reg_1_l_sel` reader - (Ibid.)"]
pub type RxFifo0Reg1LSelR = crate::FieldReader;
#[doc = "Field `rx_fifo_0_reg_1_l_sel` writer - (Ibid.)"]
pub type RxFifo0Reg1LSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUMMY9` reader - "]
pub type Dummy9R = crate::FieldReader;
#[doc = "Field `DUMMY9` writer - "]
pub type Dummy9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rx_fifo_0_reg_1_r_sel` reader - 5'd0: RX_CH0_data_out (MIC path) 5'd1: RX_CH1_data_out (MIC path) 5'd2: RX_CH2_data_out (MIC path) 5'd3: RX_CH3_data_out (MIC path) 5'd4: RX_CH4_data_out (MIC path) 5'd5: RX_CH5_data_out (MIC path) 5'd6: RX_CH6_data_out (MIC path) 5'd7: RX_CH7_data_out (MIC path) 5'd8: spa_direct_in_0 5'd9: spa_direct_in_1 5'd10: spa_direct_in_2 5'd11: spa_direct_in_3 5'd12: spa_direct_in_4 5'd13: spa_direct_in_5 5'd14: spa_direct_in_6 5'd15: spa_direct_in_7 5'd16: spb_direct_in_0 5'd17: spb_direct_in_1 5'd18: spb_direct_in_2 5'd19: spb_direct_in_3 5'd20: spb_direct_in_4 5'd21: spb_direct_in_5 5'd22: spb_direct_in_6 5'd23: spb_direct_in_7 5'd24: spc_direct_in_0 5'd25: spc_direct_in_1 5'd26: spc_direct_in_2 5'd27: spc_direct_in_3 5'd28: spc_direct_in_4 5'd29: spc_direct_in_5 5'd30: spc_direct_in_6 5'd31: spc_direct_in_7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
pub type RxFifo0Reg1RSelR = crate::FieldReader;
#[doc = "Field `rx_fifo_0_reg_1_r_sel` writer - 5'd0: RX_CH0_data_out (MIC path) 5'd1: RX_CH1_data_out (MIC path) 5'd2: RX_CH2_data_out (MIC path) 5'd3: RX_CH3_data_out (MIC path) 5'd4: RX_CH4_data_out (MIC path) 5'd5: RX_CH5_data_out (MIC path) 5'd6: RX_CH6_data_out (MIC path) 5'd7: RX_CH7_data_out (MIC path) 5'd8: spa_direct_in_0 5'd9: spa_direct_in_1 5'd10: spa_direct_in_2 5'd11: spa_direct_in_3 5'd12: spa_direct_in_4 5'd13: spa_direct_in_5 5'd14: spa_direct_in_6 5'd15: spa_direct_in_7 5'd16: spb_direct_in_0 5'd17: spb_direct_in_1 5'd18: spb_direct_in_2 5'd19: spb_direct_in_3 5'd20: spb_direct_in_4 5'd21: spb_direct_in_5 5'd22: spb_direct_in_6 5'd23: spb_direct_in_7 5'd24: spc_direct_in_0 5'd25: spc_direct_in_1 5'd26: spc_direct_in_2 5'd27: spc_direct_in_3 5'd28: spc_direct_in_4 5'd29: spc_direct_in_5 5'd30: spc_direct_in_6 5'd31: spc_direct_in_7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
pub type RxFifo0Reg1RSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUMMY8` reader - -"]
pub type Dummy8R = crate::FieldReader;
#[doc = "Field `DUMMY8` writer - -"]
pub type Dummy8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_0_reg_0_l_sel(&self) -> RxFifo0Reg0LSelR {
        RxFifo0Reg0LSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn dummy11(&self) -> Dummy11R {
        Dummy11R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_0_reg_0_r_sel(&self) -> RxFifo0Reg0RSelR {
        RxFifo0Reg0RSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn dummy10(&self) -> Dummy10R {
        Dummy10R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_0_reg_1_l_sel(&self) -> RxFifo0Reg1LSelR {
        RxFifo0Reg1LSelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dummy9(&self) -> Dummy9R {
        Dummy9R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 5'd0: RX_CH0_data_out (MIC path) 5'd1: RX_CH1_data_out (MIC path) 5'd2: RX_CH2_data_out (MIC path) 5'd3: RX_CH3_data_out (MIC path) 5'd4: RX_CH4_data_out (MIC path) 5'd5: RX_CH5_data_out (MIC path) 5'd6: RX_CH6_data_out (MIC path) 5'd7: RX_CH7_data_out (MIC path) 5'd8: spa_direct_in_0 5'd9: spa_direct_in_1 5'd10: spa_direct_in_2 5'd11: spa_direct_in_3 5'd12: spa_direct_in_4 5'd13: spa_direct_in_5 5'd14: spa_direct_in_6 5'd15: spa_direct_in_7 5'd16: spb_direct_in_0 5'd17: spb_direct_in_1 5'd18: spb_direct_in_2 5'd19: spb_direct_in_3 5'd20: spb_direct_in_4 5'd21: spb_direct_in_5 5'd22: spb_direct_in_6 5'd23: spb_direct_in_7 5'd24: spc_direct_in_0 5'd25: spc_direct_in_1 5'd26: spc_direct_in_2 5'd27: spc_direct_in_3 5'd28: spc_direct_in_4 5'd29: spc_direct_in_5 5'd30: spc_direct_in_6 5'd31: spc_direct_in_7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
    #[inline(always)]
    pub fn rx_fifo_0_reg_1_r_sel(&self) -> RxFifo0Reg1RSelR {
        RxFifo0Reg1RSelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - -"]
    #[inline(always)]
    pub fn dummy8(&self) -> Dummy8R {
        Dummy8R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_0_reg_0_l_sel(&mut self) -> RxFifo0Reg0LSelW<'_, SpDirectCtrl3Spec> {
        RxFifo0Reg0LSelW::new(self, 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn dummy11(&mut self) -> Dummy11W<'_, SpDirectCtrl3Spec> {
        Dummy11W::new(self, 5)
    }
    #[doc = "Bits 8:12 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_0_reg_0_r_sel(&mut self) -> RxFifo0Reg0RSelW<'_, SpDirectCtrl3Spec> {
        RxFifo0Reg0RSelW::new(self, 8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn dummy10(&mut self) -> Dummy10W<'_, SpDirectCtrl3Spec> {
        Dummy10W::new(self, 13)
    }
    #[doc = "Bits 16:20 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_0_reg_1_l_sel(&mut self) -> RxFifo0Reg1LSelW<'_, SpDirectCtrl3Spec> {
        RxFifo0Reg1LSelW::new(self, 16)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dummy9(&mut self) -> Dummy9W<'_, SpDirectCtrl3Spec> {
        Dummy9W::new(self, 21)
    }
    #[doc = "Bits 24:28 - 5'd0: RX_CH0_data_out (MIC path) 5'd1: RX_CH1_data_out (MIC path) 5'd2: RX_CH2_data_out (MIC path) 5'd3: RX_CH3_data_out (MIC path) 5'd4: RX_CH4_data_out (MIC path) 5'd5: RX_CH5_data_out (MIC path) 5'd6: RX_CH6_data_out (MIC path) 5'd7: RX_CH7_data_out (MIC path) 5'd8: spa_direct_in_0 5'd9: spa_direct_in_1 5'd10: spa_direct_in_2 5'd11: spa_direct_in_3 5'd12: spa_direct_in_4 5'd13: spa_direct_in_5 5'd14: spa_direct_in_6 5'd15: spa_direct_in_7 5'd16: spb_direct_in_0 5'd17: spb_direct_in_1 5'd18: spb_direct_in_2 5'd19: spb_direct_in_3 5'd20: spb_direct_in_4 5'd21: spb_direct_in_5 5'd22: spb_direct_in_6 5'd23: spb_direct_in_7 5'd24: spc_direct_in_0 5'd25: spc_direct_in_1 5'd26: spc_direct_in_2 5'd27: spc_direct_in_3 5'd28: spc_direct_in_4 5'd29: spc_direct_in_5 5'd30: spc_direct_in_6 5'd31: spc_direct_in_7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
    #[inline(always)]
    pub fn rx_fifo_0_reg_1_r_sel(&mut self) -> RxFifo0Reg1RSelW<'_, SpDirectCtrl3Spec> {
        RxFifo0Reg1RSelW::new(self, 24)
    }
    #[doc = "Bits 29:31 - -"]
    #[inline(always)]
    pub fn dummy8(&mut self) -> Dummy8W<'_, SpDirectCtrl3Spec> {
        Dummy8W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_direct_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_direct_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpDirectCtrl3Spec;
impl crate::RegisterSpec for SpDirectCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_direct_ctrl3::R`](R) reader structure"]
impl crate::Readable for SpDirectCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_direct_ctrl3::W`](W) writer structure"]
impl crate::Writable for SpDirectCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_DIRECT_CTRL3 to value 0x0302_0100"]
impl crate::Resettable for SpDirectCtrl3Spec {
    const RESET_VALUE: u32 = 0x0302_0100;
}
