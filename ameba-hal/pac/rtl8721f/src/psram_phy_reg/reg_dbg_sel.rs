#[doc = "Register `REG_DBG_SEL` reader"]
pub type R = crate::R<RegDbgSelSpec>;
#[doc = "Register `REG_DBG_SEL` writer"]
pub type W = crate::W<RegDbgSelSpec>;
#[doc = "Field `PSRAM_PHY_DBG_SEL` reader - psram phy debug select signal."]
pub type PsramPhyDbgSelR = crate::FieldReader;
#[doc = "Field `PSRAM_PHY_DBG_SEL` writer - psram phy debug select signal."]
pub type PsramPhyDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - psram phy debug select signal."]
    #[inline(always)]
    pub fn psram_phy_dbg_sel(&self) -> PsramPhyDbgSelR {
        PsramPhyDbgSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - psram phy debug select signal."]
    #[inline(always)]
    pub fn psram_phy_dbg_sel(&mut self) -> PsramPhyDbgSelW<'_, RegDbgSelSpec> {
        PsramPhyDbgSelW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegDbgSelSpec> {
        DummyW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dbg_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dbg_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDbgSelSpec;
impl crate::RegisterSpec for RegDbgSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dbg_sel::R`](R) reader structure"]
impl crate::Readable for RegDbgSelSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dbg_sel::W`](W) writer structure"]
impl crate::Writable for RegDbgSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DBG_SEL to value 0"]
impl crate::Resettable for RegDbgSelSpec {}
