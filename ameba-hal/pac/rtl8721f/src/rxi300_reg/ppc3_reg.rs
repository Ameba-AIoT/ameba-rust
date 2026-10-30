#[doc = "Register `PPC3_REG` reader"]
pub type R = crate::R<Ppc3RegSpec>;
#[doc = "Register `PPC3_REG` writer"]
pub type W = crate::W<Ppc3RegSpec>;
#[doc = "Field `TIMER_4_8_PULSE_PWM_TIMER` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Timer4_8PulsePwmTimerR = crate::BitReader;
#[doc = "Field `TIMER_4_8_PULSE_PWM_TIMER` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Timer4_8PulsePwmTimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_POR1_PORT2` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type TrngPor1Port2R = crate::BitReader;
#[doc = "Field `TRNG_POR1_PORT2` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type TrngPor1Port2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXI300` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Rxi300R = crate::BitReader;
#[doc = "Field `RXI300` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Rxi300W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSIP` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type RsipR = crate::BitReader;
#[doc = "Field `RSIP` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type RsipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Can0R = crate::BitReader;
#[doc = "Field `CAN0` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Can0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Can1R = crate::BitReader;
#[doc = "Field `CAN1` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Can1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type IrR = crate::BitReader;
#[doc = "Field `IR` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type IrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDM` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type PdmR = crate::BitReader;
#[doc = "Field `PDM` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type PdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_OST_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type CpuOstCfgR = crate::BitReader;
#[doc = "Field `CPU_OST_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type CpuOstCfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn timer_4_8_pulse_pwm_timer(&self) -> Timer4_8PulsePwmTimerR {
        Timer4_8PulsePwmTimerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn trng_por1_port2(&self) -> TrngPor1Port2R {
        TrngPor1Port2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn rxi300(&self) -> Rxi300R {
        Rxi300R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn rsip(&self) -> RsipR {
        RsipR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn can0(&self) -> Can0R {
        Can0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn can1(&self) -> Can1R {
        Can1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn ir(&self) -> IrR {
        IrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn pdm(&self) -> PdmR {
        PdmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn cpu_ost_cfg(&self) -> CpuOstCfgR {
        CpuOstCfgR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn timer_4_8_pulse_pwm_timer(&mut self) -> Timer4_8PulsePwmTimerW<'_, Ppc3RegSpec> {
        Timer4_8PulsePwmTimerW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn trng_por1_port2(&mut self) -> TrngPor1Port2W<'_, Ppc3RegSpec> {
        TrngPor1Port2W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn rxi300(&mut self) -> Rxi300W<'_, Ppc3RegSpec> {
        Rxi300W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn rsip(&mut self) -> RsipW<'_, Ppc3RegSpec> {
        RsipW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn can0(&mut self) -> Can0W<'_, Ppc3RegSpec> {
        Can0W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn can1(&mut self) -> Can1W<'_, Ppc3RegSpec> {
        Can1W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn ir(&mut self) -> IrW<'_, Ppc3RegSpec> {
        IrW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<'_, Ppc3RegSpec> {
        I2c0W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Ppc3RegSpec> {
        I2c1W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn pdm(&mut self) -> PdmW<'_, Ppc3RegSpec> {
        PdmW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn cpu_ost_cfg(&mut self) -> CpuOstCfgW<'_, Ppc3RegSpec> {
        CpuOstCfgW::new(self, 10)
    }
}
#[doc = "PPC3 configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc3_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc3_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppc3RegSpec;
impl crate::RegisterSpec for Ppc3RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc3_reg::R`](R) reader structure"]
impl crate::Readable for Ppc3RegSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc3_reg::W`](W) writer structure"]
impl crate::Writable for Ppc3RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC3_REG to value 0"]
impl crate::Resettable for Ppc3RegSpec {}
