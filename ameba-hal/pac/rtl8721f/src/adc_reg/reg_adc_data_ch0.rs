#[doc = "Register `REG_ADC_DATA_CH0` reader"]
pub type R = crate::R<RegAdcDataCh0Spec>;
#[doc = "Field `bit_adc_data_ch0` reader - This field contains the newsest conversion data of channel 0 \\[15:0\\]: newest data"]
pub type BitAdcDataCh0R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_num_ch0` reader - This field contains the newsest conversion data of channel 0 \\[19:16\\]: which channel in channel list the data belongs to, only valid when bit_adc_ch_unmask=1"]
pub type BitAdcNumCh0R = crate::FieldReader;
#[doc = "Field `bit_adc_dat_ovw_ch0` reader - This bit indicates that there is a data overwritten situation in bit_adc_data_global takeing place. A data overwritten situation is that a former conversion data is NOT read before a new conversion is written to data field. It would be cleared to 0 when a read operation of reg_adc_data_ch0. 0: There is no data overwritten case. 1: There is a data overwritten case."]
pub type BitAdcDatOvwCh0R = crate::BitReader;
#[doc = "Field `bit_adc_dat_rdy_ch0` reader - This bit indicates that a conversion is done. Every time a conversion is done, this bit should be set to 1 and it would be cleared to 0 when a read operation of reg_adc_data_ch0."]
pub type BitAdcDatRdyCh0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - This field contains the newsest conversion data of channel 0 \\[15:0\\]: newest data"]
    #[inline(always)]
    pub fn bit_adc_data_ch0(&self) -> BitAdcDataCh0R {
        BitAdcDataCh0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - This field contains the newsest conversion data of channel 0 \\[19:16\\]: which channel in channel list the data belongs to, only valid when bit_adc_ch_unmask=1"]
    #[inline(always)]
    pub fn bit_adc_num_ch0(&self) -> BitAdcNumCh0R {
        BitAdcNumCh0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - This bit indicates that there is a data overwritten situation in bit_adc_data_global takeing place. A data overwritten situation is that a former conversion data is NOT read before a new conversion is written to data field. It would be cleared to 0 when a read operation of reg_adc_data_ch0. 0: There is no data overwritten case. 1: There is a data overwritten case."]
    #[inline(always)]
    pub fn bit_adc_dat_ovw_ch0(&self) -> BitAdcDatOvwCh0R {
        BitAdcDatOvwCh0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit indicates that a conversion is done. Every time a conversion is done, this bit should be set to 1 and it would be cleared to 0 when a read operation of reg_adc_data_ch0."]
    #[inline(always)]
    pub fn bit_adc_dat_rdy_ch0(&self) -> BitAdcDatRdyCh0R {
        BitAdcDatRdyCh0R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "ADC Channel 0 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_data_ch0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcDataCh0Spec;
impl crate::RegisterSpec for RegAdcDataCh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_data_ch0::R`](R) reader structure"]
impl crate::Readable for RegAdcDataCh0Spec {}
#[doc = "`reset()` method sets REG_ADC_DATA_CH0 to value 0"]
impl crate::Resettable for RegAdcDataCh0Spec {}
