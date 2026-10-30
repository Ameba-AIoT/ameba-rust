#[doc = "Register `REG_CKD_FEN_USB_PLL` reader"]
pub type R = crate::R<RegCkdFenUsbPllSpec>;
#[doc = "Register `REG_CKD_FEN_USB_PLL` writer"]
pub type W = crate::W<RegCkdFenUsbPllSpec>;
#[doc = "Field `fen_usb_pll_hp` reader - usb_pll_divider for hp enable 1:enable 0:disable"]
pub type FenUsbPllHpR = crate::BitReader;
#[doc = "Field `fen_usb_pll_hp` writer - usb_pll_divider for hp enable 1:enable 0:disable"]
pub type FenUsbPllHpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_hperi` reader - usb_pll_divider for hperi enable 1:enable 0:disable"]
pub type FenUsbPllHperiR = crate::BitReader;
#[doc = "Field `fen_usb_pll_hperi` writer - usb_pll_divider for hperi enable 1:enable 0:disable"]
pub type FenUsbPllHperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_spic` reader - usb_pll_divider for spic enable 1:enable 0:disable"]
pub type FenUsbPllSpicR = crate::BitReader;
#[doc = "Field `fen_usb_pll_spic` writer - usb_pll_divider for spic enable 1:enable 0:disable"]
pub type FenUsbPllSpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_psram` reader - usb_pll_divider for psram enable 1:enable 0:disable"]
pub type FenUsbPllPsramR = crate::BitReader;
#[doc = "Field `fen_usb_pll_psram` writer - usb_pll_divider for psram enable 1:enable 0:disable"]
pub type FenUsbPllPsramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_shperi` reader - usb_pll_divider for shperi enable 1:enable 0:disable"]
pub type FenUsbPllShperiR = crate::BitReader;
#[doc = "Field `fen_usb_pll_shperi` writer - usb_pll_divider for shperi enable 1:enable 0:disable"]
pub type FenUsbPllShperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_can` reader - usb_pll_divider for can 1:enable 0:disable"]
pub type FenUsbPllCanR = crate::BitReader;
#[doc = "Field `fen_usb_pll_can` writer - usb_pll_divider for can 1:enable 0:disable"]
pub type FenUsbPllCanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_vo` reader - sys_pll_divider for vo enable 1:enable 0:disable"]
pub type FenUsbPllVoR = crate::BitReader;
#[doc = "Field `fen_usb_pll_vo` writer - sys_pll_divider for vo enable 1:enable 0:disable"]
pub type FenUsbPllVoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_gmac` reader - usb_pll_divider for gmac enable 1:enable 0:disable"]
pub type FenUsbPllGmacR = crate::BitReader;
#[doc = "Field `fen_usb_pll_gmac` writer - usb_pll_divider for gmac enable 1:enable 0:disable"]
pub type FenUsbPllGmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_i2s` reader - usb_pll_divider for i2s enable 1:enable 0:disable"]
pub type FenUsbPllI2sR = crate::BitReader;
#[doc = "Field `fen_usb_pll_i2s` writer - usb_pll_divider for i2s enable 1:enable 0:disable"]
pub type FenUsbPllI2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb_pll_sdh` reader - usb_pll_divider for sdh enable 1:enable 0:disable"]
pub type FenUsbPllSdhR = crate::BitReader;
#[doc = "Field `fen_usb_pll_sdh` writer - usb_pll_divider for sdh enable 1:enable 0:disable"]
pub type FenUsbPllSdhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - usb_pll_divider for hp enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_hp(&self) -> FenUsbPllHpR {
        FenUsbPllHpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - usb_pll_divider for hperi enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_hperi(&self) -> FenUsbPllHperiR {
        FenUsbPllHperiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - usb_pll_divider for spic enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_spic(&self) -> FenUsbPllSpicR {
        FenUsbPllSpicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - usb_pll_divider for psram enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_psram(&self) -> FenUsbPllPsramR {
        FenUsbPllPsramR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - usb_pll_divider for shperi enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_shperi(&self) -> FenUsbPllShperiR {
        FenUsbPllShperiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - usb_pll_divider for can 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_can(&self) -> FenUsbPllCanR {
        FenUsbPllCanR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - sys_pll_divider for vo enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_vo(&self) -> FenUsbPllVoR {
        FenUsbPllVoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - usb_pll_divider for gmac enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_gmac(&self) -> FenUsbPllGmacR {
        FenUsbPllGmacR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - usb_pll_divider for i2s enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_i2s(&self) -> FenUsbPllI2sR {
        FenUsbPllI2sR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - usb_pll_divider for sdh enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_sdh(&self) -> FenUsbPllSdhR {
        FenUsbPllSdhR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - usb_pll_divider for hp enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_hp(&mut self) -> FenUsbPllHpW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllHpW::new(self, 0)
    }
    #[doc = "Bit 1 - usb_pll_divider for hperi enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_hperi(&mut self) -> FenUsbPllHperiW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllHperiW::new(self, 1)
    }
    #[doc = "Bit 2 - usb_pll_divider for spic enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_spic(&mut self) -> FenUsbPllSpicW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllSpicW::new(self, 2)
    }
    #[doc = "Bit 3 - usb_pll_divider for psram enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_psram(&mut self) -> FenUsbPllPsramW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllPsramW::new(self, 3)
    }
    #[doc = "Bit 4 - usb_pll_divider for shperi enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_shperi(&mut self) -> FenUsbPllShperiW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllShperiW::new(self, 4)
    }
    #[doc = "Bit 5 - usb_pll_divider for can 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_can(&mut self) -> FenUsbPllCanW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllCanW::new(self, 5)
    }
    #[doc = "Bit 6 - sys_pll_divider for vo enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_vo(&mut self) -> FenUsbPllVoW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllVoW::new(self, 6)
    }
    #[doc = "Bit 7 - usb_pll_divider for gmac enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_gmac(&mut self) -> FenUsbPllGmacW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllGmacW::new(self, 7)
    }
    #[doc = "Bit 8 - usb_pll_divider for i2s enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_i2s(&mut self) -> FenUsbPllI2sW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllI2sW::new(self, 8)
    }
    #[doc = "Bit 9 - usb_pll_divider for sdh enable 1:enable 0:disable"]
    #[inline(always)]
    pub fn fen_usb_pll_sdh(&mut self) -> FenUsbPllSdhW<'_, RegCkdFenUsbPllSpec> {
        FenUsbPllSdhW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_fen_usb_pll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_fen_usb_pll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdFenUsbPllSpec;
impl crate::RegisterSpec for RegCkdFenUsbPllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_fen_usb_pll::R`](R) reader structure"]
impl crate::Readable for RegCkdFenUsbPllSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_fen_usb_pll::W`](W) writer structure"]
impl crate::Writable for RegCkdFenUsbPllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_FEN_USB_PLL to value 0x1f"]
impl crate::Resettable for RegCkdFenUsbPllSpec {
    const RESET_VALUE: u32 = 0x1f;
}
