#[doc = "Register `XTAL_FEN` reader"]
pub type R = crate::R<XtalFenSpec>;
#[doc = "Register `XTAL_FEN` writer"]
pub type W = crate::W<XtalFenSpec>;
#[doc = "Field `fen_xpdck` reader - 1:enable PDCK 0:reset PDCK module"]
pub type FenXpdckR = crate::BitReader;
#[doc = "Field `fen_xpdck` writer - 1:enable PDCK 0:reset PDCK module"]
pub type FenXpdckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_xaack` reader - 1:enable AACK 0:reset AACK module"]
pub type FenXaackR = crate::BitReader;
#[doc = "Field `fen_xaack` writer - 1:enable AACK 0:reset AACK module"]
pub type FenXaackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_xmddc` reader - 1:enable xmddc 0:reset xmddc module"]
pub type FenXmddcR = crate::BitReader;
#[doc = "Field `fen_xmddc` writer - 1:enable xmddc 0:reset xmddc module"]
pub type FenXmddcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1:enable PDCK 0:reset PDCK module"]
    #[inline(always)]
    pub fn fen_xpdck(&self) -> FenXpdckR {
        FenXpdckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:enable AACK 0:reset AACK module"]
    #[inline(always)]
    pub fn fen_xaack(&self) -> FenXaackR {
        FenXaackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:enable xmddc 0:reset xmddc module"]
    #[inline(always)]
    pub fn fen_xmddc(&self) -> FenXmddcR {
        FenXmddcR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1:enable PDCK 0:reset PDCK module"]
    #[inline(always)]
    pub fn fen_xpdck(&mut self) -> FenXpdckW<'_, XtalFenSpec> {
        FenXpdckW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:enable AACK 0:reset AACK module"]
    #[inline(always)]
    pub fn fen_xaack(&mut self) -> FenXaackW<'_, XtalFenSpec> {
        FenXaackW::new(self, 1)
    }
    #[doc = "Bit 2 - 1:enable xmddc 0:reset xmddc module"]
    #[inline(always)]
    pub fn fen_xmddc(&mut self) -> FenXmddcW<'_, XtalFenSpec> {
        FenXmddcW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_fen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_fen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalFenSpec;
impl crate::RegisterSpec for XtalFenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_fen::R`](R) reader structure"]
impl crate::Readable for XtalFenSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal_fen::W`](W) writer structure"]
impl crate::Writable for XtalFenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_FEN to value 0x04"]
impl crate::Resettable for XtalFenSpec {
    const RESET_VALUE: u32 = 0x04;
}
