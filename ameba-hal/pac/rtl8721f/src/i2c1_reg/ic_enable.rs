#[doc = "Register `IC_ENABLE` reader"]
pub type R = crate::R<IcEnableSpec>;
#[doc = "Register `IC_ENABLE` writer"]
pub type W = crate::W<IcEnableSpec>;
#[doc = "Field `ENABLE` reader - Control whether the I2C is enabled. * 0: Disable I2C (TX and RX FIFOs are held in an erased state) * 1: Enable I2C"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Control whether the I2C is enabled. * 0: Disable I2C (TX and RX FIFOs are held in an erased state) * 1: Enable I2C"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE` reader - Force master I2C stop transfer immediately after IC_ENABLE set 0. * 1: when IC_ENABLE is set 0 during transfer, force master FSM go to IDLE immediately. * 0: when IC_ENABLE is set 0 during transfer, master FSM while finish current transition (include RESTART) before go to IDLE."]
pub type ForceR = crate::BitReader;
#[doc = "Field `FORCE` writer - Force master I2C stop transfer immediately after IC_ENABLE set 0. * 1: when IC_ENABLE is set 0 during transfer, force master FSM go to IDLE immediately. * 0: when IC_ENABLE is set 0 during transfer, master FSM while finish current transition (include RESTART) before go to IDLE."]
pub type ForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Control whether the I2C is enabled. * 0: Disable I2C (TX and RX FIFOs are held in an erased state) * 1: Enable I2C"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force master I2C stop transfer immediately after IC_ENABLE set 0. * 1: when IC_ENABLE is set 0 during transfer, force master FSM go to IDLE immediately. * 0: when IC_ENABLE is set 0 during transfer, master FSM while finish current transition (include RESTART) before go to IDLE."]
    #[inline(always)]
    pub fn force(&self) -> ForceR {
        ForceR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control whether the I2C is enabled. * 0: Disable I2C (TX and RX FIFOs are held in an erased state) * 1: Enable I2C"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, IcEnableSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Force master I2C stop transfer immediately after IC_ENABLE set 0. * 1: when IC_ENABLE is set 0 during transfer, force master FSM go to IDLE immediately. * 0: when IC_ENABLE is set 0 during transfer, master FSM while finish current transition (include RESTART) before go to IDLE."]
    #[inline(always)]
    pub fn force(&mut self) -> ForceW<'_, IcEnableSpec> {
        ForceW::new(self, 1)
    }
}
#[doc = "I2C Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcEnableSpec;
impl crate::RegisterSpec for IcEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_enable::R`](R) reader structure"]
impl crate::Readable for IcEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_enable::W`](W) writer structure"]
impl crate::Writable for IcEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_ENABLE to value 0"]
impl crate::Resettable for IcEnableSpec {}
