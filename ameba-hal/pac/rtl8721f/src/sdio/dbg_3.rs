#[doc = "Register `DBG_3` reader"]
pub type R = crate::R<Dbg3Spec>;
#[doc = "Register `DBG_3` writer"]
pub type W = crate::W<Dbg3Spec>;
#[doc = "Field `ip_dbg_sel` reader - sdio ip dbg select"]
pub type IpDbgSelR = crate::FieldReader;
#[doc = "Field `ip_dbg_sel` writer - sdio ip dbg select"]
pub type IpDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sdio ip dbg select"]
    #[inline(always)]
    pub fn ip_dbg_sel(&self) -> IpDbgSelR {
        IpDbgSelR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sdio ip dbg select"]
    #[inline(always)]
    pub fn ip_dbg_sel(&mut self) -> IpDbgSelW<'_, Dbg3Spec> {
        IpDbgSelW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg3Spec;
impl crate::RegisterSpec for Dbg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_3::R`](R) reader structure"]
impl crate::Readable for Dbg3Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_3::W`](W) writer structure"]
impl crate::Writable for Dbg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_3 to value 0"]
impl crate::Resettable for Dbg3Spec {}
