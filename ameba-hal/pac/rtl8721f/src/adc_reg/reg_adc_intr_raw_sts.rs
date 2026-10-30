#[doc = "Register `REG_ADC_INTR_RAW_STS` reader"]
pub type R = crate::R<RegAdcIntrRawStsSpec>;
#[doc = "Field `bit_adc_it_cvlist_end_raw_sts` reader - merged"]
pub type BitAdcItCvlistEndRawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_cv_end_raw_sts` reader - merged"]
pub type BitAdcItCvEndRawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_chcv_end_raw_sts` reader - merged"]
pub type BitAdcItChcvEndRawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_full_raw_sts` reader - merged"]
pub type BitAdcItFifoFullRawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_over_raw_sts` reader - merged"]
pub type BitAdcItFifoOverRawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_empty_raw_sts` reader - merged"]
pub type BitAdcItFifoEmptyRawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_dat_ovw_raw_sts` reader - merged"]
pub type BitAdcItDatOvwRawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_err_raw_sts` reader - merged"]
pub type BitAdcItErrRawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch0_raw_sts` reader - merged"]
pub type BitAdcItCompCh0RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch1_raw_sts` reader - merged"]
pub type BitAdcItCompCh1RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch2_raw_sts` reader - merged"]
pub type BitAdcItCompCh2RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch3_raw_sts` reader - merged"]
pub type BitAdcItCompCh3RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch4_raw_sts` reader - merged"]
pub type BitAdcItCompCh4RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch5_raw_sts` reader - merged"]
pub type BitAdcItCompCh5RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch6_raw_sts` reader - merged"]
pub type BitAdcItCompCh6RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch7_raw_sts` reader - merged"]
pub type BitAdcItCompCh7RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch8_raw_sts` reader - merged"]
pub type BitAdcItCompCh8RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch9_raw_sts` reader - merged"]
pub type BitAdcItCompCh9RawStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch10_raw_sts` reader - "]
pub type BitAdcItCompCh10RawStsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_cvlist_end_raw_sts(&self) -> BitAdcItCvlistEndRawStsR {
        BitAdcItCvlistEndRawStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_cv_end_raw_sts(&self) -> BitAdcItCvEndRawStsR {
        BitAdcItCvEndRawStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_chcv_end_raw_sts(&self) -> BitAdcItChcvEndRawStsR {
        BitAdcItChcvEndRawStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_fifo_full_raw_sts(&self) -> BitAdcItFifoFullRawStsR {
        BitAdcItFifoFullRawStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_fifo_over_raw_sts(&self) -> BitAdcItFifoOverRawStsR {
        BitAdcItFifoOverRawStsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_fifo_empty_raw_sts(&self) -> BitAdcItFifoEmptyRawStsR {
        BitAdcItFifoEmptyRawStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_dat_ovw_raw_sts(&self) -> BitAdcItDatOvwRawStsR {
        BitAdcItDatOvwRawStsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_err_raw_sts(&self) -> BitAdcItErrRawStsR {
        BitAdcItErrRawStsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch0_raw_sts(&self) -> BitAdcItCompCh0RawStsR {
        BitAdcItCompCh0RawStsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch1_raw_sts(&self) -> BitAdcItCompCh1RawStsR {
        BitAdcItCompCh1RawStsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch2_raw_sts(&self) -> BitAdcItCompCh2RawStsR {
        BitAdcItCompCh2RawStsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch3_raw_sts(&self) -> BitAdcItCompCh3RawStsR {
        BitAdcItCompCh3RawStsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch4_raw_sts(&self) -> BitAdcItCompCh4RawStsR {
        BitAdcItCompCh4RawStsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch5_raw_sts(&self) -> BitAdcItCompCh5RawStsR {
        BitAdcItCompCh5RawStsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch6_raw_sts(&self) -> BitAdcItCompCh6RawStsR {
        BitAdcItCompCh6RawStsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch7_raw_sts(&self) -> BitAdcItCompCh7RawStsR {
        BitAdcItCompCh7RawStsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch8_raw_sts(&self) -> BitAdcItCompCh8RawStsR {
        BitAdcItCompCh8RawStsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch9_raw_sts(&self) -> BitAdcItCompCh9RawStsR {
        BitAdcItCompCh9RawStsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch10_raw_sts(&self) -> BitAdcItCompCh10RawStsR {
        BitAdcItCompCh10RawStsR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "ADC Interrupt Raw Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_intr_raw_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcIntrRawStsSpec;
impl crate::RegisterSpec for RegAdcIntrRawStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_intr_raw_sts::R`](R) reader structure"]
impl crate::Readable for RegAdcIntrRawStsSpec {}
#[doc = "`reset()` method sets REG_ADC_INTR_RAW_STS to value 0x20"]
impl crate::Resettable for RegAdcIntrRawStsSpec {
    const RESET_VALUE: u32 = 0x20;
}
