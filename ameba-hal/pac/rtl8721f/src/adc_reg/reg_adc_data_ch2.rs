#[doc = "Register `REG_ADC_DATA_CH2` reader"]
pub type R = crate::R<RegAdcDataCh2Spec>;
#[doc = "Field `bit_adc_data_ch2` reader - This field contains the newsest conversion data of channel 2 \\[15:0\\]: newest data"]
pub type BitAdcDataCh2R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_num_ch2` reader - This field contains the newsest conversion data of channel 2 \\[19:16\\]: which channel in channel list the data belongs to, only valid when bit_adc_ch_unmask=1"]
pub type BitAdcNumCh2R = crate::FieldReader;
#[doc = "Field `bit_adc_dat_ovw_ch2` reader - This bit indicates that there is a data overwritten situation in bit_adc_data_global takeing place. A data overwritten situation is that a former conversion data is NOT read before a new conversion is written to data field. It would be cleared to 0 when a read operation of reg_adc_data_ch2. 0: There is no data overwritten case. 1: There is a data overwritten case."]
pub type BitAdcDatOvwCh2R = crate::BitReader;
#[doc = "Field `bit_adc_dat_rdy_ch2` reader - This bit indicates that a conversion is done. Every time a conversion is done, this bit should be set to 1 and it would be cleared to 0 when a read operation of reg_adc_data_ch2."]
pub type BitAdcDatRdyCh2R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - This field contains the newsest conversion data of channel 2 \\[15:0\\]: newest data"]
    #[inline(always)]
    pub fn bit_adc_data_ch2(&self) -> BitAdcDataCh2R {
        BitAdcDataCh2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - This field contains the newsest conversion data of channel 2 \\[19:16\\]: which channel in channel list the data belongs to, only valid when bit_adc_ch_unmask=1"]
    #[inline(always)]
    pub fn bit_adc_num_ch2(&self) -> BitAdcNumCh2R {
        BitAdcNumCh2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - This bit indicates that there is a data overwritten situation in bit_adc_data_global takeing place. A data overwritten situation is that a former conversion data is NOT read before a new conversion is written to data field. It would be cleared to 0 when a read operation of reg_adc_data_ch2. 0: There is no data overwritten case. 1: There is a data overwritten case."]
    #[inline(always)]
    pub fn bit_adc_dat_ovw_ch2(&self) -> BitAdcDatOvwCh2R {
        BitAdcDatOvwCh2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit indicates that a conversion is done. Every time a conversion is done, this bit should be set to 1 and it would be cleared to 0 when a read operation of reg_adc_data_ch2."]
    #[inline(always)]
    pub fn bit_adc_dat_rdy_ch2(&self) -> BitAdcDatRdyCh2R {
        BitAdcDatRdyCh2R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "ADC Channel 2 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_data_ch2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcDataCh2Spec;
impl crate::RegisterSpec for RegAdcDataCh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_data_ch2::R`](R) reader structure"]
impl crate::Readable for RegAdcDataCh2Spec {}
#[doc = "`reset()` method sets REG_ADC_DATA_CH2 to value 0"]
impl crate::Resettable for RegAdcDataCh2Spec {}
