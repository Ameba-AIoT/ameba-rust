#[doc = "Register `REG_CKD_FEN_SYS_PLL` reader"]
pub type R = crate::R<RegCkdFenSysPllSpec>;
#[doc = "Register `REG_CKD_FEN_SYS_PLL` writer"]
pub type W = crate::W<RegCkdFenSysPllSpec>;
#[doc = "Field `fen_sys_pll_hp` reader - sys_pll_divider for hp enable 1:enable 0:disable"]
pub type FenSysPllHpR = crate::BitReader;
#[doc = "Field `fen_sys_pll_hp` writer - sys_pll_divider for hp enable 1:enable 0:disable"]
pub type FenSysPllHpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_hperi` reader - sys_pll_divider for hperi enable 1:enable 0:disable"]
pub type FenSysPllHperiR = crate::BitReader;
#[doc = "Field `fen_sys_pll_hperi` writer - sys_pll_divider for hperi enable 1:enable 0:disable"]
pub type FenSysPllHperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_spic` reader - sys_pll_divider for spic enable 1:enable 0:disable"]
pub type FenSysPllSpicR = crate::BitReader;
#[doc = "Field `fen_sys_pll_spic` writer - sys_pll_divider for spic enable 1:enable 0:disable"]
pub type FenSysPllSpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_psram` reader - sys_pll_divider for psram enable 1:enable 0:disable"]
pub type FenSysPllPsramR = crate::BitReader;
#[doc = "Field `fen_sys_pll_psram` writer - sys_pll_divider for psram enable 1:enable 0:disable"]
pub type FenSysPllPsramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_shperi` reader - sys_pll_divider for shperi enable 1:enable 0:disable"]
pub type FenSysPllShperiR = crate::BitReader;
#[doc = "Field `fen_sys_pll_shperi` writer - sys_pll_divider for shperi enable 1:enable 0:disable"]
pub type FenSysPllShperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_can` reader - sys_pll_divider for can 1:enable 0:disable"]
pub type FenSysPllCanR = crate::BitReader;
#[doc = "Field `fen_sys_pll_can` writer - sys_pll_divider for can 1:enable 0:disable"]
pub type FenSysPllCanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_vo` reader - sys_pll_divider for vo enable 1:enable 0:disable"]
pub type FenSysPllVoR = crate::BitReader;
#[doc = "Field `fen_sys_pll_vo` writer - sys_pll_divider for vo enable 1:enable 0:disable"]
pub type FenSysPllVoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_gmac` reader - sys_pll_divider for gmac enable 1:enable 0:disable"]
pub type FenSysPllGmacR = crate::BitReader;
#[doc = "Field `fen_sys_pll_gmac` writer - sys_pll_divider for gmac enable 1:enable 0:disable"]
pub type FenSysPllGmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_i2s` reader - sys_pll_divider for i2s enable 1:enable 0:disable"]
pub type FenSysPllI2sR = crate::BitReader;
#[doc = "Field `fen_sys_pll_i2s` writer - sys_pll_divider for i2s enable 1:enable 0:disable"]
pub type FenSysPllI2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sys_pll_sdh` reader - sys_pll_divider for sdh enable 1:enable 0:disable"]
pub type FenSysPllSdhR = crate::BitReader;
#[doc = "Field `fen_sys_pll_sdh` writer - sys_pll_divider for sdh enable 1:enable 0:disable"]
pub type FenSysPllSdhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sys_pll_divider for hp enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_hp(&self) -> FenSysPllHpR {
        FenSysPllHpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sys_pll_divider for hperi enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_hperi(&self) -> FenSysPllHperiR {
        FenSysPllHperiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sys_pll_divider for spic enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_spic(&self) -> FenSysPllSpicR {
        FenSysPllSpicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sys_pll_divider for psram enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_psram(&self) -> FenSysPllPsramR {
        FenSysPllPsramR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sys_pll_divider for shperi enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_shperi(&self) -> FenSysPllShperiR {
        FenSysPllShperiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - sys_pll_divider for can 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_can(&self) -> FenSysPllCanR {
        FenSysPllCanR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - sys_pll_divider for vo enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_vo(&self) -> FenSysPllVoR {
        FenSysPllVoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - sys_pll_divider for gmac enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_gmac(&self) -> FenSysPllGmacR {
        FenSysPllGmacR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - sys_pll_divider for i2s enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_i2s(&self) -> FenSysPllI2sR {
        FenSysPllI2sR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - sys_pll_divider for sdh enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_sdh(&self) -> FenSysPllSdhR {
        FenSysPllSdhR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sys_pll_divider for hp enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_hp(&mut self) -> FenSysPllHpW<'_, RegCkdFenSysPllSpec> {
        FenSysPllHpW::new(self, 0)
    }
    #[doc = "Bit 1 - sys_pll_divider for hperi enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_hperi(&mut self) -> FenSysPllHperiW<'_, RegCkdFenSysPllSpec> {
        FenSysPllHperiW::new(self, 1)
    }
    #[doc = "Bit 2 - sys_pll_divider for spic enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_spic(&mut self) -> FenSysPllSpicW<'_, RegCkdFenSysPllSpec> {
        FenSysPllSpicW::new(self, 2)
    }
    #[doc = "Bit 3 - sys_pll_divider for psram enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_psram(&mut self) -> FenSysPllPsramW<'_, RegCkdFenSysPllSpec> {
        FenSysPllPsramW::new(self, 3)
    }
    #[doc = "Bit 4 - sys_pll_divider for shperi enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_shperi(&mut self) -> FenSysPllShperiW<'_, RegCkdFenSysPllSpec> {
        FenSysPllShperiW::new(self, 4)
    }
    #[doc = "Bit 5 - sys_pll_divider for can 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_can(&mut self) -> FenSysPllCanW<'_, RegCkdFenSysPllSpec> {
        FenSysPllCanW::new(self, 5)
    }
    #[doc = "Bit 6 - sys_pll_divider for vo enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_vo(&mut self) -> FenSysPllVoW<'_, RegCkdFenSysPllSpec> {
        FenSysPllVoW::new(self, 6)
    }
    #[doc = "Bit 7 - sys_pll_divider for gmac enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_gmac(&mut self) -> FenSysPllGmacW<'_, RegCkdFenSysPllSpec> {
        FenSysPllGmacW::new(self, 7)
    }
    #[doc = "Bit 8 - sys_pll_divider for i2s enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_i2s(&mut self) -> FenSysPllI2sW<'_, RegCkdFenSysPllSpec> {
        FenSysPllI2sW::new(self, 8)
    }
    #[doc = "Bit 9 - sys_pll_divider for sdh enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_sys_pll_sdh(&mut self) -> FenSysPllSdhW<'_, RegCkdFenSysPllSpec> {
        FenSysPllSdhW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_fen_sys_pll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_fen_sys_pll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdFenSysPllSpec;
impl crate::RegisterSpec for RegCkdFenSysPllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_fen_sys_pll::R`](R) reader structure"]
impl crate::Readable for RegCkdFenSysPllSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_fen_sys_pll::W`](W) writer structure"]
impl crate::Writable for RegCkdFenSysPllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_FEN_SYS_PLL to value 0x1f"]
impl crate::Resettable for RegCkdFenSysPllSpec {
    const RESET_VALUE: u32 = 0x1f;
}
