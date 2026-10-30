#[doc = "Register `REG_ADC_AUTO_CSW_CTRL` reader"]
pub type R = crate::R<RegAdcAutoCswCtrlSpec>;
#[doc = "Register `REG_ADC_AUTO_CSW_CTRL` writer"]
pub type W = crate::W<RegAdcAutoCswCtrlSpec>;
#[doc = "Field `bit_adc_autosw_en` reader - This bit controls the automatic channel switch enabled or disabled. 0: Disable the automatic channel switch.If an automatic channel switch is in process, writing 0 will terminate the automatic channel switch. 1: Enable the automatic channel switch. When setting this bit, an automatic channel switch starts from the first channel in the channel switch list."]
pub type BitAdcAutoswEnR = crate::BitReader;
#[doc = "Field `bit_adc_autosw_en` writer - This bit controls the automatic channel switch enabled or disabled. 0: Disable the automatic channel switch.If an automatic channel switch is in process, writing 0 will terminate the automatic channel switch. 1: Enable the automatic channel switch. When setting this bit, an automatic channel switch starts from the first channel in the channel switch list."]
pub type BitAdcAutoswEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit controls the automatic channel switch enabled or disabled. 0: Disable the automatic channel switch.If an automatic channel switch is in process, writing 0 will terminate the automatic channel switch. 1: Enable the automatic channel switch. When setting this bit, an automatic channel switch starts from the first channel in the channel switch list."]
    #[inline(always)]
    pub fn bit_adc_autosw_en(&self) -> BitAdcAutoswEnR {
        BitAdcAutoswEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls the automatic channel switch enabled or disabled. 0: Disable the automatic channel switch.If an automatic channel switch is in process, writing 0 will terminate the automatic channel switch. 1: Enable the automatic channel switch. When setting this bit, an automatic channel switch starts from the first channel in the channel switch list."]
    #[inline(always)]
    pub fn bit_adc_autosw_en(&mut self) -> BitAdcAutoswEnW<'_, RegAdcAutoCswCtrlSpec> {
        BitAdcAutoswEnW::new(self, 0)
    }
}
#[doc = "ADC Automatic Channel Switch Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_auto_csw_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_auto_csw_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcAutoCswCtrlSpec;
impl crate::RegisterSpec for RegAdcAutoCswCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_auto_csw_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAdcAutoCswCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_auto_csw_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAdcAutoCswCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_AUTO_CSW_CTRL to value 0"]
impl crate::Resettable for RegAdcAutoCswCtrlSpec {}
