#[doc = "Register `REG_ADC_DATA_CH3` reader"]
pub type R = crate::R<RegAdcDataCh3Spec>;
#[doc = "Field `bit_adc_data_ch3` reader - This field contains the newsest conversion data of channel 3 \\[15:0\\]: newest data"]
pub type BitAdcDataCh3R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_num_ch3` reader - This field contains the newsest conversion data of channel 3 \\[19:16\\]: which channel in channel list the data belongs to, only valid when bit_adc_ch_unmask=1"]
pub type BitAdcNumCh3R = crate::FieldReader;
#[doc = "Field `bit_adc_dat_ovw_ch3` reader - This bit indicates that there is a data overwritten situation in bit_adc_data_global takeing place. A data overwritten situation is that a former conversion data is NOT read before a new conversion is written to data field. It would be cleared to 0 when a read operation of reg_adc_data_ch3. 0: There is no data overwritten case. 1: There is a data overwritten case."]
pub type BitAdcDatOvwCh3R = crate::BitReader;
#[doc = "Field `bit_adc_dat_rdy_ch3` reader - This bit indicates that a conversion is done. Every time a conversion is done, this bit should be set to 1 and it would be cleared to 0 when a read operation of reg_adc_data_ch3."]
pub type BitAdcDatRdyCh3R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - This field contains the newsest conversion data of channel 3 \\[15:0\\]: newest data"]
    #[inline(always)]
    pub fn bit_adc_data_ch3(&self) -> BitAdcDataCh3R {
        BitAdcDataCh3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - This field contains the newsest conversion data of channel 3 \\[19:16\\]: which channel in channel list the data belongs to, only valid when bit_adc_ch_unmask=1"]
    #[inline(always)]
    pub fn bit_adc_num_ch3(&self) -> BitAdcNumCh3R {
        BitAdcNumCh3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - This bit indicates that there is a data overwritten situation in bit_adc_data_global takeing place. A data overwritten situation is that a former conversion data is NOT read before a new conversion is written to data field. It would be cleared to 0 when a read operation of reg_adc_data_ch3. 0: There is no data overwritten case. 1: There is a data overwritten case."]
    #[inline(always)]
    pub fn bit_adc_dat_ovw_ch3(&self) -> BitAdcDatOvwCh3R {
        BitAdcDatOvwCh3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit indicates that a conversion is done. Every time a conversion is done, this bit should be set to 1 and it would be cleared to 0 when a read operation of reg_adc_data_ch3."]
    #[inline(always)]
    pub fn bit_adc_dat_rdy_ch3(&self) -> BitAdcDatRdyCh3R {
        BitAdcDatRdyCh3R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "ADC Channel 3 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_data_ch3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcDataCh3Spec;
impl crate::RegisterSpec for RegAdcDataCh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_data_ch3::R`](R) reader structure"]
impl crate::Readable for RegAdcDataCh3Spec {}
#[doc = "`reset()` method sets REG_ADC_DATA_CH3 to value 0"]
impl crate::Resettable for RegAdcDataCh3Spec {}
