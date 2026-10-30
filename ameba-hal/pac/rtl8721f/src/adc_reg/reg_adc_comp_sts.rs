#[doc = "Register `REG_ADC_COMP_STS` reader"]
pub type R = crate::R<RegAdcCompStsSpec>;
#[doc = "Field `bit_adc_comp_sts_ch0` reader - this field reflects the comparison result of channel 0 immediately"]
pub type BitAdcCompStsCh0R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch1` reader - this field reflects the comparison result of channel 1 immediately"]
pub type BitAdcCompStsCh1R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch2` reader - this field reflects the comparison result of channel 2 immediately"]
pub type BitAdcCompStsCh2R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch3` reader - this field reflects the comparison result of channel 3 immediately"]
pub type BitAdcCompStsCh3R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch4` reader - this field reflects the comparison result of channel 4 immediately"]
pub type BitAdcCompStsCh4R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch5` reader - this field reflects the comparison result of channel 5 immediately"]
pub type BitAdcCompStsCh5R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch6` reader - this field reflects the comparison result of channel 6 immediately"]
pub type BitAdcCompStsCh6R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch7` reader - this field reflects the comparison result of channel 7 immediately"]
pub type BitAdcCompStsCh7R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch8` reader - this field reflects the comparison result of channel 8 immediately"]
pub type BitAdcCompStsCh8R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch9` reader - this field reflects the comparison result of channel 9 immediately"]
pub type BitAdcCompStsCh9R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_sts_ch10` reader - this field reflects the comparison result of channel 10 immediately"]
pub type BitAdcCompStsCh10R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - this field reflects the comparison result of channel 0 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch0(&self) -> BitAdcCompStsCh0R {
        BitAdcCompStsCh0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - this field reflects the comparison result of channel 1 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch1(&self) -> BitAdcCompStsCh1R {
        BitAdcCompStsCh1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - this field reflects the comparison result of channel 2 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch2(&self) -> BitAdcCompStsCh2R {
        BitAdcCompStsCh2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - this field reflects the comparison result of channel 3 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch3(&self) -> BitAdcCompStsCh3R {
        BitAdcCompStsCh3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - this field reflects the comparison result of channel 4 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch4(&self) -> BitAdcCompStsCh4R {
        BitAdcCompStsCh4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - this field reflects the comparison result of channel 5 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch5(&self) -> BitAdcCompStsCh5R {
        BitAdcCompStsCh5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - this field reflects the comparison result of channel 6 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch6(&self) -> BitAdcCompStsCh6R {
        BitAdcCompStsCh6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - this field reflects the comparison result of channel 7 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch7(&self) -> BitAdcCompStsCh7R {
        BitAdcCompStsCh7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - this field reflects the comparison result of channel 8 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch8(&self) -> BitAdcCompStsCh8R {
        BitAdcCompStsCh8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - this field reflects the comparison result of channel 9 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch9(&self) -> BitAdcCompStsCh9R {
        BitAdcCompStsCh9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - this field reflects the comparison result of channel 10 immediately"]
    #[inline(always)]
    pub fn bit_adc_comp_sts_ch10(&self) -> BitAdcCompStsCh10R {
        BitAdcCompStsCh10R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[doc = "ADC Comparison Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompStsSpec;
impl crate::RegisterSpec for RegAdcCompStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_sts::R`](R) reader structure"]
impl crate::Readable for RegAdcCompStsSpec {}
#[doc = "`reset()` method sets REG_ADC_COMP_STS to value 0"]
impl crate::Resettable for RegAdcCompStsSpec {}
