#[doc = "Register `CTC_FILTER_CTRL0` reader"]
pub type R = crate::R<CtcFilterCtrl0Spec>;
#[doc = "Register `CTC_FILTER_CTRL0` writer"]
pub type W = crate::W<CtcFilterCtrl0Spec>;
#[doc = "Field `flt_en` reader - Enable ctc filter function(global),for raw data"]
pub type FltEnR = crate::BitReader;
#[doc = "Field `flt_en` writer - Enable ctc filter function(global),for raw data"]
pub type FltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iir_en` reader - Enable ctc iir function,for raw data"]
pub type IirEnR = crate::BitReader;
#[doc = "Field `iir_en` writer - Enable ctc iir function,for raw data"]
pub type IirEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `med_coef` reader - Median filter coefficient,for raw data 00:disable 01:coef is 3 10:coef is 5 11:coef is 7"]
pub type MedCoefR = crate::FieldReader;
#[doc = "Field `med_coef` writer - Median filter coefficient,for raw data 00:disable 01:coef is 3 10:coef is 5 11:coef is 7"]
pub type MedCoefW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `med1_en` reader - Enable ctc data ave median filter function, median filter coefficient is 3.When ctc_clk_lcnt 0 / 1 / 2 fills in different values and med1 is enable, the filtering effect of aliasing fixed frequency interference can be enhanced."]
pub type Med1EnR = crate::BitReader;
#[doc = "Field `med1_en` writer - Enable ctc data ave median filter function, median filter coefficient is 3.When ctc_clk_lcnt 0 / 1 / 2 fills in different values and med1 is enable, the filtering effect of aliasing fixed frequency interference can be enhanced."]
pub type Med1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `med_indiv_en` reader - enable re-filters when each scan period. default state is continuous filter."]
pub type MedIndivEnR = crate::BitReader;
#[doc = "Field `med_indiv_en` writer - enable re-filters when each scan period. default state is continuous filter."]
pub type MedIndivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mdec` reader - Decimation of sample point"]
pub type MdecR = crate::FieldReader<u16>;
#[doc = "Field `mdec` writer - Decimation of sample point"]
pub type MdecW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable ctc filter function(global),for raw data"]
    #[inline(always)]
    pub fn flt_en(&self) -> FltEnR {
        FltEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ctc iir function,for raw data"]
    #[inline(always)]
    pub fn iir_en(&self) -> IirEnR {
        IirEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Median filter coefficient,for raw data 00:disable 01:coef is 3 10:coef is 5 11:coef is 7"]
    #[inline(always)]
    pub fn med_coef(&self) -> MedCoefR {
        MedCoefR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable ctc data ave median filter function, median filter coefficient is 3.When ctc_clk_lcnt 0 / 1 / 2 fills in different values and med1 is enable, the filtering effect of aliasing fixed frequency interference can be enhanced."]
    #[inline(always)]
    pub fn med1_en(&self) -> Med1EnR {
        Med1EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable re-filters when each scan period. default state is continuous filter."]
    #[inline(always)]
    pub fn med_indiv_en(&self) -> MedIndivEnR {
        MedIndivEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Decimation of sample point"]
    #[inline(always)]
    pub fn mdec(&self) -> MdecR {
        MdecR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ctc filter function(global),for raw data"]
    #[inline(always)]
    pub fn flt_en(&mut self) -> FltEnW<'_, CtcFilterCtrl0Spec> {
        FltEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable ctc iir function,for raw data"]
    #[inline(always)]
    pub fn iir_en(&mut self) -> IirEnW<'_, CtcFilterCtrl0Spec> {
        IirEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Median filter coefficient,for raw data 00:disable 01:coef is 3 10:coef is 5 11:coef is 7"]
    #[inline(always)]
    pub fn med_coef(&mut self) -> MedCoefW<'_, CtcFilterCtrl0Spec> {
        MedCoefW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable ctc data ave median filter function, median filter coefficient is 3.When ctc_clk_lcnt 0 / 1 / 2 fills in different values and med1 is enable, the filtering effect of aliasing fixed frequency interference can be enhanced."]
    #[inline(always)]
    pub fn med1_en(&mut self) -> Med1EnW<'_, CtcFilterCtrl0Spec> {
        Med1EnW::new(self, 4)
    }
    #[doc = "Bit 5 - enable re-filters when each scan period. default state is continuous filter."]
    #[inline(always)]
    pub fn med_indiv_en(&mut self) -> MedIndivEnW<'_, CtcFilterCtrl0Spec> {
        MedIndivEnW::new(self, 5)
    }
    #[doc = "Bits 16:31 - Decimation of sample point"]
    #[inline(always)]
    pub fn mdec(&mut self) -> MdecW<'_, CtcFilterCtrl0Spec> {
        MdecW::new(self, 16)
    }
}
#[doc = "Captouch Filter Control0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_filter_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_filter_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcFilterCtrl0Spec;
impl crate::RegisterSpec for CtcFilterCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_filter_ctrl0::R`](R) reader structure"]
impl crate::Readable for CtcFilterCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctc_filter_ctrl0::W`](W) writer structure"]
impl crate::Writable for CtcFilterCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_FILTER_CTRL0 to value 0x000a_0004"]
impl crate::Resettable for CtcFilterCtrl0Spec {
    const RESET_VALUE: u32 = 0x000a_0004;
}
