#[doc = "Register `DBG_PORT_CTRL` reader"]
pub type R = crate::R<DbgPortCtrlSpec>;
#[doc = "Register `DBG_PORT_CTRL` writer"]
pub type W = crate::W<DbgPortCtrlSpec>;
#[doc = "Field `dbg_port_sel` reader - "]
pub type DbgPortSelR = crate::FieldReader;
#[doc = "Field `dbg_port_sel` writer - "]
pub type DbgPortSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `dbg_port_en2` reader - "]
pub type DbgPortEn2R = crate::BitReader;
#[doc = "Field `dbg_port_en2` writer - "]
pub type DbgPortEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbg_port_en` reader - "]
pub type DbgPortEnR = crate::BitReader;
#[doc = "Field `dbg_port_en` writer - "]
pub type DbgPortEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dbg_port_sel(&self) -> DbgPortSelR {
        DbgPortSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dbg_port_en2(&self) -> DbgPortEn2R {
        DbgPortEn2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dbg_port_en(&self) -> DbgPortEnR {
        DbgPortEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dbg_port_sel(&mut self) -> DbgPortSelW<'_, DbgPortCtrlSpec> {
        DbgPortSelW::new(self, 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dbg_port_en2(&mut self) -> DbgPortEn2W<'_, DbgPortCtrlSpec> {
        DbgPortEn2W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dbg_port_en(&mut self) -> DbgPortEnW<'_, DbgPortCtrlSpec> {
        DbgPortEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_port_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_port_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgPortCtrlSpec;
impl crate::RegisterSpec for DbgPortCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_port_ctrl::R`](R) reader structure"]
impl crate::Readable for DbgPortCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_port_ctrl::W`](W) writer structure"]
impl crate::Writable for DbgPortCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_PORT_CTRL to value 0"]
impl crate::Resettable for DbgPortCtrlSpec {}
