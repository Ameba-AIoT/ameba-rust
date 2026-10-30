#[doc = "Register `SP_DIRECT_CTRL4` reader"]
pub type R = crate::R<SpDirectCtrl4Spec>;
#[doc = "Register `SP_DIRECT_CTRL4` writer"]
pub type W = crate::W<SpDirectCtrl4Spec>;
#[doc = "Field `rx_fifo_1_reg_0_l_sel` reader - (Ibid.)"]
pub type RxFifo1Reg0LSelR = crate::FieldReader;
#[doc = "Field `rx_fifo_1_reg_0_l_sel` writer - (Ibid.)"]
pub type RxFifo1Reg0LSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUMMY15` reader - "]
pub type Dummy15R = crate::FieldReader;
#[doc = "Field `DUMMY15` writer - "]
pub type Dummy15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rx_fifo_1_reg_0_r_sel` reader - (Ibid.)"]
pub type RxFifo1Reg0RSelR = crate::FieldReader;
#[doc = "Field `rx_fifo_1_reg_0_r_sel` writer - (Ibid.)"]
pub type RxFifo1Reg0RSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUMMY14` reader - "]
pub type Dummy14R = crate::FieldReader;
#[doc = "Field `DUMMY14` writer - "]
pub type Dummy14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rx_fifo_1_reg_1_l_sel` reader - (Ibid.)"]
pub type RxFifo1Reg1LSelR = crate::FieldReader;
#[doc = "Field `rx_fifo_1_reg_1_l_sel` writer - (Ibid.)"]
pub type RxFifo1Reg1LSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUMMY13` reader - "]
pub type Dummy13R = crate::FieldReader;
#[doc = "Field `DUMMY13` writer - "]
pub type Dummy13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rx_fifo_1_reg_1_r_sel` reader - 5'd0: RX_CH0_data_out (MIC path) 5'd1: RX_CH1_data_out (MIC path) 5'd2: RX_CH2_data_out (MIC path) 5'd3: RX_CH3_data_out (MIC path) 5'd4: RX_CH4_data_out (MIC path) 5'd5: RX_CH5_data_out (MIC path) 5'd6: RX_CH6_data_out (MIC path) 5'd7: RX_CH7_data_out (MIC path) 5'd8: spa_direct_in_0 5'd9: spa_direct_in_1 5'd10: spa_direct_in_2 5'd11: spa_direct_in_3 5'd12: spa_direct_in_4 5'd13: spa_direct_in_5 5'd14: spa_direct_in_6 5'd15: spa_direct_in_7 5'd16: spb_direct_in_0 5'd17: spb_direct_in_1 5'd18: spb_direct_in_2 5'd19: spb_direct_in_3 5'd20: spb_direct_in_4 5'd21: spb_direct_in_5 5'd22: spb_direct_in_6 5'd23: spb_direct_in_7 5'd24: spc_direct_in_0 5'd25: spc_direct_in_1 5'd26: spc_direct_in_2 5'd27: spc_direct_in_3 5'd28: spc_direct_in_4 5'd29: spc_direct_in_5 5'd30: spc_direct_in_6 5'd31: spc_direct_in_7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
pub type RxFifo1Reg1RSelR = crate::FieldReader;
#[doc = "Field `rx_fifo_1_reg_1_r_sel` writer - 5'd0: RX_CH0_data_out (MIC path) 5'd1: RX_CH1_data_out (MIC path) 5'd2: RX_CH2_data_out (MIC path) 5'd3: RX_CH3_data_out (MIC path) 5'd4: RX_CH4_data_out (MIC path) 5'd5: RX_CH5_data_out (MIC path) 5'd6: RX_CH6_data_out (MIC path) 5'd7: RX_CH7_data_out (MIC path) 5'd8: spa_direct_in_0 5'd9: spa_direct_in_1 5'd10: spa_direct_in_2 5'd11: spa_direct_in_3 5'd12: spa_direct_in_4 5'd13: spa_direct_in_5 5'd14: spa_direct_in_6 5'd15: spa_direct_in_7 5'd16: spb_direct_in_0 5'd17: spb_direct_in_1 5'd18: spb_direct_in_2 5'd19: spb_direct_in_3 5'd20: spb_direct_in_4 5'd21: spb_direct_in_5 5'd22: spb_direct_in_6 5'd23: spb_direct_in_7 5'd24: spc_direct_in_0 5'd25: spc_direct_in_1 5'd26: spc_direct_in_2 5'd27: spc_direct_in_3 5'd28: spc_direct_in_4 5'd29: spc_direct_in_5 5'd30: spc_direct_in_6 5'd31: spc_direct_in_7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
pub type RxFifo1Reg1RSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DUMMY12` reader - "]
pub type Dummy12R = crate::FieldReader;
#[doc = "Field `DUMMY12` writer - "]
pub type Dummy12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_1_reg_0_l_sel(&self) -> RxFifo1Reg0LSelR {
        RxFifo1Reg0LSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn dummy15(&self) -> Dummy15R {
        Dummy15R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_1_reg_0_r_sel(&self) -> RxFifo1Reg0RSelR {
        RxFifo1Reg0RSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn dummy14(&self) -> Dummy14R {
        Dummy14R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_1_reg_1_l_sel(&self) -> RxFifo1Reg1LSelR {
        RxFifo1Reg1LSelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dummy13(&self) -> Dummy13R {
        Dummy13R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 5'd0: RX_CH0_data_out (MIC path) 5'd1: RX_CH1_data_out (MIC path) 5'd2: RX_CH2_data_out (MIC path) 5'd3: RX_CH3_data_out (MIC path) 5'd4: RX_CH4_data_out (MIC path) 5'd5: RX_CH5_data_out (MIC path) 5'd6: RX_CH6_data_out (MIC path) 5'd7: RX_CH7_data_out (MIC path) 5'd8: spa_direct_in_0 5'd9: spa_direct_in_1 5'd10: spa_direct_in_2 5'd11: spa_direct_in_3 5'd12: spa_direct_in_4 5'd13: spa_direct_in_5 5'd14: spa_direct_in_6 5'd15: spa_direct_in_7 5'd16: spb_direct_in_0 5'd17: spb_direct_in_1 5'd18: spb_direct_in_2 5'd19: spb_direct_in_3 5'd20: spb_direct_in_4 5'd21: spb_direct_in_5 5'd22: spb_direct_in_6 5'd23: spb_direct_in_7 5'd24: spc_direct_in_0 5'd25: spc_direct_in_1 5'd26: spc_direct_in_2 5'd27: spc_direct_in_3 5'd28: spc_direct_in_4 5'd29: spc_direct_in_5 5'd30: spc_direct_in_6 5'd31: spc_direct_in_7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
    #[inline(always)]
    pub fn rx_fifo_1_reg_1_r_sel(&self) -> RxFifo1Reg1RSelR {
        RxFifo1Reg1RSelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn dummy12(&self) -> Dummy12R {
        Dummy12R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_1_reg_0_l_sel(&mut self) -> RxFifo1Reg0LSelW<'_, SpDirectCtrl4Spec> {
        RxFifo1Reg0LSelW::new(self, 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn dummy15(&mut self) -> Dummy15W<'_, SpDirectCtrl4Spec> {
        Dummy15W::new(self, 5)
    }
    #[doc = "Bits 8:12 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_1_reg_0_r_sel(&mut self) -> RxFifo1Reg0RSelW<'_, SpDirectCtrl4Spec> {
        RxFifo1Reg0RSelW::new(self, 8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn dummy14(&mut self) -> Dummy14W<'_, SpDirectCtrl4Spec> {
        Dummy14W::new(self, 13)
    }
    #[doc = "Bits 16:20 - (Ibid.)"]
    #[inline(always)]
    pub fn rx_fifo_1_reg_1_l_sel(&mut self) -> RxFifo1Reg1LSelW<'_, SpDirectCtrl4Spec> {
        RxFifo1Reg1LSelW::new(self, 16)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dummy13(&mut self) -> Dummy13W<'_, SpDirectCtrl4Spec> {
        Dummy13W::new(self, 21)
    }
    #[doc = "Bits 24:28 - 5'd0: RX_CH0_data_out (MIC path) 5'd1: RX_CH1_data_out (MIC path) 5'd2: RX_CH2_data_out (MIC path) 5'd3: RX_CH3_data_out (MIC path) 5'd4: RX_CH4_data_out (MIC path) 5'd5: RX_CH5_data_out (MIC path) 5'd6: RX_CH6_data_out (MIC path) 5'd7: RX_CH7_data_out (MIC path) 5'd8: spa_direct_in_0 5'd9: spa_direct_in_1 5'd10: spa_direct_in_2 5'd11: spa_direct_in_3 5'd12: spa_direct_in_4 5'd13: spa_direct_in_5 5'd14: spa_direct_in_6 5'd15: spa_direct_in_7 5'd16: spb_direct_in_0 5'd17: spb_direct_in_1 5'd18: spb_direct_in_2 5'd19: spb_direct_in_3 5'd20: spb_direct_in_4 5'd21: spb_direct_in_5 5'd22: spb_direct_in_6 5'd23: spb_direct_in_7 5'd24: spc_direct_in_0 5'd25: spc_direct_in_1 5'd26: spc_direct_in_2 5'd27: spc_direct_in_3 5'd28: spc_direct_in_4 5'd29: spc_direct_in_5 5'd30: spc_direct_in_6 5'd31: spc_direct_in_7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
    #[inline(always)]
    pub fn rx_fifo_1_reg_1_r_sel(&mut self) -> RxFifo1Reg1RSelW<'_, SpDirectCtrl4Spec> {
        RxFifo1Reg1RSelW::new(self, 24)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn dummy12(&mut self) -> Dummy12W<'_, SpDirectCtrl4Spec> {
        Dummy12W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_direct_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_direct_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpDirectCtrl4Spec;
impl crate::RegisterSpec for SpDirectCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_direct_ctrl4::R`](R) reader structure"]
impl crate::Readable for SpDirectCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_direct_ctrl4::W`](W) writer structure"]
impl crate::Writable for SpDirectCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_DIRECT_CTRL4 to value 0x0706_0504"]
impl crate::Resettable for SpDirectCtrl4Spec {
    const RESET_VALUE: u32 = 0x0706_0504;
}
