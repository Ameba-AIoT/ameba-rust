#[doc = "Register `DBG_2` reader"]
pub type R = crate::R<Dbg2Spec>;
#[doc = "Register `DBG_2` writer"]
pub type W = crate::W<Dbg2Spec>;
#[doc = "Field `dbus_dbg_sel` reader - sdio dbus dbg select"]
pub type DbusDbgSelR = crate::FieldReader;
#[doc = "Field `dbus_dbg_sel` writer - sdio dbus dbg select"]
pub type DbusDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - sdio dbus dbg select"]
    #[inline(always)]
    pub fn dbus_dbg_sel(&self) -> DbusDbgSelR {
        DbusDbgSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - sdio dbus dbg select"]
    #[inline(always)]
    pub fn dbus_dbg_sel(&mut self) -> DbusDbgSelW<'_, Dbg2Spec> {
        DbusDbgSelW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg2Spec;
impl crate::RegisterSpec for Dbg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_2::R`](R) reader structure"]
impl crate::Readable for Dbg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_2::W`](W) writer structure"]
impl crate::Writable for Dbg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_2 to value 0"]
impl crate::Resettable for Dbg2Spec {}
