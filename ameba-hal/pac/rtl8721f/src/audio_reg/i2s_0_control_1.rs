#[doc = "Register `i2s_0_control_1` reader"]
pub type R = crate::R<I2s0Control1Spec>;
#[doc = "Register `i2s_0_control_1` writer"]
pub type W = crate::W<I2s0Control1Spec>;
#[doc = "Field `i2s_0_data_ch0_sel_rx` reader - I2S 0 Rx channel data channel 0 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh0SelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch0_sel_rx` writer - I2S 0 Rx channel data channel 0 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh0SelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2s_0_data_ch1_sel_rx` reader - I2S 0 Rx channel data channel 1 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh1SelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch1_sel_rx` writer - I2S 0 Rx channel data channel 1 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh1SelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2s_0_data_ch2_sel_rx` reader - I2S 0 Rx channel data channel 2 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh2SelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch2_sel_rx` writer - I2S 0 Rx channel data channel 2 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh2SelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2s_0_data_ch3_sel_rx` reader - I2S 0 Rx channel data channel 3 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh3SelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch3_sel_rx` writer - I2S 0 Rx channel data channel 3 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh3SelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2s_0_data_ch4_sel_rx` reader - I2S 0 Rx channel data channel 4 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh4SelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch4_sel_rx` writer - I2S 0 Rx channel data channel 4 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh4SelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2s_0_data_ch5_sel_rx` reader - I2S 0 Rx channel data channel 5 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh5SelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch5_sel_rx` writer - I2S 0 Rx channel data channel 5 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh5SelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2s_0_data_ch6_sel_rx` reader - I2S 0 Rx channel data channel 6 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh6SelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch6_sel_rx` writer - I2S 0 Rx channel data channel 6 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh6SelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2s_0_data_ch7_sel_rx` reader - I2S 0 Rx channel data channel 7 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh7SelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch7_sel_rx` writer - I2S 0 Rx channel data channel 7 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
pub type I2s0DataCh7SelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `i2s_0_data_ch0_rx_disable` reader - I2S 0 Rx channel data channel 0 1: disable 0: enable"]
pub type I2s0DataCh0RxDisableR = crate::BitReader;
#[doc = "Field `i2s_0_data_ch0_rx_disable` writer - I2S 0 Rx channel data channel 0 1: disable 0: enable"]
pub type I2s0DataCh0RxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_data_ch1_rx_disable` reader - I2S 0 Rx channel data channel 1 1: disable 0: enable"]
pub type I2s0DataCh1RxDisableR = crate::BitReader;
#[doc = "Field `i2s_0_data_ch1_rx_disable` writer - I2S 0 Rx channel data channel 1 1: disable 0: enable"]
pub type I2s0DataCh1RxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_data_ch2_rx_disable` reader - I2S 0 Rx channel data channel 2 1: disable 0: enable"]
pub type I2s0DataCh2RxDisableR = crate::BitReader;
#[doc = "Field `i2s_0_data_ch2_rx_disable` writer - I2S 0 Rx channel data channel 2 1: disable 0: enable"]
pub type I2s0DataCh2RxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_data_ch3_rx_disable` reader - I2S 0 Rx channel data channel 3 1: disable 0: enable"]
pub type I2s0DataCh3RxDisableR = crate::BitReader;
#[doc = "Field `i2s_0_data_ch3_rx_disable` writer - I2S 0 Rx channel data channel 3 1: disable 0: enable"]
pub type I2s0DataCh3RxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_data_ch4_rx_disable` reader - I2S 0 Rx channel data channel 4 1: disable 0: enable"]
pub type I2s0DataCh4RxDisableR = crate::BitReader;
#[doc = "Field `i2s_0_data_ch4_rx_disable` writer - I2S 0 Rx channel data channel 4 1: disable 0: enable"]
pub type I2s0DataCh4RxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_data_ch5_rx_disable` reader - I2S 0 Rx channel data channel 5 1: disable 0: enable"]
pub type I2s0DataCh5RxDisableR = crate::BitReader;
#[doc = "Field `i2s_0_data_ch5_rx_disable` writer - I2S 0 Rx channel data channel 5 1: disable 0: enable"]
pub type I2s0DataCh5RxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_data_ch6_rx_disable` reader - I2S 0 Rx channel data channel 6 1: disable 0: enable"]
pub type I2s0DataCh6RxDisableR = crate::BitReader;
#[doc = "Field `i2s_0_data_ch6_rx_disable` writer - I2S 0 Rx channel data channel 6 1: disable 0: enable"]
pub type I2s0DataCh6RxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_data_ch7_rx_disable` reader - I2S 0 Rx channel data channel 7 1: disable 0: enable"]
pub type I2s0DataCh7RxDisableR = crate::BitReader;
#[doc = "Field `i2s_0_data_ch7_rx_disable` writer - I2S 0 Rx channel data channel 7 1: disable 0: enable"]
pub type I2s0DataCh7RxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - I2S 0 Rx channel data channel 0 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch0_sel_rx(&self) -> I2s0DataCh0SelRxR {
        I2s0DataCh0SelRxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - I2S 0 Rx channel data channel 1 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch1_sel_rx(&self) -> I2s0DataCh1SelRxR {
        I2s0DataCh1SelRxR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - I2S 0 Rx channel data channel 2 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch2_sel_rx(&self) -> I2s0DataCh2SelRxR {
        I2s0DataCh2SelRxR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - I2S 0 Rx channel data channel 3 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch3_sel_rx(&self) -> I2s0DataCh3SelRxR {
        I2s0DataCh3SelRxR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - I2S 0 Rx channel data channel 4 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch4_sel_rx(&self) -> I2s0DataCh4SelRxR {
        I2s0DataCh4SelRxR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - I2S 0 Rx channel data channel 5 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch5_sel_rx(&self) -> I2s0DataCh5SelRxR {
        I2s0DataCh5SelRxR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - I2S 0 Rx channel data channel 6 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch6_sel_rx(&self) -> I2s0DataCh6SelRxR {
        I2s0DataCh6SelRxR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - I2S 0 Rx channel data channel 7 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch7_sel_rx(&self) -> I2s0DataCh7SelRxR {
        I2s0DataCh7SelRxR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - I2S 0 Rx channel data channel 0 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch0_rx_disable(&self) -> I2s0DataCh0RxDisableR {
        I2s0DataCh0RxDisableR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2S 0 Rx channel data channel 1 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch1_rx_disable(&self) -> I2s0DataCh1RxDisableR {
        I2s0DataCh1RxDisableR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - I2S 0 Rx channel data channel 2 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch2_rx_disable(&self) -> I2s0DataCh2RxDisableR {
        I2s0DataCh2RxDisableR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - I2S 0 Rx channel data channel 3 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch3_rx_disable(&self) -> I2s0DataCh3RxDisableR {
        I2s0DataCh3RxDisableR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - I2S 0 Rx channel data channel 4 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch4_rx_disable(&self) -> I2s0DataCh4RxDisableR {
        I2s0DataCh4RxDisableR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - I2S 0 Rx channel data channel 5 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch5_rx_disable(&self) -> I2s0DataCh5RxDisableR {
        I2s0DataCh5RxDisableR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - I2S 0 Rx channel data channel 6 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch6_rx_disable(&self) -> I2s0DataCh6RxDisableR {
        I2s0DataCh6RxDisableR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2S 0 Rx channel data channel 7 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch7_rx_disable(&self) -> I2s0DataCh7RxDisableR {
        I2s0DataCh7RxDisableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - I2S 0 Rx channel data channel 0 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch0_sel_rx(&mut self) -> I2s0DataCh0SelRxW<'_, I2s0Control1Spec> {
        I2s0DataCh0SelRxW::new(self, 0)
    }
    #[doc = "Bits 3:5 - I2S 0 Rx channel data channel 1 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch1_sel_rx(&mut self) -> I2s0DataCh1SelRxW<'_, I2s0Control1Spec> {
        I2s0DataCh1SelRxW::new(self, 3)
    }
    #[doc = "Bits 6:8 - I2S 0 Rx channel data channel 2 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch2_sel_rx(&mut self) -> I2s0DataCh2SelRxW<'_, I2s0Control1Spec> {
        I2s0DataCh2SelRxW::new(self, 6)
    }
    #[doc = "Bits 9:11 - I2S 0 Rx channel data channel 3 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch3_sel_rx(&mut self) -> I2s0DataCh3SelRxW<'_, I2s0Control1Spec> {
        I2s0DataCh3SelRxW::new(self, 9)
    }
    #[doc = "Bits 12:14 - I2S 0 Rx channel data channel 4 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch4_sel_rx(&mut self) -> I2s0DataCh4SelRxW<'_, I2s0Control1Spec> {
        I2s0DataCh4SelRxW::new(self, 12)
    }
    #[doc = "Bits 15:17 - I2S 0 Rx channel data channel 5 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch5_sel_rx(&mut self) -> I2s0DataCh5SelRxW<'_, I2s0Control1Spec> {
        I2s0DataCh5SelRxW::new(self, 15)
    }
    #[doc = "Bits 18:20 - I2S 0 Rx channel data channel 6 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch6_sel_rx(&mut self) -> I2s0DataCh6SelRxW<'_, I2s0Control1Spec> {
        I2s0DataCh6SelRxW::new(self, 18)
    }
    #[doc = "Bits 21:23 - I2S 0 Rx channel data channel 7 0: CH0 1: CH1 2/3/4/5/6/7: reserved"]
    #[inline(always)]
    pub fn i2s_0_data_ch7_sel_rx(&mut self) -> I2s0DataCh7SelRxW<'_, I2s0Control1Spec> {
        I2s0DataCh7SelRxW::new(self, 21)
    }
    #[doc = "Bit 24 - I2S 0 Rx channel data channel 0 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch0_rx_disable(&mut self) -> I2s0DataCh0RxDisableW<'_, I2s0Control1Spec> {
        I2s0DataCh0RxDisableW::new(self, 24)
    }
    #[doc = "Bit 25 - I2S 0 Rx channel data channel 1 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch1_rx_disable(&mut self) -> I2s0DataCh1RxDisableW<'_, I2s0Control1Spec> {
        I2s0DataCh1RxDisableW::new(self, 25)
    }
    #[doc = "Bit 26 - I2S 0 Rx channel data channel 2 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch2_rx_disable(&mut self) -> I2s0DataCh2RxDisableW<'_, I2s0Control1Spec> {
        I2s0DataCh2RxDisableW::new(self, 26)
    }
    #[doc = "Bit 27 - I2S 0 Rx channel data channel 3 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch3_rx_disable(&mut self) -> I2s0DataCh3RxDisableW<'_, I2s0Control1Spec> {
        I2s0DataCh3RxDisableW::new(self, 27)
    }
    #[doc = "Bit 28 - I2S 0 Rx channel data channel 4 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch4_rx_disable(&mut self) -> I2s0DataCh4RxDisableW<'_, I2s0Control1Spec> {
        I2s0DataCh4RxDisableW::new(self, 28)
    }
    #[doc = "Bit 29 - I2S 0 Rx channel data channel 5 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch5_rx_disable(&mut self) -> I2s0DataCh5RxDisableW<'_, I2s0Control1Spec> {
        I2s0DataCh5RxDisableW::new(self, 29)
    }
    #[doc = "Bit 30 - I2S 0 Rx channel data channel 6 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch6_rx_disable(&mut self) -> I2s0DataCh6RxDisableW<'_, I2s0Control1Spec> {
        I2s0DataCh6RxDisableW::new(self, 30)
    }
    #[doc = "Bit 31 - I2S 0 Rx channel data channel 7 1: disable 0: enable"]
    #[inline(always)]
    pub fn i2s_0_data_ch7_rx_disable(&mut self) -> I2s0DataCh7RxDisableW<'_, I2s0Control1Spec> {
        I2s0DataCh7RxDisableW::new(self, 31)
    }
}
#[doc = "I2S 0 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_0_control_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_0_control_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2s0Control1Spec;
impl crate::RegisterSpec for I2s0Control1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_0_control_1::R`](R) reader structure"]
impl crate::Readable for I2s0Control1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2s_0_control_1::W`](W) writer structure"]
impl crate::Writable for I2s0Control1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets i2s_0_control_1 to value 0x0002_c688"]
impl crate::Resettable for I2s0Control1Spec {
    const RESET_VALUE: u32 = 0x0002_c688;
}
