#[doc = "Register `CAN_DBG_PORT` reader"]
pub type R = crate::R<CanDbgPortSpec>;
#[doc = "Register `CAN_DBG_PORT` writer"]
pub type W = crate::W<CanDbgPortSpec>;
#[doc = "Field `can_dbg_sel` reader - debug port selection"]
pub type CanDbgSelR = crate::FieldReader;
#[doc = "Field `can_dbg_sel` writer - debug port selection"]
pub type CanDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_dbg_en` reader - debug port enable"]
pub type CanDbgEnR = crate::BitReader;
#[doc = "Field `can_dbg_en` writer - debug port enable"]
pub type CanDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - debug port selection"]
    #[inline(always)]
    pub fn can_dbg_sel(&self) -> CanDbgSelR {
        CanDbgSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - debug port enable"]
    #[inline(always)]
    pub fn can_dbg_en(&self) -> CanDbgEnR {
        CanDbgEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - debug port selection"]
    #[inline(always)]
    pub fn can_dbg_sel(&mut self) -> CanDbgSelW<'_, CanDbgPortSpec> {
        CanDbgSelW::new(self, 0)
    }
    #[doc = "Bit 8 - debug port enable"]
    #[inline(always)]
    pub fn can_dbg_en(&mut self) -> CanDbgEnW<'_, CanDbgPortSpec> {
        CanDbgEnW::new(self, 8)
    }
}
#[doc = "debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`can_dbg_port::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_dbg_port::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanDbgPortSpec;
impl crate::RegisterSpec for CanDbgPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_dbg_port::R`](R) reader structure"]
impl crate::Readable for CanDbgPortSpec {}
#[doc = "`write(|w| ..)` method takes [`can_dbg_port::W`](W) writer structure"]
impl crate::Writable for CanDbgPortSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_DBG_PORT to value 0"]
impl crate::Resettable for CanDbgPortSpec {}
