#[doc = "Register `REG_DW_GPIO_CTRL` reader"]
pub type R = crate::R<RegDwGpioCtrlSpec>;
#[doc = "Register `REG_DW_GPIO_CTRL` writer"]
pub type W = crate::W<RegDwGpioCtrlSpec>;
#[doc = "Field `gpio_divcnt_pa` reader - divider for dw gpioa debounce clk"]
pub type GpioDivcntPaR = crate::FieldReader;
#[doc = "Field `gpio_divcnt_pa` writer - divider for dw gpioa debounce clk"]
pub type GpioDivcntPaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpio_divcnt_pb` reader - divider for dw gpiob debounce clk"]
pub type GpioDivcntPbR = crate::FieldReader;
#[doc = "Field `gpio_divcnt_pb` writer - divider for dw gpiob debounce clk"]
pub type GpioDivcntPbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpio_divcnt_pc` reader - divider for dw gpioc debounce clk"]
pub type GpioDivcntPcR = crate::FieldReader;
#[doc = "Field `gpio_divcnt_pc` writer - divider for dw gpioc debounce clk"]
pub type GpioDivcntPcW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpio_reg_opt` reader - gpio data out/oe register function option 0: DR/DDR reg function are same as DW GPIO spec 1: DR/DDR reg func changes , 0:not toggle , 1:toggle current value"]
pub type GpioRegOptR = crate::BitReader;
#[doc = "Field `gpio_reg_opt` writer - gpio data out/oe register function option 0: DR/DDR reg function are same as DW GPIO spec 1: DR/DDR reg func changes , 0:not toggle , 1:toggle current value"]
pub type GpioRegOptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - divider for dw gpioa debounce clk"]
    #[inline(always)]
    pub fn gpio_divcnt_pa(&self) -> GpioDivcntPaR {
        GpioDivcntPaR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - divider for dw gpiob debounce clk"]
    #[inline(always)]
    pub fn gpio_divcnt_pb(&self) -> GpioDivcntPbR {
        GpioDivcntPbR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - divider for dw gpioc debounce clk"]
    #[inline(always)]
    pub fn gpio_divcnt_pc(&self) -> GpioDivcntPcR {
        GpioDivcntPcR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - gpio data out/oe register function option 0: DR/DDR reg function are same as DW GPIO spec 1: DR/DDR reg func changes , 0:not toggle , 1:toggle current value"]
    #[inline(always)]
    pub fn gpio_reg_opt(&self) -> GpioRegOptR {
        GpioRegOptR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - divider for dw gpioa debounce clk"]
    #[inline(always)]
    pub fn gpio_divcnt_pa(&mut self) -> GpioDivcntPaW<'_, RegDwGpioCtrlSpec> {
        GpioDivcntPaW::new(self, 0)
    }
    #[doc = "Bits 8:14 - divider for dw gpiob debounce clk"]
    #[inline(always)]
    pub fn gpio_divcnt_pb(&mut self) -> GpioDivcntPbW<'_, RegDwGpioCtrlSpec> {
        GpioDivcntPbW::new(self, 8)
    }
    #[doc = "Bits 16:22 - divider for dw gpioc debounce clk"]
    #[inline(always)]
    pub fn gpio_divcnt_pc(&mut self) -> GpioDivcntPcW<'_, RegDwGpioCtrlSpec> {
        GpioDivcntPcW::new(self, 16)
    }
    #[doc = "Bit 31 - gpio data out/oe register function option 0: DR/DDR reg function are same as DW GPIO spec 1: DR/DDR reg func changes , 0:not toggle , 1:toggle current value"]
    #[inline(always)]
    pub fn gpio_reg_opt(&mut self) -> GpioRegOptW<'_, RegDwGpioCtrlSpec> {
        GpioRegOptW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dw_gpio_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dw_gpio_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDwGpioCtrlSpec;
impl crate::RegisterSpec for RegDwGpioCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dw_gpio_ctrl::R`](R) reader structure"]
impl crate::Readable for RegDwGpioCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dw_gpio_ctrl::W`](W) writer structure"]
impl crate::Writable for RegDwGpioCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DW_GPIO_CTRL to value 0x8000_0000"]
impl crate::Resettable for RegDwGpioCtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
