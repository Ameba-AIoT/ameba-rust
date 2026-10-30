#[doc = "Register `REG_ADC_SW_TMODE` reader"]
pub type R = crate::R<RegAdcSwTmodeSpec>;
#[doc = "Register `REG_ADC_SW_TMODE` writer"]
pub type W = crate::W<RegAdcSwTmodeSpec>;
#[doc = "Field `bit_adc_sw_tmode` reader - This bit is used to change the high 4 bits in FIFO. 0: normal mode, the high 4 bits will be null or the channel number 1: software test mode, the high 4 bits will be the counter number of sample points. (loop from 0 to 15)"]
pub type BitAdcSwTmodeR = crate::BitReader;
#[doc = "Field `bit_adc_sw_tmode` writer - This bit is used to change the high 4 bits in FIFO. 0: normal mode, the high 4 bits will be null or the channel number 1: software test mode, the high 4 bits will be the counter number of sample points. (loop from 0 to 15)"]
pub type BitAdcSwTmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to change the high 4 bits in FIFO. 0: normal mode, the high 4 bits will be null or the channel number 1: software test mode, the high 4 bits will be the counter number of sample points. (loop from 0 to 15)"]
    #[inline(always)]
    pub fn bit_adc_sw_tmode(&self) -> BitAdcSwTmodeR {
        BitAdcSwTmodeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to change the high 4 bits in FIFO. 0: normal mode, the high 4 bits will be null or the channel number 1: software test mode, the high 4 bits will be the counter number of sample points. (loop from 0 to 15)"]
    #[inline(always)]
    pub fn bit_adc_sw_tmode(&mut self) -> BitAdcSwTmodeW<'_, RegAdcSwTmodeSpec> {
        BitAdcSwTmodeW::new(self, 0)
    }
}
#[doc = "ADC Software Test Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_sw_tmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_sw_tmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcSwTmodeSpec;
impl crate::RegisterSpec for RegAdcSwTmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_sw_tmode::R`](R) reader structure"]
impl crate::Readable for RegAdcSwTmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_sw_tmode::W`](W) writer structure"]
impl crate::Writable for RegAdcSwTmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_SW_TMODE to value 0"]
impl crate::Resettable for RegAdcSwTmodeSpec {}
