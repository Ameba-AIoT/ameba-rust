#[doc = "Register `REG_ADC_INTR_STS` reader"]
pub type R = crate::R<RegAdcIntrStsSpec>;
#[doc = "Field `bit_adc_it_cvlist_end_sts` reader - merged"]
pub type BitAdcItCvlistEndStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_cv_end_sts` reader - merged"]
pub type BitAdcItCvEndStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_chcv_end_sts` reader - merged"]
pub type BitAdcItChcvEndStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_full_sts` reader - merged"]
pub type BitAdcItFifoFullStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_over_sts` reader - merged"]
pub type BitAdcItFifoOverStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_fifo_empty_sts` reader - merged"]
pub type BitAdcItFifoEmptyStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_dat_ovw_sts` reader - merged"]
pub type BitAdcItDatOvwStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_err_sts` reader - merged"]
pub type BitAdcItErrStsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch0_sts` reader - merged"]
pub type BitAdcItCompCh0StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch1_sts` reader - merged"]
pub type BitAdcItCompCh1StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch2_sts` reader - merged"]
pub type BitAdcItCompCh2StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch3_sts` reader - merged"]
pub type BitAdcItCompCh3StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch4_sts` reader - merged"]
pub type BitAdcItCompCh4StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch5_sts` reader - merged"]
pub type BitAdcItCompCh5StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch6_sts` reader - merged"]
pub type BitAdcItCompCh6StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch7_sts` reader - merged"]
pub type BitAdcItCompCh7StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch8_sts` reader - merged"]
pub type BitAdcItCompCh8StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch9_sts` reader - merged"]
pub type BitAdcItCompCh9StsR = crate::BitReader;
#[doc = "Field `bit_adc_it_comp_ch10_sts` reader - "]
pub type BitAdcItCompCh10StsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_cvlist_end_sts(&self) -> BitAdcItCvlistEndStsR {
        BitAdcItCvlistEndStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_cv_end_sts(&self) -> BitAdcItCvEndStsR {
        BitAdcItCvEndStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_chcv_end_sts(&self) -> BitAdcItChcvEndStsR {
        BitAdcItChcvEndStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_fifo_full_sts(&self) -> BitAdcItFifoFullStsR {
        BitAdcItFifoFullStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_fifo_over_sts(&self) -> BitAdcItFifoOverStsR {
        BitAdcItFifoOverStsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_fifo_empty_sts(&self) -> BitAdcItFifoEmptyStsR {
        BitAdcItFifoEmptyStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_dat_ovw_sts(&self) -> BitAdcItDatOvwStsR {
        BitAdcItDatOvwStsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_err_sts(&self) -> BitAdcItErrStsR {
        BitAdcItErrStsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch0_sts(&self) -> BitAdcItCompCh0StsR {
        BitAdcItCompCh0StsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch1_sts(&self) -> BitAdcItCompCh1StsR {
        BitAdcItCompCh1StsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch2_sts(&self) -> BitAdcItCompCh2StsR {
        BitAdcItCompCh2StsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch3_sts(&self) -> BitAdcItCompCh3StsR {
        BitAdcItCompCh3StsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch4_sts(&self) -> BitAdcItCompCh4StsR {
        BitAdcItCompCh4StsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch5_sts(&self) -> BitAdcItCompCh5StsR {
        BitAdcItCompCh5StsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch6_sts(&self) -> BitAdcItCompCh6StsR {
        BitAdcItCompCh6StsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch7_sts(&self) -> BitAdcItCompCh7StsR {
        BitAdcItCompCh7StsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch8_sts(&self) -> BitAdcItCompCh8StsR {
        BitAdcItCompCh8StsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - merged"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch9_sts(&self) -> BitAdcItCompCh9StsR {
        BitAdcItCompCh9StsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bit_adc_it_comp_ch10_sts(&self) -> BitAdcItCompCh10StsR {
        BitAdcItCompCh10StsR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "ADC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_intr_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcIntrStsSpec;
impl crate::RegisterSpec for RegAdcIntrStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_intr_sts::R`](R) reader structure"]
impl crate::Readable for RegAdcIntrStsSpec {}
#[doc = "`reset()` method sets REG_ADC_INTR_STS to value 0"]
impl crate::Resettable for RegAdcIntrStsSpec {}
