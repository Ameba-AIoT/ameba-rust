#[doc = "Register `REG_ADC_DATA_GLOBAL` reader"]
pub type R = crate::R<RegAdcDataGlobalSpec>;
#[doc = "Field `bit_adc_data_global` reader - This field contains the newsest conversion data of channel in the list. \\[15:0\\]: newest data"]
pub type BitAdcDataGlobalR = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_ch_num_global` reader - This field contains the newsest conversion data of channel in the list. \\[19:16\\]: which channel in channel list the data belongs to, only valid when bit_adc_ch_unmask=1"]
pub type BitAdcChNumGlobalR = crate::FieldReader;
#[doc = "Field `bit_adc_dat_ovw_global` reader - This bit indicates that there is a data overwritten situation in bit_adc_data_global takes place. A data overwritten situation is that a former conversion data is NOT read before a new conversion is written to data field. 0: there is no data overwritten case. 1: there is a data overwritten case."]
pub type BitAdcDatOvwGlobalR = crate::BitReader;
#[doc = "Field `bit_adc_dat_rdy_global` reader - This bit indicates that a conversion is done. Every time a conversion is done, this bit should be set to 1 and it would be cleared to 0 when a read operation of reg_adc_data_global"]
pub type BitAdcDatRdyGlobalR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - This field contains the newsest conversion data of channel in the list. \\[15:0\\]: newest data"]
    #[inline(always)]
    pub fn bit_adc_data_global(&self) -> BitAdcDataGlobalR {
        BitAdcDataGlobalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - This field contains the newsest conversion data of channel in the list. \\[19:16\\]: which channel in channel list the data belongs to, only valid when bit_adc_ch_unmask=1"]
    #[inline(always)]
    pub fn bit_adc_ch_num_global(&self) -> BitAdcChNumGlobalR {
        BitAdcChNumGlobalR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - This bit indicates that there is a data overwritten situation in bit_adc_data_global takes place. A data overwritten situation is that a former conversion data is NOT read before a new conversion is written to data field. 0: there is no data overwritten case. 1: there is a data overwritten case."]
    #[inline(always)]
    pub fn bit_adc_dat_ovw_global(&self) -> BitAdcDatOvwGlobalR {
        BitAdcDatOvwGlobalR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit indicates that a conversion is done. Every time a conversion is done, this bit should be set to 1 and it would be cleared to 0 when a read operation of reg_adc_data_global"]
    #[inline(always)]
    pub fn bit_adc_dat_rdy_global(&self) -> BitAdcDatRdyGlobalR {
        BitAdcDatRdyGlobalR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "ADC Global Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_data_global::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcDataGlobalSpec;
impl crate::RegisterSpec for RegAdcDataGlobalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_data_global::R`](R) reader structure"]
impl crate::Readable for RegAdcDataGlobalSpec {}
#[doc = "`reset()` method sets REG_ADC_DATA_GLOBAL to value 0"]
impl crate::Resettable for RegAdcDataGlobalSpec {}
