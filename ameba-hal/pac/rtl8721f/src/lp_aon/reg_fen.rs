#[doc = "Register `REG_FEN` reader"]
pub type R = crate::R<RegFenSpec>;
#[doc = "Register `REG_FEN` writer"]
pub type W = crate::W<RegFenSpec>;
#[doc = "Field `fen_bor` reader - Enable bor reset function, default is 0 when power on"]
pub type FenBorR = crate::BitReader;
#[doc = "Field `fen_bor` writer - Enable bor reset function, default is 0 when power on"]
pub type FenBorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_iwdg` reader - 1: Enable IWDG by HW autoload. Once enabled, IWDG can't be disabled unless por asserts."]
pub type FenIwdgR = crate::BitReader;
#[doc = "Field `fen_iwdg` writer - 1: Enable IWDG by HW autoload. Once enabled, IWDG can't be disabled unless por asserts."]
pub type FenIwdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_atim` reader - 1: Enable AON timer 0: Reset AON timer Note: AON timer starts to run only when event happens."]
pub type FenAtimR = crate::BitReader;
#[doc = "Field `fen_atim` writer - 1: Enable AON timer 0: Reset AON timer Note: AON timer starts to run only when event happens."]
pub type FenAtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sdm` reader - 1: Enable 32.768K SDM function 0: Disable"]
pub type FenSdmR = crate::BitReader;
#[doc = "Field `fen_sdm` writer - 1: Enable 32.768K SDM function 0: Disable"]
pub type FenSdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_ledc` reader - 1: Enable led display function 0: Disable Note: this module is in syson power domain"]
pub type FenLedcR = crate::BitReader;
#[doc = "Field `fen_ledc` writer - 1: Enable led display function 0: Disable Note: this module is in syson power domain"]
pub type FenLedcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_px` reader - 1: Enable pad interface function 0: Disable Note: this module is in syson power domain"]
pub type FenPxR = crate::BitReader;
#[doc = "Field `fen_px` writer - 1: Enable pad interface function 0: Disable Note: this module is in syson power domain"]
pub type FenPxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_lpon` reader - LS global reset 1: Enable LS function 0: Disable"]
pub type FenLponR = crate::BitReader;
#[doc = "Field `fen_lpon` writer - LS global reset 1: Enable LS function 0: Disable"]
pub type FenLponW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable bor reset function, default is 0 when power on"]
    #[inline(always)]
    pub fn fen_bor(&self) -> FenBorR {
        FenBorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable IWDG by HW autoload. Once enabled, IWDG can't be disabled unless por asserts."]
    #[inline(always)]
    pub fn fen_iwdg(&self) -> FenIwdgR {
        FenIwdgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable AON timer 0: Reset AON timer Note: AON timer starts to run only when event happens."]
    #[inline(always)]
    pub fn fen_atim(&self) -> FenAtimR {
        FenAtimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable 32.768K SDM function 0: Disable"]
    #[inline(always)]
    pub fn fen_sdm(&self) -> FenSdmR {
        FenSdmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable led display function 0: Disable Note: this module is in syson power domain"]
    #[inline(always)]
    pub fn fen_ledc(&self) -> FenLedcR {
        FenLedcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Enable pad interface function 0: Disable Note: this module is in syson power domain"]
    #[inline(always)]
    pub fn fen_px(&self) -> FenPxR {
        FenPxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LS global reset 1: Enable LS function 0: Disable"]
    #[inline(always)]
    pub fn fen_lpon(&self) -> FenLponR {
        FenLponR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bor reset function, default is 0 when power on"]
    #[inline(always)]
    pub fn fen_bor(&mut self) -> FenBorW<'_, RegFenSpec> {
        FenBorW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable IWDG by HW autoload. Once enabled, IWDG can't be disabled unless por asserts."]
    #[inline(always)]
    pub fn fen_iwdg(&mut self) -> FenIwdgW<'_, RegFenSpec> {
        FenIwdgW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Enable AON timer 0: Reset AON timer Note: AON timer starts to run only when event happens."]
    #[inline(always)]
    pub fn fen_atim(&mut self) -> FenAtimW<'_, RegFenSpec> {
        FenAtimW::new(self, 2)
    }
    #[doc = "Bit 4 - 1: Enable 32.768K SDM function 0: Disable"]
    #[inline(always)]
    pub fn fen_sdm(&mut self) -> FenSdmW<'_, RegFenSpec> {
        FenSdmW::new(self, 4)
    }
    #[doc = "Bit 8 - 1: Enable led display function 0: Disable Note: this module is in syson power domain"]
    #[inline(always)]
    pub fn fen_ledc(&mut self) -> FenLedcW<'_, RegFenSpec> {
        FenLedcW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Enable pad interface function 0: Disable Note: this module is in syson power domain"]
    #[inline(always)]
    pub fn fen_px(&mut self) -> FenPxW<'_, RegFenSpec> {
        FenPxW::new(self, 9)
    }
    #[doc = "Bit 16 - LS global reset 1: Enable LS function 0: Disable"]
    #[inline(always)]
    pub fn fen_lpon(&mut self) -> FenLponW<'_, RegFenSpec> {
        FenLponW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFenSpec;
impl crate::RegisterSpec for RegFenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fen::R`](R) reader structure"]
impl crate::Readable for RegFenSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fen::W`](W) writer structure"]
impl crate::Writable for RegFenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FEN to value 0x0200"]
impl crate::Resettable for RegFenSpec {
    const RESET_VALUE: u32 = 0x0200;
}
