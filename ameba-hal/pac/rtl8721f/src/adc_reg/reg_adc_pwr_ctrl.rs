#[doc = "Register `REG_ADC_PWR_CTRL` reader"]
pub type R = crate::R<RegAdcPwrCtrlSpec>;
#[doc = "Register `REG_ADC_PWR_CTRL` writer"]
pub type W = crate::W<RegAdcPwrCtrlSpec>;
#[doc = "Field `bit_adc_pwr_ctrl` reader - This field is used to power control"]
pub type BitAdcPwrCtrlR = crate::BitReader;
#[doc = "Field `bit_adc_pwr_ctrl` writer - This field is used to power control"]
pub type BitAdcPwrCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_dbg_sel` reader - Debug port select"]
pub type BitAdcDbgSelR = crate::FieldReader;
#[doc = "Field `bit_adc_dbg_sel` writer - Debug port select"]
pub type BitAdcDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bit_adc_dbg_en` reader - Debug port enable"]
pub type BitAdcDbgEnR = crate::BitReader;
#[doc = "Field `bit_adc_dbg_en` writer - Debug port enable"]
pub type BitAdcDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_pwr_keep` reader - keep adc power on at all times. automatic adjustment resumes after shutdown. 0: disable 1: enable"]
pub type BitAdcPwrKeepR = crate::BitReader;
#[doc = "Field `bit_adc_pwr_keep` writer - keep adc power on at all times. automatic adjustment resumes after shutdown. 0: disable 1: enable"]
pub type BitAdcPwrKeepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field is used to power control"]
    #[inline(always)]
    pub fn bit_adc_pwr_ctrl(&self) -> BitAdcPwrCtrlR {
        BitAdcPwrCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Debug port select"]
    #[inline(always)]
    pub fn bit_adc_dbg_sel(&self) -> BitAdcDbgSelR {
        BitAdcDbgSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Debug port enable"]
    #[inline(always)]
    pub fn bit_adc_dbg_en(&self) -> BitAdcDbgEnR {
        BitAdcDbgEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - keep adc power on at all times. automatic adjustment resumes after shutdown. 0: disable 1: enable"]
    #[inline(always)]
    pub fn bit_adc_pwr_keep(&self) -> BitAdcPwrKeepR {
        BitAdcPwrKeepR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to power control"]
    #[inline(always)]
    pub fn bit_adc_pwr_ctrl(&mut self) -> BitAdcPwrCtrlW<'_, RegAdcPwrCtrlSpec> {
        BitAdcPwrCtrlW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Debug port select"]
    #[inline(always)]
    pub fn bit_adc_dbg_sel(&mut self) -> BitAdcDbgSelW<'_, RegAdcPwrCtrlSpec> {
        BitAdcDbgSelW::new(self, 1)
    }
    #[doc = "Bit 3 - Debug port enable"]
    #[inline(always)]
    pub fn bit_adc_dbg_en(&mut self) -> BitAdcDbgEnW<'_, RegAdcPwrCtrlSpec> {
        BitAdcDbgEnW::new(self, 3)
    }
    #[doc = "Bit 4 - keep adc power on at all times. automatic adjustment resumes after shutdown. 0: disable 1: enable"]
    #[inline(always)]
    pub fn bit_adc_pwr_keep(&mut self) -> BitAdcPwrKeepW<'_, RegAdcPwrCtrlSpec> {
        BitAdcPwrKeepW::new(self, 4)
    }
}
#[doc = "ADC Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_pwr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_pwr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcPwrCtrlSpec;
impl crate::RegisterSpec for RegAdcPwrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_pwr_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAdcPwrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_pwr_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAdcPwrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_PWR_CTRL to value 0"]
impl crate::Resettable for RegAdcPwrCtrlSpec {}
