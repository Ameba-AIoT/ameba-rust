#[doc = "Register `LED_CTRL` reader"]
pub type R = crate::R<LedCtrlSpec>;
#[doc = "Register `LED_CTRL` writer"]
pub type W = crate::W<LedCtrlSpec>;
#[doc = "Field `Ledsel0` reader - Custom led0 select."]
pub type Ledsel0R = crate::FieldReader;
#[doc = "Field `Ledsel0` writer - Custom led0 select."]
pub type Ledsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Ledsel1` reader - Custom led1 select."]
pub type Ledsel1R = crate::FieldReader;
#[doc = "Field `Ledsel1` writer - Custom led1 select."]
pub type Ledsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Ledsel2` reader - Custom led2 select."]
pub type Ledsel2R = crate::FieldReader;
#[doc = "Field `Ledsel2` writer - Custom led2 select."]
pub type Ledsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Ledsel3` reader - Custom led3 select."]
pub type Ledsel3R = crate::FieldReader;
#[doc = "Field `Ledsel3` writer - Custom led3 select."]
pub type Ledsel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ledsel` reader - Non custom led select. Following EEE spec, combine the Link signal and LPI signal in the same LED pin. While entering LPI state, LED will blink with 2784ms cycle, of which the first 2349ms LED light is turned off, after 435ms is turned on. NO EEE in RL6166."]
pub type LedselR = crate::FieldReader;
#[doc = "Field `ledsel` writer - Non custom led select. Following EEE spec, combine the Link signal and LPI signal in the same LED pin. While entering LPI state, LED will blink with 2784ms cycle, of which the first 2349ms LED light is turned off, after 435ms is turned on. NO EEE in RL6166."]
pub type LedselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Custom_led` reader - Custom led mode. 1: using custom led matrix table. 0: using Non-custom led table."]
pub type CustomLedR = crate::BitReader;
#[doc = "Field `Custom_led` writer - Custom led mode. 1: using custom led matrix table. 0: using Non-custom led table."]
pub type CustomLedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eee_en_led` reader - Enable/disable lpi led."]
pub type EeeEnLedR = crate::BitReader;
#[doc = "Field `eee_en_led` writer - Enable/disable lpi led."]
pub type EeeEnLedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Custom led0 select."]
    #[inline(always)]
    pub fn ledsel0(&self) -> Ledsel0R {
        Ledsel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Custom led1 select."]
    #[inline(always)]
    pub fn ledsel1(&self) -> Ledsel1R {
        Ledsel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Custom led2 select."]
    #[inline(always)]
    pub fn ledsel2(&self) -> Ledsel2R {
        Ledsel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Custom led3 select."]
    #[inline(always)]
    pub fn ledsel3(&self) -> Ledsel3R {
        Ledsel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Non custom led select. Following EEE spec, combine the Link signal and LPI signal in the same LED pin. While entering LPI state, LED will blink with 2784ms cycle, of which the first 2349ms LED light is turned off, after 435ms is turned on. NO EEE in RL6166."]
    #[inline(always)]
    pub fn ledsel(&self) -> LedselR {
        LedselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Custom led mode. 1: using custom led matrix table. 0: using Non-custom led table."]
    #[inline(always)]
    pub fn custom_led(&self) -> CustomLedR {
        CustomLedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable/disable lpi led."]
    #[inline(always)]
    pub fn eee_en_led(&self) -> EeeEnLedR {
        EeeEnLedR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Custom led0 select."]
    #[inline(always)]
    pub fn ledsel0(&mut self) -> Ledsel0W<'_, LedCtrlSpec> {
        Ledsel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Custom led1 select."]
    #[inline(always)]
    pub fn ledsel1(&mut self) -> Ledsel1W<'_, LedCtrlSpec> {
        Ledsel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Custom led2 select."]
    #[inline(always)]
    pub fn ledsel2(&mut self) -> Ledsel2W<'_, LedCtrlSpec> {
        Ledsel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Custom led3 select."]
    #[inline(always)]
    pub fn ledsel3(&mut self) -> Ledsel3W<'_, LedCtrlSpec> {
        Ledsel3W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Non custom led select. Following EEE spec, combine the Link signal and LPI signal in the same LED pin. While entering LPI state, LED will blink with 2784ms cycle, of which the first 2349ms LED light is turned off, after 435ms is turned on. NO EEE in RL6166."]
    #[inline(always)]
    pub fn ledsel(&mut self) -> LedselW<'_, LedCtrlSpec> {
        LedselW::new(self, 16)
    }
    #[doc = "Bit 18 - Custom led mode. 1: using custom led matrix table. 0: using Non-custom led table."]
    #[inline(always)]
    pub fn custom_led(&mut self) -> CustomLedW<'_, LedCtrlSpec> {
        CustomLedW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable/disable lpi led."]
    #[inline(always)]
    pub fn eee_en_led(&mut self) -> EeeEnLedW<'_, LedCtrlSpec> {
        EeeEnLedW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`led_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`led_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LedCtrlSpec;
impl crate::RegisterSpec for LedCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`led_ctrl::R`](R) reader structure"]
impl crate::Readable for LedCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`led_ctrl::W`](W) writer structure"]
impl crate::Writable for LedCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LED_CTRL to value 0"]
impl crate::Resettable for LedCtrlSpec {}
