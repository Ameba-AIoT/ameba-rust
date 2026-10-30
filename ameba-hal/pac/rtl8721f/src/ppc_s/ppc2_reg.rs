#[doc = "Register `PPC2_REG` reader"]
pub type R = crate::R<Ppc2RegSpec>;
#[doc = "Register `PPC2_REG` writer"]
pub type W = crate::W<Ppc2RegSpec>;
#[doc = "Field `OTPC_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type OtpcCfgR = crate::BitReader;
#[doc = "Field `OTPC_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type OtpcCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_KEY_MANAGE` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type AesKeyManageR = crate::BitReader;
#[doc = "Field `AES_KEY_MANAGE` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type AesKeyManageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSON_NS` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type SysonNsR = crate::BitReader;
#[doc = "Field `SYSON_NS` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type SysonNsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART0` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_BT` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart3BtR = crate::BitReader;
#[doc = "Field `UART3_BT` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart3BtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4_LOG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart4LogR = crate::BitReader;
#[doc = "Field `UART4_LOG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Uart4LogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_A_B_C` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type GpioABCR = crate::BitReader;
#[doc = "Field `GPIO_A_B_C` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type GpioABCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_COMP` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type AdcCompR = crate::BitReader;
#[doc = "Field `ADC_COMP` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type AdcCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_TOUCH` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type CapTouchR = crate::BitReader;
#[doc = "Field `CAP_TOUCH` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type CapTouchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA_KEY_MANAGE` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type ShaKeyManageR = crate::BitReader;
#[doc = "Field `SHA_KEY_MANAGE` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type ShaKeyManageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type IpcR = crate::BitReader;
#[doc = "Field `IPC` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type IpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THERMAL` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type ThermalR = crate::BitReader;
#[doc = "Field `THERMAL` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type ThermalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DGB_TIMER` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type DgbTimerR = crate::BitReader;
#[doc = "Field `DGB_TIMER` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type DgbTimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC_TIMER_0_1` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type PmcTimer0_1R = crate::BitReader;
#[doc = "Field `PMC_TIMER_0_1` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type PmcTimer0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_0_3_BASIC` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Timer0_3BasicR = crate::BitReader;
#[doc = "Field `TIMER_0_3_BASIC` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Timer0_3BasicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn otpc_cfg(&self) -> OtpcCfgR {
        OtpcCfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn aes_key_manage(&self) -> AesKeyManageR {
        AesKeyManageR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn syson_ns(&self) -> SysonNsR {
        SysonNsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart3_bt(&self) -> Uart3BtR {
        Uart3BtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart4_log(&self) -> Uart4LogR {
        Uart4LogR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn gpio_a_b_c(&self) -> GpioABCR {
        GpioABCR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn adc_comp(&self) -> AdcCompR {
        AdcCompR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn cap_touch(&self) -> CapTouchR {
        CapTouchR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sha_key_manage(&self) -> ShaKeyManageR {
        ShaKeyManageR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn ipc(&self) -> IpcR {
        IpcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn thermal(&self) -> ThermalR {
        ThermalR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn dgb_timer(&self) -> DgbTimerR {
        DgbTimerR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn pmc_timer_0_1(&self) -> PmcTimer0_1R {
        PmcTimer0_1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn timer_0_3_basic(&self) -> Timer0_3BasicR {
        Timer0_3BasicR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn otpc_cfg(&mut self) -> OtpcCfgW<'_, Ppc2RegSpec> {
        OtpcCfgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn aes_key_manage(&mut self) -> AesKeyManageW<'_, Ppc2RegSpec> {
        AesKeyManageW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn syson_ns(&mut self) -> SysonNsW<'_, Ppc2RegSpec> {
        SysonNsW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart0(&mut self) -> Uart0W<'_, Ppc2RegSpec> {
        Uart0W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, Ppc2RegSpec> {
        Uart1W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, Ppc2RegSpec> {
        Uart2W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart3_bt(&mut self) -> Uart3BtW<'_, Ppc2RegSpec> {
        Uart3BtW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn uart4_log(&mut self) -> Uart4LogW<'_, Ppc2RegSpec> {
        Uart4LogW::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn gpio_a_b_c(&mut self) -> GpioABCW<'_, Ppc2RegSpec> {
        GpioABCW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn adc_comp(&mut self) -> AdcCompW<'_, Ppc2RegSpec> {
        AdcCompW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn cap_touch(&mut self) -> CapTouchW<'_, Ppc2RegSpec> {
        CapTouchW::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sha_key_manage(&mut self) -> ShaKeyManageW<'_, Ppc2RegSpec> {
        ShaKeyManageW::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IpcW<'_, Ppc2RegSpec> {
        IpcW::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn thermal(&mut self) -> ThermalW<'_, Ppc2RegSpec> {
        ThermalW::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn dgb_timer(&mut self) -> DgbTimerW<'_, Ppc2RegSpec> {
        DgbTimerW::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn pmc_timer_0_1(&mut self) -> PmcTimer0_1W<'_, Ppc2RegSpec> {
        PmcTimer0_1W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn timer_0_3_basic(&mut self) -> Timer0_3BasicW<'_, Ppc2RegSpec> {
        Timer0_3BasicW::new(self, 16)
    }
}
#[doc = "PPC2 configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc2_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc2_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppc2RegSpec;
impl crate::RegisterSpec for Ppc2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc2_reg::R`](R) reader structure"]
impl crate::Readable for Ppc2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc2_reg::W`](W) writer structure"]
impl crate::Writable for Ppc2RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC2_REG to value 0x04"]
impl crate::Resettable for Ppc2RegSpec {
    const RESET_VALUE: u32 = 0x04;
}
