#[doc = "Register `REG_ADC_BUSY_STS` reader"]
pub type R = crate::R<RegAdcBusyStsSpec>;
#[doc = "Field `bit_adc_busy_sts` reader - This bit reflects the ADC is busy or not. If the ADC is processing a conversion of a channel, this bit remains 1 which indicates it's busy. Once a conversion is done, this bit becomes 0 which indicates it's ready to do another conversion. 0: The ADC is ready. 1: The ADC is busy."]
pub type BitAdcBusyStsR = crate::BitReader;
#[doc = "Field `bit_adc_fifo_full_real` reader - 0: FIFO in ADC not full real; 1: FIFO in ADC full real"]
pub type BitAdcFifoFullRealR = crate::BitReader;
#[doc = "Field `bit_adc_fifo_empty` reader - 0: FIFO in ADC not empty; 1: FIFO in ADC empty"]
pub type BitAdcFifoEmptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit reflects the ADC is busy or not. If the ADC is processing a conversion of a channel, this bit remains 1 which indicates it's busy. Once a conversion is done, this bit becomes 0 which indicates it's ready to do another conversion. 0: The ADC is ready. 1: The ADC is busy."]
    #[inline(always)]
    pub fn bit_adc_busy_sts(&self) -> BitAdcBusyStsR {
        BitAdcBusyStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: FIFO in ADC not full real; 1: FIFO in ADC full real"]
    #[inline(always)]
    pub fn bit_adc_fifo_full_real(&self) -> BitAdcFifoFullRealR {
        BitAdcFifoFullRealR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: FIFO in ADC not empty; 1: FIFO in ADC empty"]
    #[inline(always)]
    pub fn bit_adc_fifo_empty(&self) -> BitAdcFifoEmptyR {
        BitAdcFifoEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ADC Busy Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_busy_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcBusyStsSpec;
impl crate::RegisterSpec for RegAdcBusyStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_busy_sts::R`](R) reader structure"]
impl crate::Readable for RegAdcBusyStsSpec {}
#[doc = "`reset()` method sets REG_ADC_BUSY_STS to value 0x04"]
impl crate::Resettable for RegAdcBusyStsSpec {
    const RESET_VALUE: u32 = 0x04;
}
