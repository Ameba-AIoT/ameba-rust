#[doc = "Register `REG_CKD_FEN_XTAL` reader"]
pub type R = crate::R<RegCkdFenXtalSpec>;
#[doc = "Register `REG_CKD_FEN_XTAL` writer"]
pub type W = crate::W<RegCkdFenXtalSpec>;
#[doc = "Field `fen_xtal_adc` reader - enable xtal divider for adc use 1:enable 0:disable"]
pub type FenXtalAdcR = crate::BitReader;
#[doc = "Field `fen_xtal_adc` writer - enable xtal divider for adc use 1:enable 0:disable"]
pub type FenXtalAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_xtal_dtim` reader - enable xtal divider for dtim use 1:enable 0:disable"]
pub type FenXtalDtimR = crate::BitReader;
#[doc = "Field `fen_xtal_dtim` writer - enable xtal divider for dtim use 1:enable 0:disable"]
pub type FenXtalDtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_xtal_ltim` reader - enable xtal divider for ltim\\[5:0\\] use 1:enable 0:disable"]
pub type FenXtalLtimR = crate::BitReader;
#[doc = "Field `fen_xtal_ltim` writer - enable xtal divider for ltim\\[5:0\\] use 1:enable 0:disable"]
pub type FenXtalLtimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable xtal divider for adc use 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_xtal_adc(&self) -> FenXtalAdcR {
        FenXtalAdcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable xtal divider for dtim use 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_xtal_dtim(&self) -> FenXtalDtimR {
        FenXtalDtimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable xtal divider for ltim\\[5:0\\] use 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_xtal_ltim(&self) -> FenXtalLtimR {
        FenXtalLtimR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable xtal divider for adc use 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_xtal_adc(&mut self) -> FenXtalAdcW<'_, RegCkdFenXtalSpec> {
        FenXtalAdcW::new(self, 0)
    }
    #[doc = "Bit 1 - enable xtal divider for dtim use 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_xtal_dtim(&mut self) -> FenXtalDtimW<'_, RegCkdFenXtalSpec> {
        FenXtalDtimW::new(self, 1)
    }
    #[doc = "Bit 2 - enable xtal divider for ltim\\[5:0\\] use 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_xtal_ltim(&mut self) -> FenXtalLtimW<'_, RegCkdFenXtalSpec> {
        FenXtalLtimW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_fen_xtal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_fen_xtal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdFenXtalSpec;
impl crate::RegisterSpec for RegCkdFenXtalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_fen_xtal::R`](R) reader structure"]
impl crate::Readable for RegCkdFenXtalSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_fen_xtal::W`](W) writer structure"]
impl crate::Writable for RegCkdFenXtalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_FEN_XTAL to value 0x07"]
impl crate::Resettable for RegCkdFenXtalSpec {
    const RESET_VALUE: u32 = 0x07;
}
