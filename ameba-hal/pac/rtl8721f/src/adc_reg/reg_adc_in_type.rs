#[doc = "Register `REG_ADC_IN_TYPE` reader"]
pub type R = crate::R<RegAdcInTypeSpec>;
#[doc = "Register `REG_ADC_IN_TYPE` writer"]
pub type W = crate::W<RegAdcInTypeSpec>;
#[doc = "Field `bit_adc_in_type_ch0` reader - This bit controls channel 0 input type. If it's set to differential type(value of this bit is 1), channel 1 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 0 is single-end input. 1: no use"]
pub type BitAdcInTypeCh0R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch0` writer - This bit controls channel 0 input type. If it's set to differential type(value of this bit is 1), channel 1 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 0 is single-end input. 1: no use"]
pub type BitAdcInTypeCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_in_type_ch1` reader - This bit controls channel 1 input type. If it's set to differential type(value of this bit is 1), channel 0 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 1 is single-end input. 1: no use"]
pub type BitAdcInTypeCh1R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch1` writer - This bit controls channel 1 input type. If it's set to differential type(value of this bit is 1), channel 0 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 1 is single-end input. 1: no use"]
pub type BitAdcInTypeCh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_in_type_ch2` reader - This bit controls channel 2 input type. If it's set to differential type(value of this bit is 1), channel 3 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 2 is single-end input. 1: no use"]
pub type BitAdcInTypeCh2R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch2` writer - This bit controls channel 2 input type. If it's set to differential type(value of this bit is 1), channel 3 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 2 is single-end input. 1: no use"]
pub type BitAdcInTypeCh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_in_type_ch3` reader - This bit controls channel 3 input type. If it's set to differential type(value of this bit is 1), channel 2 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 3 is single-end input. 1: no use"]
pub type BitAdcInTypeCh3R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch3` writer - This bit controls channel 3 input type. If it's set to differential type(value of this bit is 1), channel 2 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 3 is single-end input. 1: no use"]
pub type BitAdcInTypeCh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_in_type_ch4` reader - This bit controls channel 4 input type. If it's set to differential type(value of this bit is 1), channel 5 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 4 is single-end input. 1: no use"]
pub type BitAdcInTypeCh4R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch4` writer - This bit controls channel 4 input type. If it's set to differential type(value of this bit is 1), channel 5 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 4 is single-end input. 1: no use"]
pub type BitAdcInTypeCh4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_in_type_ch5` reader - This bit controls channel 5 input type. If it's set to differential type(value of this bit is 1), channel 4 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 5 is single-end input. 1: no use"]
pub type BitAdcInTypeCh5R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch5` writer - This bit controls channel 5 input type. If it's set to differential type(value of this bit is 1), channel 4 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 5 is single-end input. 1: no use"]
pub type BitAdcInTypeCh5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_in_type_ch6` reader - This bit controls channel 6 input type. If it's set to differential type(value of this bit is 1), channel 5 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 6 is single-end input. 1:no use"]
pub type BitAdcInTypeCh6R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch6` writer - This bit controls channel 6 input type. If it's set to differential type(value of this bit is 1), channel 5 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 6 is single-end input. 1:no use"]
pub type BitAdcInTypeCh6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_in_type_ch7` reader - This bit controls channel 7 input type. If it's set to differential type(value of this bit is 1), channel 6 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 7 is single-end input. 1: no use"]
pub type BitAdcInTypeCh7R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch7` writer - This bit controls channel 7 input type. If it's set to differential type(value of this bit is 1), channel 6 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 7 is single-end input. 1: no use"]
pub type BitAdcInTypeCh7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_in_type_ch8` reader - This bit reflects which kind of input type channel 8 is. Since channel 8 is a fixed internal channel."]
pub type BitAdcInTypeCh8R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch9` reader - This bit reflects which kind of input type channel 9 is. Since channel 9 is a fixed internal channel."]
pub type BitAdcInTypeCh9R = crate::BitReader;
#[doc = "Field `bit_adc_in_type_ch10` reader - This bit reflects which kind of input type channel 10 is. Since channel 10 is a fixed internal channel."]
pub type BitAdcInTypeCh10R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit controls channel 0 input type. If it's set to differential type(value of this bit is 1), channel 1 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 0 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch0(&self) -> BitAdcInTypeCh0R {
        BitAdcInTypeCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit controls channel 1 input type. If it's set to differential type(value of this bit is 1), channel 0 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 1 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch1(&self) -> BitAdcInTypeCh1R {
        BitAdcInTypeCh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit controls channel 2 input type. If it's set to differential type(value of this bit is 1), channel 3 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 2 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch2(&self) -> BitAdcInTypeCh2R {
        BitAdcInTypeCh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit controls channel 3 input type. If it's set to differential type(value of this bit is 1), channel 2 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 3 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch3(&self) -> BitAdcInTypeCh3R {
        BitAdcInTypeCh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit controls channel 4 input type. If it's set to differential type(value of this bit is 1), channel 5 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 4 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch4(&self) -> BitAdcInTypeCh4R {
        BitAdcInTypeCh4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit controls channel 5 input type. If it's set to differential type(value of this bit is 1), channel 4 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 5 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch5(&self) -> BitAdcInTypeCh5R {
        BitAdcInTypeCh5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls channel 6 input type. If it's set to differential type(value of this bit is 1), channel 5 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 6 is single-end input. 1:no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch6(&self) -> BitAdcInTypeCh6R {
        BitAdcInTypeCh6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit controls channel 7 input type. If it's set to differential type(value of this bit is 1), channel 6 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 7 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch7(&self) -> BitAdcInTypeCh7R {
        BitAdcInTypeCh7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reflects which kind of input type channel 8 is. Since channel 8 is a fixed internal channel."]
    #[inline(always)]
    pub fn bit_adc_in_type_ch8(&self) -> BitAdcInTypeCh8R {
        BitAdcInTypeCh8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit reflects which kind of input type channel 9 is. Since channel 9 is a fixed internal channel."]
    #[inline(always)]
    pub fn bit_adc_in_type_ch9(&self) -> BitAdcInTypeCh9R {
        BitAdcInTypeCh9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit reflects which kind of input type channel 10 is. Since channel 10 is a fixed internal channel."]
    #[inline(always)]
    pub fn bit_adc_in_type_ch10(&self) -> BitAdcInTypeCh10R {
        BitAdcInTypeCh10R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls channel 0 input type. If it's set to differential type(value of this bit is 1), channel 1 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 0 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch0(&mut self) -> BitAdcInTypeCh0W<'_, RegAdcInTypeSpec> {
        BitAdcInTypeCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit controls channel 1 input type. If it's set to differential type(value of this bit is 1), channel 0 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 1 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch1(&mut self) -> BitAdcInTypeCh1W<'_, RegAdcInTypeSpec> {
        BitAdcInTypeCh1W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit controls channel 2 input type. If it's set to differential type(value of this bit is 1), channel 3 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 2 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch2(&mut self) -> BitAdcInTypeCh2W<'_, RegAdcInTypeSpec> {
        BitAdcInTypeCh2W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit controls channel 3 input type. If it's set to differential type(value of this bit is 1), channel 2 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 3 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch3(&mut self) -> BitAdcInTypeCh3W<'_, RegAdcInTypeSpec> {
        BitAdcInTypeCh3W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit controls channel 4 input type. If it's set to differential type(value of this bit is 1), channel 5 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 4 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch4(&mut self) -> BitAdcInTypeCh4W<'_, RegAdcInTypeSpec> {
        BitAdcInTypeCh4W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit controls channel 5 input type. If it's set to differential type(value of this bit is 1), channel 4 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 5 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch5(&mut self) -> BitAdcInTypeCh5W<'_, RegAdcInTypeSpec> {
        BitAdcInTypeCh5W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit controls channel 6 input type. If it's set to differential type(value of this bit is 1), channel 5 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 6 is single-end input. 1:no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch6(&mut self) -> BitAdcInTypeCh6W<'_, RegAdcInTypeSpec> {
        BitAdcInTypeCh6W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit controls channel 7 input type. If it's set to differential type(value of this bit is 1), channel 6 should also be set to differential type since they are hardware-fixed differential pair. 0: ADC channel 7 is single-end input. 1: no use"]
    #[inline(always)]
    pub fn bit_adc_in_type_ch7(&mut self) -> BitAdcInTypeCh7W<'_, RegAdcInTypeSpec> {
        BitAdcInTypeCh7W::new(self, 7)
    }
}
#[doc = "ADC Input Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_in_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_in_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcInTypeSpec;
impl crate::RegisterSpec for RegAdcInTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_in_type::R`](R) reader structure"]
impl crate::Readable for RegAdcInTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_in_type::W`](W) writer structure"]
impl crate::Writable for RegAdcInTypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_IN_TYPE to value 0"]
impl crate::Resettable for RegAdcInTypeSpec {}
