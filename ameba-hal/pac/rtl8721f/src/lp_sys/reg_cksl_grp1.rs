#[doc = "Register `REG_CKSL_GRP1` reader"]
pub type R = crate::R<RegCkslGrp1Spec>;
#[doc = "Register `REG_CKSL_GRP1` writer"]
pub type W = crate::W<RegCkslGrp1Spec>;
#[doc = "Field `cksl_gpio4m` reader - gpio ip using 4m clk selection ##0: sdm32k ##1: osc4m"]
pub type CkslGpio4mR = crate::BitReader;
#[doc = "Field `cksl_gpio4m` writer - gpio ip using 4m clk selection ##0: sdm32k ##1: osc4m"]
pub type CkslGpio4mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_gpio` reader - GPIO INT clock selecton: ##0: bus_clk ##1: ip_clk"]
pub type CkslGpioR = crate::BitReader;
#[doc = "Field `cksl_gpio` writer - GPIO INT clock selecton: ##0: bus_clk ##1: ip_clk"]
pub type CkslGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_adc` reader - ADC INT clock selection ##0: bus_clk ##1: ip_clk"]
pub type CkslAdcR = crate::BitReader;
#[doc = "Field `cksl_adc` writer - ADC INT clock selection ##0: bus_clk ##1: ip_clk"]
pub type CkslAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_dtim` reader - dtim clock selection ##0: sdm32k ##1: xtal Note:when select to xtal, it will divided to 1M fixed"]
pub type CkslDtimR = crate::BitReader;
#[doc = "Field `cksl_dtim` writer - dtim clock selection ##0: sdm32k ##1: xtal Note:when select to xtal, it will divided to 1M fixed"]
pub type CkslDtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_adc4m` reader - lp adc 4m clk selection ##0: osc4m ##1: xtal Note:when select xtal , it must set divider"]
pub type CkslAdc4mR = crate::BitReader;
#[doc = "Field `cksl_adc4m` writer - lp adc 4m clk selection ##0: osc4m ##1: xtal Note:when select xtal , it must set divider"]
pub type CkslAdc4mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_uart0` reader - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
pub type CkslUart0R = crate::BitReader;
#[doc = "Field `cksl_uart0` writer - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
pub type CkslUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_uart1` reader - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
pub type CkslUart1R = crate::BitReader;
#[doc = "Field `cksl_uart1` writer - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
pub type CkslUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_uart2` reader - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
pub type CkslUart2R = crate::BitReader;
#[doc = "Field `cksl_uart2` writer - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
pub type CkslUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_uart3` reader - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
pub type CkslUart3R = crate::BitReader;
#[doc = "Field `cksl_uart3` writer - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
pub type CkslUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_loguart` reader - Select loguart's input clock ##0: osc4m ##1: xtal"]
pub type CkslLoguartR = crate::BitReader;
#[doc = "Field `cksl_loguart` writer - Select loguart's input clock ##0: osc4m ##1: xtal"]
pub type CkslLoguartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_sys_32k` reader - system/wifi 32k clk source selection ##0: osc131k ##1: xtal312p5k"]
pub type CkslSys32kR = crate::BitReader;
#[doc = "Field `cksl_sys_32k` writer - system/wifi 32k clk source selection ##0: osc131k ##1: xtal312p5k"]
pub type CkslSys32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_bt_32k` reader - BT 32k clk source selection ##0: osc131k ##1: xtal312p5k"]
pub type CkslBt32kR = crate::BitReader;
#[doc = "Field `cksl_bt_32k` writer - BT 32k clk source selection ##0: osc131k ##1: xtal312p5k"]
pub type CkslBt32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_ltim0` reader - LS timer 0 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
pub type CkslLtim0R = crate::BitReader;
#[doc = "Field `cksl_ltim0` writer - LS timer 0 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
pub type CkslLtim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_ltim1` reader - LS timer 1 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
pub type CkslLtim1R = crate::BitReader;
#[doc = "Field `cksl_ltim1` writer - LS timer 1 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
pub type CkslLtim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_ltim2` reader - LS timer 2 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
pub type CkslLtim2R = crate::BitReader;
#[doc = "Field `cksl_ltim2` writer - LS timer 2 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
pub type CkslLtim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cksl_ltim3` reader - LS timer 3 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
pub type CkslLtim3R = crate::BitReader;
#[doc = "Field `cksl_ltim3` writer - LS timer 3 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
pub type CkslLtim3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - gpio ip using 4m clk selection ##0: sdm32k ##1: osc4m"]
    #[inline(always)]
    pub fn cksl_gpio4m(&self) -> CkslGpio4mR {
        CkslGpio4mR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO INT clock selecton: ##0: bus_clk ##1: ip_clk"]
    #[inline(always)]
    pub fn cksl_gpio(&self) -> CkslGpioR {
        CkslGpioR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC INT clock selection ##0: bus_clk ##1: ip_clk"]
    #[inline(always)]
    pub fn cksl_adc(&self) -> CkslAdcR {
        CkslAdcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - dtim clock selection ##0: sdm32k ##1: xtal Note:when select to xtal, it will divided to 1M fixed"]
    #[inline(always)]
    pub fn cksl_dtim(&self) -> CkslDtimR {
        CkslDtimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - lp adc 4m clk selection ##0: osc4m ##1: xtal Note:when select xtal , it must set divider"]
    #[inline(always)]
    pub fn cksl_adc4m(&self) -> CkslAdc4mR {
        CkslAdc4mR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
    #[inline(always)]
    pub fn cksl_uart0(&self) -> CkslUart0R {
        CkslUart0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
    #[inline(always)]
    pub fn cksl_uart1(&self) -> CkslUart1R {
        CkslUart1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
    #[inline(always)]
    pub fn cksl_uart2(&self) -> CkslUart2R {
        CkslUart2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
    #[inline(always)]
    pub fn cksl_uart3(&self) -> CkslUart3R {
        CkslUart3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select loguart's input clock ##0: osc4m ##1: xtal"]
    #[inline(always)]
    pub fn cksl_loguart(&self) -> CkslLoguartR {
        CkslLoguartR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - system/wifi 32k clk source selection ##0: osc131k ##1: xtal312p5k"]
    #[inline(always)]
    pub fn cksl_sys_32k(&self) -> CkslSys32kR {
        CkslSys32kR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BT 32k clk source selection ##0: osc131k ##1: xtal312p5k"]
    #[inline(always)]
    pub fn cksl_bt_32k(&self) -> CkslBt32kR {
        CkslBt32kR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - LS timer 0 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
    #[inline(always)]
    pub fn cksl_ltim0(&self) -> CkslLtim0R {
        CkslLtim0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LS timer 1 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
    #[inline(always)]
    pub fn cksl_ltim1(&self) -> CkslLtim1R {
        CkslLtim1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LS timer 2 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
    #[inline(always)]
    pub fn cksl_ltim2(&self) -> CkslLtim2R {
        CkslLtim2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LS timer 3 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
    #[inline(always)]
    pub fn cksl_ltim3(&self) -> CkslLtim3R {
        CkslLtim3R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - gpio ip using 4m clk selection ##0: sdm32k ##1: osc4m"]
    #[inline(always)]
    pub fn cksl_gpio4m(&mut self) -> CkslGpio4mW<'_, RegCkslGrp1Spec> {
        CkslGpio4mW::new(self, 3)
    }
    #[doc = "Bit 5 - GPIO INT clock selecton: ##0: bus_clk ##1: ip_clk"]
    #[inline(always)]
    pub fn cksl_gpio(&mut self) -> CkslGpioW<'_, RegCkslGrp1Spec> {
        CkslGpioW::new(self, 5)
    }
    #[doc = "Bit 6 - ADC INT clock selection ##0: bus_clk ##1: ip_clk"]
    #[inline(always)]
    pub fn cksl_adc(&mut self) -> CkslAdcW<'_, RegCkslGrp1Spec> {
        CkslAdcW::new(self, 6)
    }
    #[doc = "Bit 8 - dtim clock selection ##0: sdm32k ##1: xtal Note:when select to xtal, it will divided to 1M fixed"]
    #[inline(always)]
    pub fn cksl_dtim(&mut self) -> CkslDtimW<'_, RegCkslGrp1Spec> {
        CkslDtimW::new(self, 8)
    }
    #[doc = "Bit 9 - lp adc 4m clk selection ##0: osc4m ##1: xtal Note:when select xtal , it must set divider"]
    #[inline(always)]
    pub fn cksl_adc4m(&mut self) -> CkslAdc4mW<'_, RegCkslGrp1Spec> {
        CkslAdc4mW::new(self, 9)
    }
    #[doc = "Bit 10 - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
    #[inline(always)]
    pub fn cksl_uart0(&mut self) -> CkslUart0W<'_, RegCkslGrp1Spec> {
        CkslUart0W::new(self, 10)
    }
    #[doc = "Bit 11 - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
    #[inline(always)]
    pub fn cksl_uart1(&mut self) -> CkslUart1W<'_, RegCkslGrp1Spec> {
        CkslUart1W::new(self, 11)
    }
    #[doc = "Bit 12 - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
    #[inline(always)]
    pub fn cksl_uart2(&mut self) -> CkslUart2W<'_, RegCkslGrp1Spec> {
        CkslUart2W::new(self, 12)
    }
    #[doc = "Bit 13 - Select uart's input rx clock ##0: osc2m ##1: xtal Note:tx input clock is always xtal , and divided in uart IP"]
    #[inline(always)]
    pub fn cksl_uart3(&mut self) -> CkslUart3W<'_, RegCkslGrp1Spec> {
        CkslUart3W::new(self, 13)
    }
    #[doc = "Bit 14 - Select loguart's input clock ##0: osc4m ##1: xtal"]
    #[inline(always)]
    pub fn cksl_loguart(&mut self) -> CkslLoguartW<'_, RegCkslGrp1Spec> {
        CkslLoguartW::new(self, 14)
    }
    #[doc = "Bit 16 - system/wifi 32k clk source selection ##0: osc131k ##1: xtal312p5k"]
    #[inline(always)]
    pub fn cksl_sys_32k(&mut self) -> CkslSys32kW<'_, RegCkslGrp1Spec> {
        CkslSys32kW::new(self, 16)
    }
    #[doc = "Bit 17 - BT 32k clk source selection ##0: osc131k ##1: xtal312p5k"]
    #[inline(always)]
    pub fn cksl_bt_32k(&mut self) -> CkslBt32kW<'_, RegCkslGrp1Spec> {
        CkslBt32kW::new(self, 17)
    }
    #[doc = "Bit 20 - LS timer 0 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
    #[inline(always)]
    pub fn cksl_ltim0(&mut self) -> CkslLtim0W<'_, RegCkslGrp1Spec> {
        CkslLtim0W::new(self, 20)
    }
    #[doc = "Bit 21 - LS timer 1 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
    #[inline(always)]
    pub fn cksl_ltim1(&mut self) -> CkslLtim1W<'_, RegCkslGrp1Spec> {
        CkslLtim1W::new(self, 21)
    }
    #[doc = "Bit 22 - LS timer 2 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
    #[inline(always)]
    pub fn cksl_ltim2(&mut self) -> CkslLtim2W<'_, RegCkslGrp1Spec> {
        CkslLtim2W::new(self, 22)
    }
    #[doc = "Bit 23 - LS timer 3 clock source select ##0: sdm32k ##1: xtal Note:using XTAL divider out , which can be 40M/N (N=2~63)"]
    #[inline(always)]
    pub fn cksl_ltim3(&mut self) -> CkslLtim3W<'_, RegCkslGrp1Spec> {
        CkslLtim3W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cksl_grp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cksl_grp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkslGrp1Spec;
impl crate::RegisterSpec for RegCkslGrp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cksl_grp1::R`](R) reader structure"]
impl crate::Readable for RegCkslGrp1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_cksl_grp1::W`](W) writer structure"]
impl crate::Writable for RegCkslGrp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKSL_GRP1 to value 0x0002_7d00"]
impl crate::Resettable for RegCkslGrp1Spec {
    const RESET_VALUE: u32 = 0x0002_7d00;
}
