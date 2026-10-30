#[doc = "Register `REG_GPIO_CTRL` reader"]
pub type R = crate::R<RegGpioCtrlSpec>;
#[doc = "Register `REG_GPIO_CTRL` writer"]
pub type W = crate::W<RegGpioCtrlSpec>;
#[doc = "Field `GPIO_WAKE_FEN` reader - 1: Enable wake pin function 0: Disable Every GPIO has its own enable. When aon pmc is not in DSLP, wake pins will not effect PMC state althrougth this bit is set."]
pub type GpioWakeFenR = crate::FieldReader;
#[doc = "Field `GPIO_WAKE_FEN` writer - 1: Enable wake pin function 0: Disable Every GPIO has its own enable. When aon pmc is not in DSLP, wake pins will not effect PMC state althrougth this bit is set."]
pub type GpioWakeFenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPIO_DBNC_FEN` reader - 1: Enable gpio debouncer function 0: Disable Default disable for simualtion issue"]
pub type GpioDbncFenR = crate::BitReader;
#[doc = "Field `GPIO_DBNC_FEN` writer - 1: Enable gpio debouncer function 0: Disable Default disable for simualtion issue"]
pub type GpioDbncFenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_DBNC_CYC` reader - The GPIO signal will be filtered by the number of debounce cycles of specified in this register. (100k/128k)"]
pub type GpioDbncCycR = crate::FieldReader<u16>;
#[doc = "Field `GPIO_DBNC_CYC` writer - The GPIO signal will be filtered by the number of debounce cycles of specified in this register. (100k/128k)"]
pub type GpioDbncCycW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `GPIO_WAKDET_POLY` reader - Wake pin polarity 0: low active 1: high active bit1: wake pin1 polarity bit0: wake pin0 polarity Note: once a wake pin is set, the pads PU/PD will be force by this poly value, and soft config have no use."]
pub type GpioWakdetPolyR = crate::FieldReader;
#[doc = "Field `GPIO_WAKDET_POLY` writer - Wake pin polarity 0: low active 1: high active bit1: wake pin1 polarity bit0: wake pin0 polarity Note: once a wake pin is set, the pads PU/PD will be force by this poly value, and soft config have no use."]
pub type GpioWakdetPolyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1: Enable wake pin function 0: Disable Every GPIO has its own enable. When aon pmc is not in DSLP, wake pins will not effect PMC state althrougth this bit is set."]
    #[inline(always)]
    pub fn gpio_wake_fen(&self) -> GpioWakeFenR {
        GpioWakeFenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 1: Enable gpio debouncer function 0: Disable Default disable for simualtion issue"]
    #[inline(always)]
    pub fn gpio_dbnc_fen(&self) -> GpioDbncFenR {
        GpioDbncFenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:22 - The GPIO signal will be filtered by the number of debounce cycles of specified in this register. (100k/128k)"]
    #[inline(always)]
    pub fn gpio_dbnc_cyc(&self) -> GpioDbncCycR {
        GpioDbncCycR::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Wake pin polarity 0: low active 1: high active bit1: wake pin1 polarity bit0: wake pin0 polarity Note: once a wake pin is set, the pads PU/PD will be force by this poly value, and soft config have no use."]
    #[inline(always)]
    pub fn gpio_wakdet_poly(&self) -> GpioWakdetPolyR {
        GpioWakdetPolyR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1: Enable wake pin function 0: Disable Every GPIO has its own enable. When aon pmc is not in DSLP, wake pins will not effect PMC state althrougth this bit is set."]
    #[inline(always)]
    pub fn gpio_wake_fen(&mut self) -> GpioWakeFenW<'_, RegGpioCtrlSpec> {
        GpioWakeFenW::new(self, 0)
    }
    #[doc = "Bit 7 - 1: Enable gpio debouncer function 0: Disable Default disable for simualtion issue"]
    #[inline(always)]
    pub fn gpio_dbnc_fen(&mut self) -> GpioDbncFenW<'_, RegGpioCtrlSpec> {
        GpioDbncFenW::new(self, 7)
    }
    #[doc = "Bits 8:22 - The GPIO signal will be filtered by the number of debounce cycles of specified in this register. (100k/128k)"]
    #[inline(always)]
    pub fn gpio_dbnc_cyc(&mut self) -> GpioDbncCycW<'_, RegGpioCtrlSpec> {
        GpioDbncCycW::new(self, 8)
    }
    #[doc = "Bits 24:27 - Wake pin polarity 0: low active 1: high active bit1: wake pin1 polarity bit0: wake pin0 polarity Note: once a wake pin is set, the pads PU/PD will be force by this poly value, and soft config have no use."]
    #[inline(always)]
    pub fn gpio_wakdet_poly(&mut self) -> GpioWakdetPolyW<'_, RegGpioCtrlSpec> {
        GpioWakdetPolyW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gpio_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_gpio_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGpioCtrlSpec;
impl crate::RegisterSpec for RegGpioCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gpio_ctrl::R`](R) reader structure"]
impl crate::Readable for RegGpioCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_gpio_ctrl::W`](W) writer structure"]
impl crate::Writable for RegGpioCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GPIO_CTRL to value 0x0001_0000"]
impl crate::Resettable for RegGpioCtrlSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
