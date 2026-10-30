#[doc = "Register `adc_0_control_0` reader"]
pub type R = crate::R<Adc0Control0Spec>;
#[doc = "Register `adc_0_control_0` writer"]
pub type W = crate::W<Adc0Control0Spec>;
#[doc = "Field `adc_0_dmic_src_sel` reader - Channel 0 DMIC source selection 3'b000: MIC 1 rising 3'b001: MIC 1 falling 3'b010: MIC 2 rising 3'b011: MIC 2 falling other: reserved"]
pub type Adc0DmicSrcSelR = crate::FieldReader;
#[doc = "Field `adc_0_dmic_src_sel` writer - Channel 0 DMIC source selection 3'b000: MIC 1 rising 3'b001: MIC 1 falling 3'b010: MIC 2 rising 3'b011: MIC 2 falling other: reserved"]
pub type Adc0DmicSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adc_0_dmic_lpf2nd_en` reader - Channel 0 DMIC path SRC 2nd LPF control 0: Disable 1: Enable"]
pub type Adc0DmicLpf2ndEnR = crate::BitReader;
#[doc = "Field `adc_0_dmic_lpf2nd_en` writer - Channel 0 DMIC path SRC 2nd LPF control 0: Disable 1: Enable"]
pub type Adc0DmicLpf2ndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_dmic_lpf1st_en` reader - Channel 0 DMIC path SRC 1st LPF control 0: Disable 1: Enable"]
pub type Adc0DmicLpf1stEnR = crate::BitReader;
#[doc = "Field `adc_0_dmic_lpf1st_en` writer - Channel 0 DMIC path SRC 1st LPF control 0: Disable 1: Enable"]
pub type Adc0DmicLpf1stEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_dmic_lpf1st_fc_sel` reader - Channel 0 DMIC path SRC 1st LPF FC"]
pub type Adc0DmicLpf1stFcSelR = crate::FieldReader;
#[doc = "Field `adc_0_dmic_lpf1st_fc_sel` writer - Channel 0 DMIC path SRC 1st LPF FC"]
pub type Adc0DmicLpf1stFcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_0_dmic_mix_mute` reader - Channel 0 DMIC input path mute 0: Un-Mute 1: Mute"]
pub type Adc0DmicMixMuteR = crate::BitReader;
#[doc = "Field `adc_0_dmic_mix_mute` writer - Channel 0 DMIC input path mute 0: Un-Mute 1: Mute"]
pub type Adc0DmicMixMuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_ad_zdet_func` reader - Channel 0 ADC path zero detection function selection 2'b00: immediate change 2'b01: zero detection & immediate change 2'b10: zero detection & step 2'b11: zero detection & timeout step"]
pub type Adc0AdZdetFuncR = crate::FieldReader;
#[doc = "Field `adc_0_ad_zdet_func` writer - Channel 0 ADC path zero detection function selection 2'b00: immediate change 2'b01: zero detection & immediate change 2'b10: zero detection & step 2'b11: zero detection & timeout step"]
pub type Adc0AdZdetFuncW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_0_ad_zdet_tout` reader - Channel 0 ADC path zero detection time out selection 2'b00: 1024*16 samples 2'b01: 1024*32 samples 2'b10: 1024*64 samples 2'b11: 64 samples"]
pub type Adc0AdZdetToutR = crate::FieldReader;
#[doc = "Field `adc_0_ad_zdet_tout` writer - Channel 0 ADC path zero detection time out selection 2'b00: 1024*16 samples 2'b01: 1024*32 samples 2'b10: 1024*64 samples 2'b11: 64 samples"]
pub type Adc0AdZdetToutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_0_ad_mute` reader - Channel 0 ADC path mute 0: Un-Mute 1: Mute"]
pub type Adc0AdMuteR = crate::BitReader;
#[doc = "Field `adc_0_ad_mute` writer - Channel 0 ADC path mute 0: Un-Mute 1: Mute"]
pub type Adc0AdMuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_hpf_rsvd` reader - Channel 0 ADC path reserved"]
pub type Adc0HpfRsvdR = crate::BitReader;
#[doc = "Field `adc_0_hpf_rsvd` writer - Channel 0 ADC path reserved"]
pub type Adc0HpfRsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_dchpf_en` reader - Channel 0 ADC path high pass filter enable control (filter DC) 0: Disable 1: Enable"]
pub type Adc0DchpfEnR = crate::BitReader;
#[doc = "Field `adc_0_dchpf_en` writer - Channel 0 ADC path high pass filter enable control (filter DC) 0: Disable 1: Enable"]
pub type Adc0DchpfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_dchpf_fc_sel` reader - Channel 0 ADC path high pass filter Fc"]
pub type Adc0DchpfFcSelR = crate::FieldReader;
#[doc = "Field `adc_0_dchpf_fc_sel` writer - Channel 0 ADC path high pass filter Fc"]
pub type Adc0DchpfFcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adc_0_dmic_lpf2nd_fc_sel` reader - Uplink Channel 0 DMIC path SRC 2nd LPF FC"]
pub type Adc0DmicLpf2ndFcSelR = crate::FieldReader;
#[doc = "Field `adc_0_dmic_lpf2nd_fc_sel` writer - Uplink Channel 0 DMIC path SRC 2nd LPF FC"]
pub type Adc0DmicLpf2ndFcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Channel 0 DMIC source selection 3'b000: MIC 1 rising 3'b001: MIC 1 falling 3'b010: MIC 2 rising 3'b011: MIC 2 falling other: reserved"]
    #[inline(always)]
    pub fn adc_0_dmic_src_sel(&self) -> Adc0DmicSrcSelR {
        Adc0DmicSrcSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Channel 0 DMIC path SRC 2nd LPF control 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf2nd_en(&self) -> Adc0DmicLpf2ndEnR {
        Adc0DmicLpf2ndEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 DMIC path SRC 1st LPF control 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf1st_en(&self) -> Adc0DmicLpf1stEnR {
        Adc0DmicLpf1stEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Channel 0 DMIC path SRC 1st LPF FC"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf1st_fc_sel(&self) -> Adc0DmicLpf1stFcSelR {
        Adc0DmicLpf1stFcSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Channel 0 DMIC input path mute 0: Un-Mute 1: Mute"]
    #[inline(always)]
    pub fn adc_0_dmic_mix_mute(&self) -> Adc0DmicMixMuteR {
        Adc0DmicMixMuteR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Channel 0 ADC path zero detection function selection 2'b00: immediate change 2'b01: zero detection & immediate change 2'b10: zero detection & step 2'b11: zero detection & timeout step"]
    #[inline(always)]
    pub fn adc_0_ad_zdet_func(&self) -> Adc0AdZdetFuncR {
        Adc0AdZdetFuncR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Channel 0 ADC path zero detection time out selection 2'b00: 1024*16 samples 2'b01: 1024*32 samples 2'b10: 1024*64 samples 2'b11: 64 samples"]
    #[inline(always)]
    pub fn adc_0_ad_zdet_tout(&self) -> Adc0AdZdetToutR {
        Adc0AdZdetToutR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Channel 0 ADC path mute 0: Un-Mute 1: Mute"]
    #[inline(always)]
    pub fn adc_0_ad_mute(&self) -> Adc0AdMuteR {
        Adc0AdMuteR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 0 ADC path reserved"]
    #[inline(always)]
    pub fn adc_0_hpf_rsvd(&self) -> Adc0HpfRsvdR {
        Adc0HpfRsvdR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 0 ADC path high pass filter enable control (filter DC) 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn adc_0_dchpf_en(&self) -> Adc0DchpfEnR {
        Adc0DchpfEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - Channel 0 ADC path high pass filter Fc"]
    #[inline(always)]
    pub fn adc_0_dchpf_fc_sel(&self) -> Adc0DchpfFcSelR {
        Adc0DchpfFcSelR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Uplink Channel 0 DMIC path SRC 2nd LPF FC"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf2nd_fc_sel(&self) -> Adc0DmicLpf2ndFcSelR {
        Adc0DmicLpf2ndFcSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 DMIC source selection 3'b000: MIC 1 rising 3'b001: MIC 1 falling 3'b010: MIC 2 rising 3'b011: MIC 2 falling other: reserved"]
    #[inline(always)]
    pub fn adc_0_dmic_src_sel(&mut self) -> Adc0DmicSrcSelW<'_, Adc0Control0Spec> {
        Adc0DmicSrcSelW::new(self, 0)
    }
    #[doc = "Bit 3 - Channel 0 DMIC path SRC 2nd LPF control 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf2nd_en(&mut self) -> Adc0DmicLpf2ndEnW<'_, Adc0Control0Spec> {
        Adc0DmicLpf2ndEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 0 DMIC path SRC 1st LPF control 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf1st_en(&mut self) -> Adc0DmicLpf1stEnW<'_, Adc0Control0Spec> {
        Adc0DmicLpf1stEnW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Channel 0 DMIC path SRC 1st LPF FC"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf1st_fc_sel(&mut self) -> Adc0DmicLpf1stFcSelW<'_, Adc0Control0Spec> {
        Adc0DmicLpf1stFcSelW::new(self, 5)
    }
    #[doc = "Bit 7 - Channel 0 DMIC input path mute 0: Un-Mute 1: Mute"]
    #[inline(always)]
    pub fn adc_0_dmic_mix_mute(&mut self) -> Adc0DmicMixMuteW<'_, Adc0Control0Spec> {
        Adc0DmicMixMuteW::new(self, 7)
    }
    #[doc = "Bits 18:19 - Channel 0 ADC path zero detection function selection 2'b00: immediate change 2'b01: zero detection & immediate change 2'b10: zero detection & step 2'b11: zero detection & timeout step"]
    #[inline(always)]
    pub fn adc_0_ad_zdet_func(&mut self) -> Adc0AdZdetFuncW<'_, Adc0Control0Spec> {
        Adc0AdZdetFuncW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Channel 0 ADC path zero detection time out selection 2'b00: 1024*16 samples 2'b01: 1024*32 samples 2'b10: 1024*64 samples 2'b11: 64 samples"]
    #[inline(always)]
    pub fn adc_0_ad_zdet_tout(&mut self) -> Adc0AdZdetToutW<'_, Adc0Control0Spec> {
        Adc0AdZdetToutW::new(self, 20)
    }
    #[doc = "Bit 22 - Channel 0 ADC path mute 0: Un-Mute 1: Mute"]
    #[inline(always)]
    pub fn adc_0_ad_mute(&mut self) -> Adc0AdMuteW<'_, Adc0Control0Spec> {
        Adc0AdMuteW::new(self, 22)
    }
    #[doc = "Bit 23 - Channel 0 ADC path reserved"]
    #[inline(always)]
    pub fn adc_0_hpf_rsvd(&mut self) -> Adc0HpfRsvdW<'_, Adc0Control0Spec> {
        Adc0HpfRsvdW::new(self, 23)
    }
    #[doc = "Bit 26 - Channel 0 ADC path high pass filter enable control (filter DC) 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn adc_0_dchpf_en(&mut self) -> Adc0DchpfEnW<'_, Adc0Control0Spec> {
        Adc0DchpfEnW::new(self, 26)
    }
    #[doc = "Bits 27:29 - Channel 0 ADC path high pass filter Fc"]
    #[inline(always)]
    pub fn adc_0_dchpf_fc_sel(&mut self) -> Adc0DchpfFcSelW<'_, Adc0Control0Spec> {
        Adc0DchpfFcSelW::new(self, 27)
    }
    #[doc = "Bits 30:31 - Uplink Channel 0 DMIC path SRC 2nd LPF FC"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf2nd_fc_sel(&mut self) -> Adc0DmicLpf2ndFcSelW<'_, Adc0Control0Spec> {
        Adc0DmicLpf2ndFcSelW::new(self, 30)
    }
}
#[doc = "ADC Channel 0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_0_control_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_0_control_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0Control0Spec;
impl crate::RegisterSpec for Adc0Control0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_0_control_0::R`](R) reader structure"]
impl crate::Readable for Adc0Control0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_0_control_0::W`](W) writer structure"]
impl crate::Writable for Adc0Control0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_0_control_0 to value 0x2008_0098"]
impl crate::Resettable for Adc0Control0Spec {
    const RESET_VALUE: u32 = 0x2008_0098;
}
