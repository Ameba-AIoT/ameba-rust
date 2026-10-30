#[doc = "Register `DBG_GPIO_MAP_1` reader"]
pub type R = crate::R<DbgGpioMap1Spec>;
#[doc = "Register `DBG_GPIO_MAP_1` writer"]
pub type W = crate::W<DbgGpioMap1Spec>;
#[doc = "Field `gpio_dmux_sel4` reader - "]
pub type GpioDmuxSel4R = crate::FieldReader;
#[doc = "Field `gpio_dmux_sel4` writer - "]
pub type GpioDmuxSel4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpio_dmux_sel5` reader - "]
pub type GpioDmuxSel5R = crate::FieldReader;
#[doc = "Field `gpio_dmux_sel5` writer - "]
pub type GpioDmuxSel5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpio_dmux_sel6` reader - "]
pub type GpioDmuxSel6R = crate::FieldReader;
#[doc = "Field `gpio_dmux_sel6` writer - "]
pub type GpioDmuxSel6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpio_dmux_sel7` reader - "]
pub type GpioDmuxSel7R = crate::FieldReader;
#[doc = "Field `gpio_dmux_sel7` writer - "]
pub type GpioDmuxSel7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpio_dmux_sel4(&self) -> GpioDmuxSel4R {
        GpioDmuxSel4R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpio_dmux_sel5(&self) -> GpioDmuxSel5R {
        GpioDmuxSel5R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gpio_dmux_sel6(&self) -> GpioDmuxSel6R {
        GpioDmuxSel6R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn gpio_dmux_sel7(&self) -> GpioDmuxSel7R {
        GpioDmuxSel7R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpio_dmux_sel4(&mut self) -> GpioDmuxSel4W<'_, DbgGpioMap1Spec> {
        GpioDmuxSel4W::new(self, 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpio_dmux_sel5(&mut self) -> GpioDmuxSel5W<'_, DbgGpioMap1Spec> {
        GpioDmuxSel5W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gpio_dmux_sel6(&mut self) -> GpioDmuxSel6W<'_, DbgGpioMap1Spec> {
        GpioDmuxSel6W::new(self, 16)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn gpio_dmux_sel7(&mut self) -> GpioDmuxSel7W<'_, DbgGpioMap1Spec> {
        GpioDmuxSel7W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_gpio_map_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_gpio_map_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgGpioMap1Spec;
impl crate::RegisterSpec for DbgGpioMap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_gpio_map_1::R`](R) reader structure"]
impl crate::Readable for DbgGpioMap1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_gpio_map_1::W`](W) writer structure"]
impl crate::Writable for DbgGpioMap1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_GPIO_MAP_1 to value 0x0706_0504"]
impl crate::Resettable for DbgGpioMap1Spec {
    const RESET_VALUE: u32 = 0x0706_0504;
}
