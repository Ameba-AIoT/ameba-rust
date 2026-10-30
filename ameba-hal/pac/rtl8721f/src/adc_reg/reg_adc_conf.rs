#[doc = "Register `REG_ADC_CONF` reader"]
pub type R = crate::R<RegAdcConfSpec>;
#[doc = "Register `REG_ADC_CONF` writer"]
pub type W = crate::W<RegAdcConfSpec>;
#[doc = "Field `bit_adc_op_mod` reader - These bits selects ADC operation mode. 0: software trigger mode. 1:automatic mode. 2:timer-trigger mode. 3:comparator-assist mode"]
pub type BitAdcOpModR = crate::FieldReader;
#[doc = "Field `bit_adc_op_mod` writer - These bits selects ADC operation mode. 0: software trigger mode. 1:automatic mode. 2:timer-trigger mode. 3:comparator-assist mode"]
pub type BitAdcOpModW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `bit_adc_cvlist_len` reader - This field defines the number of items in the ADC conversion channel list."]
pub type BitAdcCvlistLenR = crate::FieldReader;
#[doc = "Field `bit_adc_cvlist_len` writer - This field defines the number of items in the ADC conversion channel list."]
pub type BitAdcCvlistLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_enable` reader - This bit is for ADC enable control"]
pub type BitAdcEnableR = crate::BitReader;
#[doc = "Field `bit_adc_enable` writer - This bit is for ADC enable control"]
pub type BitAdcEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:3 - These bits selects ADC operation mode. 0: software trigger mode. 1:automatic mode. 2:timer-trigger mode. 3:comparator-assist mode"]
    #[inline(always)]
    pub fn bit_adc_op_mod(&self) -> BitAdcOpModR {
        BitAdcOpModR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - This field defines the number of items in the ADC conversion channel list."]
    #[inline(always)]
    pub fn bit_adc_cvlist_len(&self) -> BitAdcCvlistLenR {
        BitAdcCvlistLenR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - This bit is for ADC enable control"]
    #[inline(always)]
    pub fn bit_adc_enable(&self) -> BitAdcEnableR {
        BitAdcEnableR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - These bits selects ADC operation mode. 0: software trigger mode. 1:automatic mode. 2:timer-trigger mode. 3:comparator-assist mode"]
    #[inline(always)]
    pub fn bit_adc_op_mod(&mut self) -> BitAdcOpModW<'_, RegAdcConfSpec> {
        BitAdcOpModW::new(self, 1)
    }
    #[doc = "Bits 4:7 - This field defines the number of items in the ADC conversion channel list."]
    #[inline(always)]
    pub fn bit_adc_cvlist_len(&mut self) -> BitAdcCvlistLenW<'_, RegAdcConfSpec> {
        BitAdcCvlistLenW::new(self, 4)
    }
    #[doc = "Bit 9 - This bit is for ADC enable control"]
    #[inline(always)]
    pub fn bit_adc_enable(&mut self) -> BitAdcEnableW<'_, RegAdcConfSpec> {
        BitAdcEnableW::new(self, 9)
    }
}
#[doc = "ADC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcConfSpec;
impl crate::RegisterSpec for RegAdcConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_conf::R`](R) reader structure"]
impl crate::Readable for RegAdcConfSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_conf::W`](W) writer structure"]
impl crate::Writable for RegAdcConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_CONF to value 0"]
impl crate::Resettable for RegAdcConfSpec {}
