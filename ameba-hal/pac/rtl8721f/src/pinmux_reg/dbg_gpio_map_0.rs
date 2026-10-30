#[doc = "Register `DBG_GPIO_MAP_0` reader"]
pub type R = crate::R<DbgGpioMap0Spec>;
#[doc = "Register `DBG_GPIO_MAP_0` writer"]
pub type W = crate::W<DbgGpioMap0Spec>;
#[doc = "Field `gpio_dmux_sel0` reader - "]
pub type GpioDmuxSel0R = crate::FieldReader;
#[doc = "Field `gpio_dmux_sel0` writer - "]
pub type GpioDmuxSel0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpio_dmux_sel1` reader - "]
pub type GpioDmuxSel1R = crate::FieldReader;
#[doc = "Field `gpio_dmux_sel1` writer - "]
pub type GpioDmuxSel1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpio_dmux_sel2` reader - "]
pub type GpioDmuxSel2R = crate::FieldReader;
#[doc = "Field `gpio_dmux_sel2` writer - "]
pub type GpioDmuxSel2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpio_dmux_sel3` reader - "]
pub type GpioDmuxSel3R = crate::FieldReader;
#[doc = "Field `gpio_dmux_sel3` writer - "]
pub type GpioDmuxSel3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpio_dmux_sel0(&self) -> GpioDmuxSel0R {
        GpioDmuxSel0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpio_dmux_sel1(&self) -> GpioDmuxSel1R {
        GpioDmuxSel1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gpio_dmux_sel2(&self) -> GpioDmuxSel2R {
        GpioDmuxSel2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn gpio_dmux_sel3(&self) -> GpioDmuxSel3R {
        GpioDmuxSel3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpio_dmux_sel0(&mut self) -> GpioDmuxSel0W<'_, DbgGpioMap0Spec> {
        GpioDmuxSel0W::new(self, 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpio_dmux_sel1(&mut self) -> GpioDmuxSel1W<'_, DbgGpioMap0Spec> {
        GpioDmuxSel1W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gpio_dmux_sel2(&mut self) -> GpioDmuxSel2W<'_, DbgGpioMap0Spec> {
        GpioDmuxSel2W::new(self, 16)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn gpio_dmux_sel3(&mut self) -> GpioDmuxSel3W<'_, DbgGpioMap0Spec> {
        GpioDmuxSel3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_gpio_map_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_gpio_map_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgGpioMap0Spec;
impl crate::RegisterSpec for DbgGpioMap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_gpio_map_0::R`](R) reader structure"]
impl crate::Readable for DbgGpioMap0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_gpio_map_0::W`](W) writer structure"]
impl crate::Writable for DbgGpioMap0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_GPIO_MAP_0 to value 0x0302_0100"]
impl crate::Resettable for DbgGpioMap0Spec {
    const RESET_VALUE: u32 = 0x0302_0100;
}
