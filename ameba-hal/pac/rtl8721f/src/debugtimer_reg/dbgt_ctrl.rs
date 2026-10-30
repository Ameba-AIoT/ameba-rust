#[doc = "Register `DBGT_CTRL` reader"]
pub type R = crate::R<DbgtCtrlSpec>;
#[doc = "Register `DBGT_CTRL` writer"]
pub type W = crate::W<DbgtCtrlSpec>;
#[doc = "Field `enable` reader - Debug timer enable. 1: The counter is enabled. 0: The counter is disabled."]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Debug timer enable. 1: The counter is enabled. 0: The counter is disabled."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbgt_crv_set` writer - set dbgt_crv0/1 to dbgtcnt"]
pub type DbgtCrvSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug timer enable. 1: The counter is enabled. 0: The counter is disabled."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug timer enable. 1: The counter is enabled. 0: The counter is disabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, DbgtCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - set dbgt_crv0/1 to dbgtcnt"]
    #[inline(always)]
    pub fn dbgt_crv_set(&mut self) -> DbgtCrvSetW<'_, DbgtCtrlSpec> {
        DbgtCrvSetW::new(self, 1)
    }
}
#[doc = "debugtimer control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgtCtrlSpec;
impl crate::RegisterSpec for DbgtCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgt_ctrl::R`](R) reader structure"]
impl crate::Readable for DbgtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgt_ctrl::W`](W) writer structure"]
impl crate::Writable for DbgtCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGT_CTRL to value 0x01"]
impl crate::Resettable for DbgtCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
