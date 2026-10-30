#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `suspend_n` reader - suspend, low active"]
pub type SuspendNR = crate::BitReader;
#[doc = "Field `suspend_n` writer - suspend, low active"]
pub type SuspendNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4_gated_disable` reader - Disable L4 clock gated"]
pub type L4GatedDisableR = crate::BitReader;
#[doc = "Field `L4_gated_disable` writer - Disable L4 clock gated"]
pub type L4GatedDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Dbus_endian_sel` reader - 0: Little; 1: Big"]
pub type DbusEndianSelR = crate::BitReader;
#[doc = "Field `Dbus_endian_sel` writer - 0: Little; 1: Big"]
pub type DbusEndianSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4_gated_disable_ip` reader - Disable L4 clock gated for ip"]
pub type L4GatedDisableIpR = crate::BitReader;
#[doc = "Field `L4_gated_disable_ip` writer - Disable L4 clock gated for ip"]
pub type L4GatedDisableIpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IP_endian_sel` reader - 0:Little; 1:Big"]
pub type IpEndianSelR = crate::BitReader;
#[doc = "Field `IP_endian_sel` writer - 0:Little; 1:Big"]
pub type IpEndianSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - suspend, low active"]
    #[inline(always)]
    pub fn suspend_n(&self) -> SuspendNR {
        SuspendNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable L4 clock gated"]
    #[inline(always)]
    pub fn l4_gated_disable(&self) -> L4GatedDisableR {
        L4GatedDisableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Little; 1: Big"]
    #[inline(always)]
    pub fn dbus_endian_sel(&self) -> DbusEndianSelR {
        DbusEndianSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable L4 clock gated for ip"]
    #[inline(always)]
    pub fn l4_gated_disable_ip(&self) -> L4GatedDisableIpR {
        L4GatedDisableIpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0:Little; 1:Big"]
    #[inline(always)]
    pub fn ip_endian_sel(&self) -> IpEndianSelR {
        IpEndianSelR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - suspend, low active"]
    #[inline(always)]
    pub fn suspend_n(&mut self) -> SuspendNW<'_, CtlSpec> {
        SuspendNW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable L4 clock gated"]
    #[inline(always)]
    pub fn l4_gated_disable(&mut self) -> L4GatedDisableW<'_, CtlSpec> {
        L4GatedDisableW::new(self, 1)
    }
    #[doc = "Bit 2 - 0: Little; 1: Big"]
    #[inline(always)]
    pub fn dbus_endian_sel(&mut self) -> DbusEndianSelW<'_, CtlSpec> {
        DbusEndianSelW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable L4 clock gated for ip"]
    #[inline(always)]
    pub fn l4_gated_disable_ip(&mut self) -> L4GatedDisableIpW<'_, CtlSpec> {
        L4GatedDisableIpW::new(self, 3)
    }
    #[doc = "Bit 4 - 0:Little; 1:Big"]
    #[inline(always)]
    pub fn ip_endian_sel(&mut self) -> IpEndianSelW<'_, CtlSpec> {
        IpEndianSelW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0x01"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x01;
}
