#[doc = "Register `IC_CON` reader"]
pub type R = crate::R<IcConSpec>;
#[doc = "Register `IC_CON` writer"]
pub type W = crate::W<IcConSpec>;
#[doc = "Field `MASTER_MODE` reader - This bit controls whether the I2C master is enabled. * 0: master disabled * 1: master enabled"]
pub type MasterModeR = crate::BitReader;
#[doc = "Field `MASTER_MODE` writer - This bit controls whether the I2C master is enabled. * 0: master disabled * 1: master enabled"]
pub type MasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - These bits control at which speed the I2C operates; its setting is relevant only if one is operating the I2C in master mode. * 1: standard mode (0 to 100kbit/s) * 2: fast mode (<=400kbit/s) * 3: high speed mode (<=3.4Mbit/s)"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `SPEED` writer - These bits control at which speed the I2C operates; its setting is relevant only if one is operating the I2C in master mode. * 1: standard mode (0 to 100kbit/s) * 2: fast mode (<=400kbit/s) * 3: high speed mode (<=3.4Mbit/s)"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC_10BITADDR_SLAVE` reader - When acting as a slave, this bit controls whether the I2C responds to 7- or 10-bit addresses. * 0: 7-bit addressing * 1: 10-bit addressing"]
pub type Ic10bitaddrSlaveR = crate::BitReader;
#[doc = "Field `IC_10BITADDR_SLAVE` writer - When acting as a slave, this bit controls whether the I2C responds to 7- or 10-bit addresses. * 0: 7-bit addressing * 1: 10-bit addressing"]
pub type Ic10bitaddrSlaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_RESTATRT_EN` reader - Determine whether RESTART conditions may be sent when acting as a master. * 0: disable * 1: enable"]
pub type IcRestatrtEnR = crate::BitReader;
#[doc = "Field `IC_RESTATRT_EN` writer - Determine whether RESTART conditions may be sent when acting as a master. * 0: disable * 1: enable"]
pub type IcRestatrtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_SLAVE_DISABLE_0` reader - This bit controls whether I2C has its slave1 (7-bit or 10-bit address) disabled. * 0: slave1 is enabled * 1: slave1 is disabled"]
pub type IcSlaveDisable0R = crate::BitReader;
#[doc = "Field `IC_SLAVE_DISABLE_0` writer - This bit controls whether I2C has its slave1 (7-bit or 10-bit address) disabled. * 0: slave1 is enabled * 1: slave1 is disabled"]
pub type IcSlaveDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_SLAVE_DISABLE_1` reader - This bit controls whether I2C has its slave2 (7-bit address) disabled. * 0: slave2 is enabled * 1: slave2 is disabled"]
pub type IcSlaveDisable1R = crate::BitReader;
#[doc = "Field `IC_SLAVE_DISABLE_1` writer - This bit controls whether I2C has its slave2 (7-bit address) disabled. * 0: slave2 is enabled * 1: slave2 is disabled"]
pub type IcSlaveDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit controls whether the I2C master is enabled. * 0: master disabled * 1: master enabled"]
    #[inline(always)]
    pub fn master_mode(&self) -> MasterModeR {
        MasterModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the I2C operates; its setting is relevant only if one is operating the I2C in master mode. * 1: standard mode (0 to 100kbit/s) * 2: fast mode (<=400kbit/s) * 3: high speed mode (<=3.4Mbit/s)"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - When acting as a slave, this bit controls whether the I2C responds to 7- or 10-bit addresses. * 0: 7-bit addressing * 1: 10-bit addressing"]
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&self) -> Ic10bitaddrSlaveR {
        Ic10bitaddrSlaveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Determine whether RESTART conditions may be sent when acting as a master. * 0: disable * 1: enable"]
    #[inline(always)]
    pub fn ic_restatrt_en(&self) -> IcRestatrtEnR {
        IcRestatrtEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave1 (7-bit or 10-bit address) disabled. * 0: slave1 is enabled * 1: slave1 is disabled"]
    #[inline(always)]
    pub fn ic_slave_disable_0(&self) -> IcSlaveDisable0R {
        IcSlaveDisable0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit controls whether I2C has its slave2 (7-bit address) disabled. * 0: slave2 is enabled * 1: slave2 is disabled"]
    #[inline(always)]
    pub fn ic_slave_disable_1(&self) -> IcSlaveDisable1R {
        IcSlaveDisable1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls whether the I2C master is enabled. * 0: master disabled * 1: master enabled"]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MasterModeW<'_, IcConSpec> {
        MasterModeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the I2C operates; its setting is relevant only if one is operating the I2C in master mode. * 1: standard mode (0 to 100kbit/s) * 2: fast mode (<=400kbit/s) * 3: high speed mode (<=3.4Mbit/s)"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, IcConSpec> {
        SpeedW::new(self, 1)
    }
    #[doc = "Bit 3 - When acting as a slave, this bit controls whether the I2C responds to 7- or 10-bit addresses. * 0: 7-bit addressing * 1: 10-bit addressing"]
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&mut self) -> Ic10bitaddrSlaveW<'_, IcConSpec> {
        Ic10bitaddrSlaveW::new(self, 3)
    }
    #[doc = "Bit 5 - Determine whether RESTART conditions may be sent when acting as a master. * 0: disable * 1: enable"]
    #[inline(always)]
    pub fn ic_restatrt_en(&mut self) -> IcRestatrtEnW<'_, IcConSpec> {
        IcRestatrtEnW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit controls whether I2C has its slave1 (7-bit or 10-bit address) disabled. * 0: slave1 is enabled * 1: slave1 is disabled"]
    #[inline(always)]
    pub fn ic_slave_disable_0(&mut self) -> IcSlaveDisable0W<'_, IcConSpec> {
        IcSlaveDisable0W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit controls whether I2C has its slave2 (7-bit address) disabled. * 0: slave2 is enabled * 1: slave2 is disabled"]
    #[inline(always)]
    pub fn ic_slave_disable_1(&mut self) -> IcSlaveDisable1W<'_, IcConSpec> {
        IcSlaveDisable1W::new(self, 7)
    }
}
#[doc = "I2C Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcConSpec;
impl crate::RegisterSpec for IcConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_con::R`](R) reader structure"]
impl crate::Readable for IcConSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_con::W`](W) writer structure"]
impl crate::Writable for IcConSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_CON to value 0"]
impl crate::Resettable for IcConSpec {}
