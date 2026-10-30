#[doc = "Register `adc_align_control_reg` reader"]
pub type R = crate::R<AdcAlignControlRegSpec>;
#[doc = "Register `adc_align_control_reg` writer"]
pub type W = crate::W<AdcAlignControlRegSpec>;
#[doc = "Field `adc_0_align_ch_sel` reader - Uplink CH0 align channel selection 0: CH0 1: CH1 Others: reserved"]
pub type Adc0AlignChSelR = crate::FieldReader;
#[doc = "Field `adc_0_align_ch_sel` writer - Uplink CH0 align channel selection 0: CH0 1: CH1 Others: reserved"]
pub type Adc0AlignChSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adc_0_align_en` reader - Uplink CH0 align enable"]
pub type Adc0AlignEnR = crate::BitReader;
#[doc = "Field `adc_0_align_en` writer - Uplink CH0 align enable"]
pub type Adc0AlignEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_align_ch_sel` reader - Uplink CH1 align channel selection 0: CH0 1: CH1 Others: reserved"]
pub type Adc1AlignChSelR = crate::FieldReader;
#[doc = "Field `adc_1_align_ch_sel` writer - Uplink CH1 align channel selection 0: CH0 1: CH1 Others: reserved"]
pub type Adc1AlignChSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adc_1_align_en` reader - Uplink CH1 align enable"]
pub type Adc1AlignEnR = crate::BitReader;
#[doc = "Field `adc_1_align_en` writer - Uplink CH1 align enable"]
pub type Adc1AlignEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Uplink CH0 align channel selection 0: CH0 1: CH1 Others: reserved"]
    #[inline(always)]
    pub fn adc_0_align_ch_sel(&self) -> Adc0AlignChSelR {
        Adc0AlignChSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Uplink CH0 align enable"]
    #[inline(always)]
    pub fn adc_0_align_en(&self) -> Adc0AlignEnR {
        Adc0AlignEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Uplink CH1 align channel selection 0: CH0 1: CH1 Others: reserved"]
    #[inline(always)]
    pub fn adc_1_align_ch_sel(&self) -> Adc1AlignChSelR {
        Adc1AlignChSelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Uplink CH1 align enable"]
    #[inline(always)]
    pub fn adc_1_align_en(&self) -> Adc1AlignEnR {
        Adc1AlignEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Uplink CH0 align channel selection 0: CH0 1: CH1 Others: reserved"]
    #[inline(always)]
    pub fn adc_0_align_ch_sel(&mut self) -> Adc0AlignChSelW<'_, AdcAlignControlRegSpec> {
        Adc0AlignChSelW::new(self, 0)
    }
    #[doc = "Bit 3 - Uplink CH0 align enable"]
    #[inline(always)]
    pub fn adc_0_align_en(&mut self) -> Adc0AlignEnW<'_, AdcAlignControlRegSpec> {
        Adc0AlignEnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Uplink CH1 align channel selection 0: CH0 1: CH1 Others: reserved"]
    #[inline(always)]
    pub fn adc_1_align_ch_sel(&mut self) -> Adc1AlignChSelW<'_, AdcAlignControlRegSpec> {
        Adc1AlignChSelW::new(self, 4)
    }
    #[doc = "Bit 7 - Uplink CH1 align enable"]
    #[inline(always)]
    pub fn adc_1_align_en(&mut self) -> Adc1AlignEnW<'_, AdcAlignControlRegSpec> {
        Adc1AlignEnW::new(self, 7)
    }
}
#[doc = "ADC Align Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_align_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_align_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcAlignControlRegSpec;
impl crate::RegisterSpec for AdcAlignControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_align_control_reg::R`](R) reader structure"]
impl crate::Readable for AdcAlignControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_align_control_reg::W`](W) writer structure"]
impl crate::Writable for AdcAlignControlRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_align_control_reg to value 0"]
impl crate::Resettable for AdcAlignControlRegSpec {}
