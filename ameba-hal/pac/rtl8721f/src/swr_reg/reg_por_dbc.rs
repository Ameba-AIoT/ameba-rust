#[doc = "Register `REG_POR_DBC` reader"]
pub type R = crate::R<RegPorDbcSpec>;
#[doc = "Register `REG_POR_DBC` writer"]
pub type W = crate::W<RegPorDbcSpec>;
#[doc = "Field `por_off_dbc_cyc` reader - POR_OFF debounce cycle , base in 100K clock period"]
pub type PorOffDbcCycR = crate::FieldReader;
#[doc = "Field `por_off_dbc_cyc` writer - POR_OFF debounce cycle , base in 100K clock period"]
pub type PorOffDbcCycW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `por_off_dbc_fen` reader - Enable POR_OFF debounce"]
pub type PorOffDbcFenR = crate::BitReader;
#[doc = "Field `por_off_dbc_fen` writer - Enable POR_OFF debounce"]
pub type PorOffDbcFenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - POR_OFF debounce cycle , base in 100K clock period"]
    #[inline(always)]
    pub fn por_off_dbc_cyc(&self) -> PorOffDbcCycR {
        PorOffDbcCycR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable POR_OFF debounce"]
    #[inline(always)]
    pub fn por_off_dbc_fen(&self) -> PorOffDbcFenR {
        PorOffDbcFenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - POR_OFF debounce cycle , base in 100K clock period"]
    #[inline(always)]
    pub fn por_off_dbc_cyc(&mut self) -> PorOffDbcCycW<'_, RegPorDbcSpec> {
        PorOffDbcCycW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable POR_OFF debounce"]
    #[inline(always)]
    pub fn por_off_dbc_fen(&mut self) -> PorOffDbcFenW<'_, RegPorDbcSpec> {
        PorOffDbcFenW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_por_dbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_por_dbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPorDbcSpec;
impl crate::RegisterSpec for RegPorDbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_por_dbc::R`](R) reader structure"]
impl crate::Readable for RegPorDbcSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_por_dbc::W`](W) writer structure"]
impl crate::Writable for RegPorDbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_POR_DBC to value 0x01"]
impl crate::Resettable for RegPorDbcSpec {
    const RESET_VALUE: u32 = 0x01;
}
