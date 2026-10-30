#[doc = "Register `REG_SIC_DBNC` reader"]
pub type R = crate::R<RegSicDbncSpec>;
#[doc = "Register `REG_SIC_DBNC` writer"]
pub type W = crate::W<RegSicDbncSpec>;
#[doc = "Field `sic_dbnc_fen` reader - SIC clk/data input debounce enable"]
pub type SicDbncFenR = crate::BitReader;
#[doc = "Field `sic_dbnc_fen` writer - SIC clk/data input debounce enable"]
pub type SicDbncFenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sic_dbnc_cyc` reader - SIC clk/data input debounce cycle , base on 4M clock"]
pub type SicDbncCycR = crate::FieldReader;
#[doc = "Field `sic_dbnc_cyc` writer - SIC clk/data input debounce cycle , base on 4M clock"]
pub type SicDbncCycW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - SIC clk/data input debounce enable"]
    #[inline(always)]
    pub fn sic_dbnc_fen(&self) -> SicDbncFenR {
        SicDbncFenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - SIC clk/data input debounce cycle , base on 4M clock"]
    #[inline(always)]
    pub fn sic_dbnc_cyc(&self) -> SicDbncCycR {
        SicDbncCycR::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SIC clk/data input debounce enable"]
    #[inline(always)]
    pub fn sic_dbnc_fen(&mut self) -> SicDbncFenW<'_, RegSicDbncSpec> {
        SicDbncFenW::new(self, 0)
    }
    #[doc = "Bits 4:8 - SIC clk/data input debounce cycle , base on 4M clock"]
    #[inline(always)]
    pub fn sic_dbnc_cyc(&mut self) -> SicDbncCycW<'_, RegSicDbncSpec> {
        SicDbncCycW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sic_dbnc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sic_dbnc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSicDbncSpec;
impl crate::RegisterSpec for RegSicDbncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sic_dbnc::R`](R) reader structure"]
impl crate::Readable for RegSicDbncSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sic_dbnc::W`](W) writer structure"]
impl crate::Writable for RegSicDbncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SIC_DBNC to value 0x20"]
impl crate::Resettable for RegSicDbncSpec {
    const RESET_VALUE: u32 = 0x20;
}
