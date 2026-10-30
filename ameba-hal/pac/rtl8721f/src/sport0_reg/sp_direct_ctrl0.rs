#[doc = "Register `SP_DIRECT_CTRL0` reader"]
pub type R = crate::R<SpDirectCtrl0Spec>;
#[doc = "Register `SP_DIRECT_CTRL0` writer"]
pub type W = crate::W<SpDirectCtrl0Spec>;
#[doc = "Field `tx_ch0_data_sel` reader - (Ibid.) 4'h8: direct_reg_0"]
pub type TxCh0DataSelR = crate::FieldReader;
#[doc = "Field `tx_ch0_data_sel` writer - (Ibid.) 4'h8: direct_reg_0"]
pub type TxCh0DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tx_ch1_data_sel` reader - (Ibid.) 4'h8: direct_reg_1"]
pub type TxCh1DataSelR = crate::FieldReader;
#[doc = "Field `tx_ch1_data_sel` writer - (Ibid.) 4'h8: direct_reg_1"]
pub type TxCh1DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tx_ch2_data_sel` reader - (Ibid.) 4'h8: direct_reg_2"]
pub type TxCh2DataSelR = crate::FieldReader;
#[doc = "Field `tx_ch2_data_sel` writer - (Ibid.) 4'h8: direct_reg_2"]
pub type TxCh2DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tx_ch3_data_sel` reader - (Ibid.) 4'h8: direct_reg_3"]
pub type TxCh3DataSelR = crate::FieldReader;
#[doc = "Field `tx_ch3_data_sel` writer - (Ibid.) 4'h8: direct_reg_3"]
pub type TxCh3DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tx_ch4_data_sel` reader - (Ibid.) 4'h8: direct_reg_4"]
pub type TxCh4DataSelR = crate::FieldReader;
#[doc = "Field `tx_ch4_data_sel` writer - (Ibid.) 4'h8: direct_reg_4"]
pub type TxCh4DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tx_ch5_data_sel` reader - (Ibid.) 4'h8: direct_reg_5"]
pub type TxCh5DataSelR = crate::FieldReader;
#[doc = "Field `tx_ch5_data_sel` writer - (Ibid.) 4'h8: direct_reg_5"]
pub type TxCh5DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tx_ch6_data_sel` reader - (Ibid.) 4'h8: direct_reg_6"]
pub type TxCh6DataSelR = crate::FieldReader;
#[doc = "Field `tx_ch6_data_sel` writer - (Ibid.) 4'h8: direct_reg_6"]
pub type TxCh6DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tx_ch7_data_sel` reader - 4'h0: tx_fifo_0_reg_0_l 4'h1: tx_fifo_0_reg_0_r 4'h2: tx_fifo_0_reg_1_l 4'h3: tx_fifo_0_reg_1_r 4'h4: tx_fifo_1_reg_0_l 4'h5: tx_fifo_1_reg_0_r 4'h6: tx_fifo_1_reg_1_l 4'h7: tx_fifo_1_reg_1_r 4'h8: direct_reg_7"]
pub type TxCh7DataSelR = crate::FieldReader;
#[doc = "Field `tx_ch7_data_sel` writer - 4'h0: tx_fifo_0_reg_0_l 4'h1: tx_fifo_0_reg_0_r 4'h2: tx_fifo_0_reg_1_l 4'h3: tx_fifo_0_reg_1_r 4'h4: tx_fifo_1_reg_0_l 4'h5: tx_fifo_1_reg_0_r 4'h6: tx_fifo_1_reg_1_l 4'h7: tx_fifo_1_reg_1_r 4'h8: direct_reg_7"]
pub type TxCh7DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - (Ibid.) 4'h8: direct_reg_0"]
    #[inline(always)]
    pub fn tx_ch0_data_sel(&self) -> TxCh0DataSelR {
        TxCh0DataSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - (Ibid.) 4'h8: direct_reg_1"]
    #[inline(always)]
    pub fn tx_ch1_data_sel(&self) -> TxCh1DataSelR {
        TxCh1DataSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - (Ibid.) 4'h8: direct_reg_2"]
    #[inline(always)]
    pub fn tx_ch2_data_sel(&self) -> TxCh2DataSelR {
        TxCh2DataSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - (Ibid.) 4'h8: direct_reg_3"]
    #[inline(always)]
    pub fn tx_ch3_data_sel(&self) -> TxCh3DataSelR {
        TxCh3DataSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - (Ibid.) 4'h8: direct_reg_4"]
    #[inline(always)]
    pub fn tx_ch4_data_sel(&self) -> TxCh4DataSelR {
        TxCh4DataSelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - (Ibid.) 4'h8: direct_reg_5"]
    #[inline(always)]
    pub fn tx_ch5_data_sel(&self) -> TxCh5DataSelR {
        TxCh5DataSelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - (Ibid.) 4'h8: direct_reg_6"]
    #[inline(always)]
    pub fn tx_ch6_data_sel(&self) -> TxCh6DataSelR {
        TxCh6DataSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 4'h0: tx_fifo_0_reg_0_l 4'h1: tx_fifo_0_reg_0_r 4'h2: tx_fifo_0_reg_1_l 4'h3: tx_fifo_0_reg_1_r 4'h4: tx_fifo_1_reg_0_l 4'h5: tx_fifo_1_reg_0_r 4'h6: tx_fifo_1_reg_1_l 4'h7: tx_fifo_1_reg_1_r 4'h8: direct_reg_7"]
    #[inline(always)]
    pub fn tx_ch7_data_sel(&self) -> TxCh7DataSelR {
        TxCh7DataSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - (Ibid.) 4'h8: direct_reg_0"]
    #[inline(always)]
    pub fn tx_ch0_data_sel(&mut self) -> TxCh0DataSelW<'_, SpDirectCtrl0Spec> {
        TxCh0DataSelW::new(self, 0)
    }
    #[doc = "Bits 4:7 - (Ibid.) 4'h8: direct_reg_1"]
    #[inline(always)]
    pub fn tx_ch1_data_sel(&mut self) -> TxCh1DataSelW<'_, SpDirectCtrl0Spec> {
        TxCh1DataSelW::new(self, 4)
    }
    #[doc = "Bits 8:11 - (Ibid.) 4'h8: direct_reg_2"]
    #[inline(always)]
    pub fn tx_ch2_data_sel(&mut self) -> TxCh2DataSelW<'_, SpDirectCtrl0Spec> {
        TxCh2DataSelW::new(self, 8)
    }
    #[doc = "Bits 12:15 - (Ibid.) 4'h8: direct_reg_3"]
    #[inline(always)]
    pub fn tx_ch3_data_sel(&mut self) -> TxCh3DataSelW<'_, SpDirectCtrl0Spec> {
        TxCh3DataSelW::new(self, 12)
    }
    #[doc = "Bits 16:19 - (Ibid.) 4'h8: direct_reg_4"]
    #[inline(always)]
    pub fn tx_ch4_data_sel(&mut self) -> TxCh4DataSelW<'_, SpDirectCtrl0Spec> {
        TxCh4DataSelW::new(self, 16)
    }
    #[doc = "Bits 20:23 - (Ibid.) 4'h8: direct_reg_5"]
    #[inline(always)]
    pub fn tx_ch5_data_sel(&mut self) -> TxCh5DataSelW<'_, SpDirectCtrl0Spec> {
        TxCh5DataSelW::new(self, 20)
    }
    #[doc = "Bits 24:27 - (Ibid.) 4'h8: direct_reg_6"]
    #[inline(always)]
    pub fn tx_ch6_data_sel(&mut self) -> TxCh6DataSelW<'_, SpDirectCtrl0Spec> {
        TxCh6DataSelW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 4'h0: tx_fifo_0_reg_0_l 4'h1: tx_fifo_0_reg_0_r 4'h2: tx_fifo_0_reg_1_l 4'h3: tx_fifo_0_reg_1_r 4'h4: tx_fifo_1_reg_0_l 4'h5: tx_fifo_1_reg_0_r 4'h6: tx_fifo_1_reg_1_l 4'h7: tx_fifo_1_reg_1_r 4'h8: direct_reg_7"]
    #[inline(always)]
    pub fn tx_ch7_data_sel(&mut self) -> TxCh7DataSelW<'_, SpDirectCtrl0Spec> {
        TxCh7DataSelW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_direct_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_direct_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpDirectCtrl0Spec;
impl crate::RegisterSpec for SpDirectCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_direct_ctrl0::R`](R) reader structure"]
impl crate::Readable for SpDirectCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_direct_ctrl0::W`](W) writer structure"]
impl crate::Writable for SpDirectCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_DIRECT_CTRL0 to value 0x7654_3210"]
impl crate::Resettable for SpDirectCtrl0Spec {
    const RESET_VALUE: u32 = 0x7654_3210;
}
