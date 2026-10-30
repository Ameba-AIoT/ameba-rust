#[doc = "Register `REG_ADC_COMP_CTRL` reader"]
pub type R = crate::R<RegAdcCompCtrlSpec>;
#[doc = "Register `REG_ADC_COMP_CTRL` writer"]
pub type W = crate::W<RegAdcCompCtrlSpec>;
#[doc = "Field `bit_adc_comp_ctrl_ch0` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh0R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch0` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch1` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh1R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch1` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch2` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh2R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch2` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch3` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh3R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch3` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch4` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh4R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch4` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch5` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh5R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch5` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch6` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh6R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch6` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch7` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh7R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch7` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch8` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh8R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch8` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch9` reader - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh9R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch9` writer - the function is same as bit_adc_comp_ctrl_ch10"]
pub type BitAdcCompCtrlCh9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_comp_ctrl_ch10` reader - This field defines ADC channel comparison criteria which would notify system by interrupt when the criterion matches. Once a criterion matches, ADC should send an interrupt signal to system if the related interrupt mask is unmask. ADC also updates comparison results in reg_adc_comp_sts. 0: When Vin < bit_adc_comp_th_l_chx, comparison criterion matches. 1: When Vin > bit_adc_comp_th_h_chx, comparison criterion matches. 2: When Vin >= bit_adc_comp_th_l_chx and Vin <= bit_adc_comp_th_h_chx, comparison criterion matches. 2: When Vin < bit_adc_comp_th_l_chx or Vin > bit_adc_comp_th_h_chx, comparison criterion matches."]
pub type BitAdcCompCtrlCh10R = crate::FieldReader;
#[doc = "Field `bit_adc_comp_ctrl_ch10` writer - This field defines ADC channel comparison criteria which would notify system by interrupt when the criterion matches. Once a criterion matches, ADC should send an interrupt signal to system if the related interrupt mask is unmask. ADC also updates comparison results in reg_adc_comp_sts. 0: When Vin < bit_adc_comp_th_l_chx, comparison criterion matches. 1: When Vin > bit_adc_comp_th_h_chx, comparison criterion matches. 2: When Vin >= bit_adc_comp_th_l_chx and Vin <= bit_adc_comp_th_h_chx, comparison criterion matches. 2: When Vin < bit_adc_comp_th_l_chx or Vin > bit_adc_comp_th_h_chx, comparison criterion matches."]
pub type BitAdcCompCtrlCh10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch0(&self) -> BitAdcCompCtrlCh0R {
        BitAdcCompCtrlCh0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch1(&self) -> BitAdcCompCtrlCh1R {
        BitAdcCompCtrlCh1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch2(&self) -> BitAdcCompCtrlCh2R {
        BitAdcCompCtrlCh2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch3(&self) -> BitAdcCompCtrlCh3R {
        BitAdcCompCtrlCh3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch4(&self) -> BitAdcCompCtrlCh4R {
        BitAdcCompCtrlCh4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch5(&self) -> BitAdcCompCtrlCh5R {
        BitAdcCompCtrlCh5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch6(&self) -> BitAdcCompCtrlCh6R {
        BitAdcCompCtrlCh6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch7(&self) -> BitAdcCompCtrlCh7R {
        BitAdcCompCtrlCh7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch8(&self) -> BitAdcCompCtrlCh8R {
        BitAdcCompCtrlCh8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch9(&self) -> BitAdcCompCtrlCh9R {
        BitAdcCompCtrlCh9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - This field defines ADC channel comparison criteria which would notify system by interrupt when the criterion matches. Once a criterion matches, ADC should send an interrupt signal to system if the related interrupt mask is unmask. ADC also updates comparison results in reg_adc_comp_sts. 0: When Vin < bit_adc_comp_th_l_chx, comparison criterion matches. 1: When Vin > bit_adc_comp_th_h_chx, comparison criterion matches. 2: When Vin >= bit_adc_comp_th_l_chx and Vin <= bit_adc_comp_th_h_chx, comparison criterion matches. 2: When Vin < bit_adc_comp_th_l_chx or Vin > bit_adc_comp_th_h_chx, comparison criterion matches."]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch10(&self) -> BitAdcCompCtrlCh10R {
        BitAdcCompCtrlCh10R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch0(&mut self) -> BitAdcCompCtrlCh0W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch1(&mut self) -> BitAdcCompCtrlCh1W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch2(&mut self) -> BitAdcCompCtrlCh2W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch3(&mut self) -> BitAdcCompCtrlCh3W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch4(&mut self) -> BitAdcCompCtrlCh4W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch5(&mut self) -> BitAdcCompCtrlCh5W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch6(&mut self) -> BitAdcCompCtrlCh6W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch7(&mut self) -> BitAdcCompCtrlCh7W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch8(&mut self) -> BitAdcCompCtrlCh8W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - the function is same as bit_adc_comp_ctrl_ch10"]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch9(&mut self) -> BitAdcCompCtrlCh9W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - This field defines ADC channel comparison criteria which would notify system by interrupt when the criterion matches. Once a criterion matches, ADC should send an interrupt signal to system if the related interrupt mask is unmask. ADC also updates comparison results in reg_adc_comp_sts. 0: When Vin < bit_adc_comp_th_l_chx, comparison criterion matches. 1: When Vin > bit_adc_comp_th_h_chx, comparison criterion matches. 2: When Vin >= bit_adc_comp_th_l_chx and Vin <= bit_adc_comp_th_h_chx, comparison criterion matches. 2: When Vin < bit_adc_comp_th_l_chx or Vin > bit_adc_comp_th_h_chx, comparison criterion matches."]
    #[inline(always)]
    pub fn bit_adc_comp_ctrl_ch10(&mut self) -> BitAdcCompCtrlCh10W<'_, RegAdcCompCtrlSpec> {
        BitAdcCompCtrlCh10W::new(self, 20)
    }
}
#[doc = "ADC Comparison Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompCtrlSpec;
impl crate::RegisterSpec for RegAdcCompCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAdcCompCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAdcCompCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_CTRL to value 0"]
impl crate::Resettable for RegAdcCompCtrlSpec {}
