#[doc = "Register `REG_ADC_INTR_CTRL` reader"]
pub type R = crate::R<RegAdcIntrCtrlSpec>;
#[doc = "Register `REG_ADC_INTR_CTRL` writer"]
pub type W = crate::W<RegAdcIntrCtrlSpec>;
#[doc = "Field `bit_adc_it_cvlist_end_en` reader - This bit controls the interrupt is enabled or not when a conversion of the last channel in the list is done. For example, in automatic mode conversions would be executed continuously. Every time the last channel conversion is done, which means all channel conversions in the list is done, ADC could notify system if This bit is set. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItCvlistEndEnR = crate::BitReader;
#[doc = "Field `bit_adc_it_cvlist_end_en` writer - This bit controls the interrupt is enabled or not when a conversion of the last channel in the list is done. For example, in automatic mode conversions would be executed continuously. Every time the last channel conversion is done, which means all channel conversions in the list is done, ADC could notify system if This bit is set. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItCvlistEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_cv_end_en` reader - This bit controls the interrupt is enabled or not every time a conversion is done. No matter ADC module is in what kind of operation mode. Every time a conversion is executed, ADC module would notify system if This bit is set. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItCvEndEnR = crate::BitReader;
#[doc = "Field `bit_adc_it_cv_end_en` writer - This bit controls the interrupt is enabled or not every time a conversion is done. No matter ADC module is in what kind of operation mode. Every time a conversion is executed, ADC module would notify system if This bit is set. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItCvEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_chcv_end_en` reader - This bit controls the interrupt is enabled or not when a particular channel conversion is done. Please refer to reg_adc_it_chno_con 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItChcvEndEnR = crate::BitReader;
#[doc = "Field `bit_adc_it_chcv_end_en` writer - This bit controls the interrupt is enabled or not when a particular channel conversion is done. Please refer to reg_adc_it_chno_con 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItChcvEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_fifo_full_en` reader - This bit controls the interrupt is enabled or not when a FIFO full state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItFifoFullEnR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_full_en` writer - This bit controls the interrupt is enabled or not when a FIFO full state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItFifoFullEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_fifo_over_en` reader - This bit controls the interrupt is enabled or not when a FIFO overflow state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItFifoOverEnR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_over_en` writer - This bit controls the interrupt is enabled or not when a FIFO overflow state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItFifoOverEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_fifo_empty_en` reader - This bit controls the interrupt is enabled or not when a FIFO empty state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItFifoEmptyEnR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_empty_en` writer - This bit controls the interrupt is enabled or not when a FIFO empty state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItFifoEmptyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_dat_ovw_en` reader - This bit controls the interrupt is enabled or not when a data overwritten situation takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItDatOvwEnR = crate::BitReader;
#[doc = "Field `bit_adc_it_dat_ovw_en` writer - This bit controls the interrupt is enabled or not when a data overwritten situation takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItDatOvwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_err_en` reader - This bit controls the interrupt is enabled or not when an error state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItErrEnR = crate::BitReader;
#[doc = "Field `bit_adc_it_err_en` writer - This bit controls the interrupt is enabled or not when an error state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch0_en` reader - This bit controls the interrupt is enabled or not when channel 0 comparison criterion matches. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItCompCh0EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch0_en` writer - This bit controls the interrupt is enabled or not when channel 0 comparison criterion matches. 0: This interrupt is disabled. 1: This interrupt is enabled."]
pub type BitAdcItCompCh0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch1_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh1EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch1_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch2_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh2EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch2_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch3_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh3EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch3_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch4_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh4EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch4_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch5_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh5EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch5_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch6_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh6EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch6_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch7_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh7EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch7_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch8_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh8EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch8_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch9_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh9EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch9_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh9EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_ch10_en` reader - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh10EnR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch10_en` writer - the function is same as bit_adc_it_comp_ch0_en"]
pub type BitAdcItCompCh10EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_it_comp_det_mod` reader - This bit controls the interrupt bit_adc_it_comp_chx operation. comparison criterion matching is rising edge. 0: The detection mode is level detection. when the comparison criterion is match, bit_adc_it_comp_chx will send interrupt continuously. 1: The detection mode is edge detection. when the comparison criterion is match, bit_adc_it_comp_chx will send interrupt once at the time of matching."]
pub type BitAdcItCompDetModR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_det_mod` writer - This bit controls the interrupt bit_adc_it_comp_chx operation. comparison criterion matching is rising edge. 0: The detection mode is level detection. when the comparison criterion is match, bit_adc_it_comp_chx will send interrupt continuously. 1: The detection mode is edge detection. when the comparison criterion is match, bit_adc_it_comp_chx will send interrupt once at the time of matching."]
pub type BitAdcItCompDetModW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit controls the interrupt is enabled or not when a conversion of the last channel in the list is done. For example, in automatic mode conversions would be executed continuously. Every time the last channel conversion is done, which means all channel conversions in the list is done, ADC could notify system if This bit is set. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_cvlist_end_en(&self) -> BitAdcItCvlistEndEnR {
        BitAdcItCvlistEndEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit controls the interrupt is enabled or not every time a conversion is done. No matter ADC module is in what kind of operation mode. Every time a conversion is executed, ADC module would notify system if This bit is set. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_cv_end_en(&self) -> BitAdcItCvEndEnR {
        BitAdcItCvEndEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit controls the interrupt is enabled or not when a particular channel conversion is done. Please refer to reg_adc_it_chno_con 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_chcv_end_en(&self) -> BitAdcItChcvEndEnR {
        BitAdcItChcvEndEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit controls the interrupt is enabled or not when a FIFO full state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_fifo_full_en(&self) -> BitAdcItFifoFullEnR {
        BitAdcItFifoFullEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit controls the interrupt is enabled or not when a FIFO overflow state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_fifo_over_en(&self) -> BitAdcItFifoOverEnR {
        BitAdcItFifoOverEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit controls the interrupt is enabled or not when a FIFO empty state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_fifo_empty_en(&self) -> BitAdcItFifoEmptyEnR {
        BitAdcItFifoEmptyEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls the interrupt is enabled or not when a data overwritten situation takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_dat_ovw_en(&self) -> BitAdcItDatOvwEnR {
        BitAdcItDatOvwEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit controls the interrupt is enabled or not when an error state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_err_en(&self) -> BitAdcItErrEnR {
        BitAdcItErrEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit controls the interrupt is enabled or not when channel 0 comparison criterion matches. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch0_en(&self) -> BitAdcItCompCh0EnR {
        BitAdcItCompCh0EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch1_en(&self) -> BitAdcItCompCh1EnR {
        BitAdcItCompCh1EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch2_en(&self) -> BitAdcItCompCh2EnR {
        BitAdcItCompCh2EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch3_en(&self) -> BitAdcItCompCh3EnR {
        BitAdcItCompCh3EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch4_en(&self) -> BitAdcItCompCh4EnR {
        BitAdcItCompCh4EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch5_en(&self) -> BitAdcItCompCh5EnR {
        BitAdcItCompCh5EnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch6_en(&self) -> BitAdcItCompCh6EnR {
        BitAdcItCompCh6EnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch7_en(&self) -> BitAdcItCompCh7EnR {
        BitAdcItCompCh7EnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch8_en(&self) -> BitAdcItCompCh8EnR {
        BitAdcItCompCh8EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch9_en(&self) -> BitAdcItCompCh9EnR {
        BitAdcItCompCh9EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch10_en(&self) -> BitAdcItCompCh10EnR {
        BitAdcItCompCh10EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit controls the interrupt bit_adc_it_comp_chx operation. comparison criterion matching is rising edge. 0: The detection mode is level detection. when the comparison criterion is match, bit_adc_it_comp_chx will send interrupt continuously. 1: The detection mode is edge detection. when the comparison criterion is match, bit_adc_it_comp_chx will send interrupt once at the time of matching."]
    #[inline(always)]
    pub fn bit_adc_it_comp_det_mod(&self) -> BitAdcItCompDetModR {
        BitAdcItCompDetModR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls the interrupt is enabled or not when a conversion of the last channel in the list is done. For example, in automatic mode conversions would be executed continuously. Every time the last channel conversion is done, which means all channel conversions in the list is done, ADC could notify system if This bit is set. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_cvlist_end_en(&mut self) -> BitAdcItCvlistEndEnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCvlistEndEnW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit controls the interrupt is enabled or not every time a conversion is done. No matter ADC module is in what kind of operation mode. Every time a conversion is executed, ADC module would notify system if This bit is set. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_cv_end_en(&mut self) -> BitAdcItCvEndEnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCvEndEnW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit controls the interrupt is enabled or not when a particular channel conversion is done. Please refer to reg_adc_it_chno_con 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_chcv_end_en(&mut self) -> BitAdcItChcvEndEnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItChcvEndEnW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit controls the interrupt is enabled or not when a FIFO full state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_fifo_full_en(&mut self) -> BitAdcItFifoFullEnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItFifoFullEnW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit controls the interrupt is enabled or not when a FIFO overflow state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_fifo_over_en(&mut self) -> BitAdcItFifoOverEnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItFifoOverEnW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit controls the interrupt is enabled or not when a FIFO empty state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_fifo_empty_en(&mut self) -> BitAdcItFifoEmptyEnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItFifoEmptyEnW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit controls the interrupt is enabled or not when a data overwritten situation takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_dat_ovw_en(&mut self) -> BitAdcItDatOvwEnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItDatOvwEnW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit controls the interrupt is enabled or not when an error state takes place. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_err_en(&mut self) -> BitAdcItErrEnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItErrEnW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit controls the interrupt is enabled or not when channel 0 comparison criterion matches. 0: This interrupt is disabled. 1: This interrupt is enabled."]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch0_en(&mut self) -> BitAdcItCompCh0EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh0EnW::new(self, 8)
    }
    #[doc = "Bit 9 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch1_en(&mut self) -> BitAdcItCompCh1EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh1EnW::new(self, 9)
    }
    #[doc = "Bit 10 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch2_en(&mut self) -> BitAdcItCompCh2EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh2EnW::new(self, 10)
    }
    #[doc = "Bit 11 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch3_en(&mut self) -> BitAdcItCompCh3EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh3EnW::new(self, 11)
    }
    #[doc = "Bit 12 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch4_en(&mut self) -> BitAdcItCompCh4EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh4EnW::new(self, 12)
    }
    #[doc = "Bit 13 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch5_en(&mut self) -> BitAdcItCompCh5EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh5EnW::new(self, 13)
    }
    #[doc = "Bit 14 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch6_en(&mut self) -> BitAdcItCompCh6EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh6EnW::new(self, 14)
    }
    #[doc = "Bit 15 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch7_en(&mut self) -> BitAdcItCompCh7EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh7EnW::new(self, 15)
    }
    #[doc = "Bit 16 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch8_en(&mut self) -> BitAdcItCompCh8EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh8EnW::new(self, 16)
    }
    #[doc = "Bit 17 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch9_en(&mut self) -> BitAdcItCompCh9EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh9EnW::new(self, 17)
    }
    #[doc = "Bit 18 - the function is same as bit_adc_it_comp_ch0_en"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch10_en(&mut self) -> BitAdcItCompCh10EnW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompCh10EnW::new(self, 18)
    }
    #[doc = "Bit 31 - This bit controls the interrupt bit_adc_it_comp_chx operation. comparison criterion matching is rising edge. 0: The detection mode is level detection. when the comparison criterion is match, bit_adc_it_comp_chx will send interrupt continuously. 1: The detection mode is edge detection. when the comparison criterion is match, bit_adc_it_comp_chx will send interrupt once at the time of matching."]
    #[inline(always)]
    pub fn bit_adc_it_comp_det_mod(&mut self) -> BitAdcItCompDetModW<'_, RegAdcIntrCtrlSpec> {
        BitAdcItCompDetModW::new(self, 31)
    }
}
#[doc = "ADC Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_intr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_intr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcIntrCtrlSpec;
impl crate::RegisterSpec for RegAdcIntrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_intr_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAdcIntrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_intr_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAdcIntrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_INTR_CTRL to value 0"]
impl crate::Resettable for RegAdcIntrCtrlSpec {}
