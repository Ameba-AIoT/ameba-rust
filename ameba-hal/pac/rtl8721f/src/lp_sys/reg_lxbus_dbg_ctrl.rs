#[doc = "Register `REG_LXBUS_DBG_CTRL` reader"]
pub type R = crate::R<RegLxbusDbgCtrlSpec>;
#[doc = "Register `REG_LXBUS_DBG_CTRL` writer"]
pub type W = crate::W<RegLxbusDbgCtrlSpec>;
#[doc = "Field `enbustimeout` reader - lx sys bus arb timeout enable"]
pub type EnbustimeoutR = crate::BitReader;
#[doc = "Field `enbustimeout` writer - lx sys bus arb timeout enable"]
pub type EnbustimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bustimeout` reader - lx sys bus arb timeout thresh"]
pub type BustimeoutR = crate::FieldReader;
#[doc = "Field `bustimeout` writer - lx sys bus arb timeout thresh"]
pub type BustimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lx1_lock_bus` reader - lock lx1 bus for debug"]
pub type Lx1LockBusR = crate::BitReader;
#[doc = "Field `lx1_lock_bus` writer - lock lx1 bus for debug"]
pub type Lx1LockBusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lx1_lock_ok` reader - lock lx1 bus lock ok"]
pub type Lx1LockOkR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - lx sys bus arb timeout enable"]
    #[inline(always)]
    pub fn enbustimeout(&self) -> EnbustimeoutR {
        EnbustimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - lx sys bus arb timeout thresh"]
    #[inline(always)]
    pub fn bustimeout(&self) -> BustimeoutR {
        BustimeoutR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - lock lx1 bus for debug"]
    #[inline(always)]
    pub fn lx1_lock_bus(&self) -> Lx1LockBusR {
        Lx1LockBusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - lock lx1 bus lock ok"]
    #[inline(always)]
    pub fn lx1_lock_ok(&self) -> Lx1LockOkR {
        Lx1LockOkR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lx sys bus arb timeout enable"]
    #[inline(always)]
    pub fn enbustimeout(&mut self) -> EnbustimeoutW<'_, RegLxbusDbgCtrlSpec> {
        EnbustimeoutW::new(self, 0)
    }
    #[doc = "Bits 4:5 - lx sys bus arb timeout thresh"]
    #[inline(always)]
    pub fn bustimeout(&mut self) -> BustimeoutW<'_, RegLxbusDbgCtrlSpec> {
        BustimeoutW::new(self, 4)
    }
    #[doc = "Bit 8 - lock lx1 bus for debug"]
    #[inline(always)]
    pub fn lx1_lock_bus(&mut self) -> Lx1LockBusW<'_, RegLxbusDbgCtrlSpec> {
        Lx1LockBusW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lxbus_dbg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lxbus_dbg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLxbusDbgCtrlSpec;
impl crate::RegisterSpec for RegLxbusDbgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lxbus_dbg_ctrl::R`](R) reader structure"]
impl crate::Readable for RegLxbusDbgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lxbus_dbg_ctrl::W`](W) writer structure"]
impl crate::Writable for RegLxbusDbgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LXBUS_DBG_CTRL to value 0"]
impl crate::Resettable for RegLxbusDbgCtrlSpec {}
