#[doc = "Register `REG_FEN_GRP0` reader"]
pub type R = crate::R<RegFenGrp0Spec>;
#[doc = "Register `REG_FEN_GRP0` writer"]
pub type W = crate::W<RegFenGrp0Spec>;
#[doc = "Field `fen_soc` reader - 1: enable hs soc sytem 0: disable hs soc system (Note: used as soc power on reset)"]
pub type FenSocR = crate::BitReader;
#[doc = "Field `fen_soc` writer - 1: enable hs soc sytem 0: disable hs soc system (Note: used as soc power on reset)"]
pub type FenSocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_plfm` reader - 1: enable platform 0: reset"]
pub type FenPlfmR = crate::BitReader;
#[doc = "Field `fen_plfm` writer - 1: enable platform 0: reset"]
pub type FenPlfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_cpu1` reader - 1: enable cpu1 (KM4NS) 0: reset"]
pub type FenCpu1R = crate::BitReader;
#[doc = "Field `fen_cpu1` writer - 1: enable cpu1 (KM4NS) 0: reset"]
pub type FenCpu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_cpu0` reader - 1: enable cpu0 (KM4TZ) 0: reset"]
pub type FenCpu0R = crate::BitReader;
#[doc = "Field `fen_cpu0` writer - 1: enable cpu0 (KM4TZ) 0: reset"]
pub type FenCpu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_mjpeg` reader - 1: enable mjpeg module 0: reset No use"]
pub type FenMjpegR = crate::BitReader;
#[doc = "Field `fen_mjpeg` writer - 1: enable mjpeg module 0: reset No use"]
pub type FenMjpegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_ipc` reader - 1: enable IPC function 0: reset"]
pub type FenIpcR = crate::BitReader;
#[doc = "Field `fen_ipc` writer - 1: enable IPC function 0: reset"]
pub type FenIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_dmac` reader - 1: enable GDMA function 0: disable"]
pub type FenDmacR = crate::BitReader;
#[doc = "Field `fen_dmac` writer - 1: enable GDMA function 0: disable"]
pub type FenDmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_ppe` reader - 1: enable ppe module 0: reset No use"]
pub type FenPpeR = crate::BitReader;
#[doc = "Field `fen_ppe` writer - 1: enable ppe module 0: reset No use"]
pub type FenPpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_crypto` reader - 1: enable IPSEC (AES + HMAC) function 0: disable"]
pub type FenCryptoR = crate::BitReader;
#[doc = "Field `fen_crypto` writer - 1: enable IPSEC (AES + HMAC) function 0: disable"]
pub type FenCryptoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_ac` reader - 1: enable PDM(ac) module 0: reset No use"]
pub type FenAcR = crate::BitReader;
#[doc = "Field `fen_ac` writer - 1: enable PDM(ac) module 0: reset No use"]
pub type FenAcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_spic` reader - Flash spic function enable 1: enable flash 0: disable flash"]
pub type FenSpicR = crate::BitReader;
#[doc = "Field `fen_spic` writer - Flash spic function enable 1: enable flash 0: disable flash"]
pub type FenSpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_lcdc` reader - 1: enable ppe module 0: reset No use"]
pub type FenLcdcR = crate::BitReader;
#[doc = "Field `fen_lcdc` writer - 1: enable ppe module 0: reset No use"]
pub type FenLcdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_psram` reader - 1: enable PSRAM function 0: disable"]
pub type FenPsramR = crate::BitReader;
#[doc = "Field `fen_psram` writer - 1: enable PSRAM function 0: disable"]
pub type FenPsramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_spi` reader - 1: enable SPI1 function 0: disable"]
pub type FenSpiR = crate::FieldReader;
#[doc = "Field `fen_spi` writer - 1: enable SPI1 function 0: disable"]
pub type FenSpiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fen_can` reader - 1: enable can0/1 module 0: reset No use"]
pub type FenCanR = crate::FieldReader;
#[doc = "Field `fen_can` writer - 1: enable can0/1 module 0: reset No use"]
pub type FenCanW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fen_trng` reader - 1: enable TRNG function 0: disable"]
pub type FenTrngR = crate::BitReader;
#[doc = "Field `fen_trng` writer - 1: enable TRNG function 0: disable"]
pub type FenTrngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_gmac` reader - 1: enable ppe module 0: reset No use"]
pub type FenGmacR = crate::BitReader;
#[doc = "Field `fen_gmac` writer - 1: enable ppe module 0: reset No use"]
pub type FenGmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sport` reader - 1: enable SPORT function 0: disable"]
pub type FenSportR = crate::BitReader;
#[doc = "Field `fen_sport` writer - 1: enable SPORT function 0: disable"]
pub type FenSportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sdh` reader - 1: enable sdio host function 0: disable"]
pub type FenSdhR = crate::BitReader;
#[doc = "Field `fen_sdh` writer - 1: enable sdio host function 0: disable"]
pub type FenSdhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sdd` reader - 1: enable sdio device function 0: disable"]
pub type FenSddR = crate::BitReader;
#[doc = "Field `fen_sdd` writer - 1: enable sdio device function 0: disable"]
pub type FenSddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_pwm` reader - 1: enable PWM TIMER function 0: disable \\[4:1\\]:pwm \\[0\\]:pulse"]
pub type FenPwmR = crate::FieldReader;
#[doc = "Field `fen_pwm` writer - 1: enable PWM TIMER function 0: disable \\[4:1\\]:pwm \\[0\\]:pulse"]
pub type FenPwmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `fen_pke` reader - 1: enable pke module 0: reset No use"]
pub type FenPkeR = crate::BitReader;
#[doc = "Field `fen_pke` writer - 1: enable pke module 0: reset No use"]
pub type FenPkeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable hs soc sytem 0: disable hs soc system (Note: used as soc power on reset)"]
    #[inline(always)]
    pub fn fen_soc(&self) -> FenSocR {
        FenSocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enable platform 0: reset"]
    #[inline(always)]
    pub fn fen_plfm(&self) -> FenPlfmR {
        FenPlfmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: enable cpu1 (KM4NS) 0: reset"]
    #[inline(always)]
    pub fn fen_cpu1(&self) -> FenCpu1R {
        FenCpu1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: enable cpu0 (KM4TZ) 0: reset"]
    #[inline(always)]
    pub fn fen_cpu0(&self) -> FenCpu0R {
        FenCpu0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: enable mjpeg module 0: reset No use"]
    #[inline(always)]
    pub fn fen_mjpeg(&self) -> FenMjpegR {
        FenMjpegR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: enable IPC function 0: reset"]
    #[inline(always)]
    pub fn fen_ipc(&self) -> FenIpcR {
        FenIpcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: enable GDMA function 0: disable"]
    #[inline(always)]
    pub fn fen_dmac(&self) -> FenDmacR {
        FenDmacR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: enable ppe module 0: reset No use"]
    #[inline(always)]
    pub fn fen_ppe(&self) -> FenPpeR {
        FenPpeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: enable IPSEC (AES + HMAC) function 0: disable"]
    #[inline(always)]
    pub fn fen_crypto(&self) -> FenCryptoR {
        FenCryptoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: enable PDM(ac) module 0: reset No use"]
    #[inline(always)]
    pub fn fen_ac(&self) -> FenAcR {
        FenAcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Flash spic function enable 1: enable flash 0: disable flash"]
    #[inline(always)]
    pub fn fen_spic(&self) -> FenSpicR {
        FenSpicR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: enable ppe module 0: reset No use"]
    #[inline(always)]
    pub fn fen_lcdc(&self) -> FenLcdcR {
        FenLcdcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: enable PSRAM function 0: disable"]
    #[inline(always)]
    pub fn fen_psram(&self) -> FenPsramR {
        FenPsramR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 1: enable SPI1 function 0: disable"]
    #[inline(always)]
    pub fn fen_spi(&self) -> FenSpiR {
        FenSpiR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 1: enable can0/1 module 0: reset No use"]
    #[inline(always)]
    pub fn fen_can(&self) -> FenCanR {
        FenCanR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 1: enable TRNG function 0: disable"]
    #[inline(always)]
    pub fn fen_trng(&self) -> FenTrngR {
        FenTrngR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: enable ppe module 0: reset No use"]
    #[inline(always)]
    pub fn fen_gmac(&self) -> FenGmacR {
        FenGmacR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: enable SPORT function 0: disable"]
    #[inline(always)]
    pub fn fen_sport(&self) -> FenSportR {
        FenSportR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: enable sdio host function 0: disable"]
    #[inline(always)]
    pub fn fen_sdh(&self) -> FenSdhR {
        FenSdhR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: enable sdio device function 0: disable"]
    #[inline(always)]
    pub fn fen_sdd(&self) -> FenSddR {
        FenSddR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - 1: enable PWM TIMER function 0: disable \\[4:1\\]:pwm \\[0\\]:pulse"]
    #[inline(always)]
    pub fn fen_pwm(&self) -> FenPwmR {
        FenPwmR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - 1: enable pke module 0: reset No use"]
    #[inline(always)]
    pub fn fen_pke(&self) -> FenPkeR {
        FenPkeR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable hs soc sytem 0: disable hs soc system (Note: used as soc power on reset)"]
    #[inline(always)]
    pub fn fen_soc(&mut self) -> FenSocW<'_, RegFenGrp0Spec> {
        FenSocW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enable platform 0: reset"]
    #[inline(always)]
    pub fn fen_plfm(&mut self) -> FenPlfmW<'_, RegFenGrp0Spec> {
        FenPlfmW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: enable cpu1 (KM4NS) 0: reset"]
    #[inline(always)]
    pub fn fen_cpu1(&mut self) -> FenCpu1W<'_, RegFenGrp0Spec> {
        FenCpu1W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: enable cpu0 (KM4TZ) 0: reset"]
    #[inline(always)]
    pub fn fen_cpu0(&mut self) -> FenCpu0W<'_, RegFenGrp0Spec> {
        FenCpu0W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: enable mjpeg module 0: reset No use"]
    #[inline(always)]
    pub fn fen_mjpeg(&mut self) -> FenMjpegW<'_, RegFenGrp0Spec> {
        FenMjpegW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: enable IPC function 0: reset"]
    #[inline(always)]
    pub fn fen_ipc(&mut self) -> FenIpcW<'_, RegFenGrp0Spec> {
        FenIpcW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: enable GDMA function 0: disable"]
    #[inline(always)]
    pub fn fen_dmac(&mut self) -> FenDmacW<'_, RegFenGrp0Spec> {
        FenDmacW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: enable ppe module 0: reset No use"]
    #[inline(always)]
    pub fn fen_ppe(&mut self) -> FenPpeW<'_, RegFenGrp0Spec> {
        FenPpeW::new(self, 7)
    }
    #[doc = "Bit 8 - 1: enable IPSEC (AES + HMAC) function 0: disable"]
    #[inline(always)]
    pub fn fen_crypto(&mut self) -> FenCryptoW<'_, RegFenGrp0Spec> {
        FenCryptoW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: enable PDM(ac) module 0: reset No use"]
    #[inline(always)]
    pub fn fen_ac(&mut self) -> FenAcW<'_, RegFenGrp0Spec> {
        FenAcW::new(self, 9)
    }
    #[doc = "Bit 10 - Flash spic function enable 1: enable flash 0: disable flash"]
    #[inline(always)]
    pub fn fen_spic(&mut self) -> FenSpicW<'_, RegFenGrp0Spec> {
        FenSpicW::new(self, 10)
    }
    #[doc = "Bit 11 - 1: enable ppe module 0: reset No use"]
    #[inline(always)]
    pub fn fen_lcdc(&mut self) -> FenLcdcW<'_, RegFenGrp0Spec> {
        FenLcdcW::new(self, 11)
    }
    #[doc = "Bit 13 - 1: enable PSRAM function 0: disable"]
    #[inline(always)]
    pub fn fen_psram(&mut self) -> FenPsramW<'_, RegFenGrp0Spec> {
        FenPsramW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 1: enable SPI1 function 0: disable"]
    #[inline(always)]
    pub fn fen_spi(&mut self) -> FenSpiW<'_, RegFenGrp0Spec> {
        FenSpiW::new(self, 14)
    }
    #[doc = "Bits 16:17 - 1: enable can0/1 module 0: reset No use"]
    #[inline(always)]
    pub fn fen_can(&mut self) -> FenCanW<'_, RegFenGrp0Spec> {
        FenCanW::new(self, 16)
    }
    #[doc = "Bit 18 - 1: enable TRNG function 0: disable"]
    #[inline(always)]
    pub fn fen_trng(&mut self) -> FenTrngW<'_, RegFenGrp0Spec> {
        FenTrngW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: enable ppe module 0: reset No use"]
    #[inline(always)]
    pub fn fen_gmac(&mut self) -> FenGmacW<'_, RegFenGrp0Spec> {
        FenGmacW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: enable SPORT function 0: disable"]
    #[inline(always)]
    pub fn fen_sport(&mut self) -> FenSportW<'_, RegFenGrp0Spec> {
        FenSportW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: enable sdio host function 0: disable"]
    #[inline(always)]
    pub fn fen_sdh(&mut self) -> FenSdhW<'_, RegFenGrp0Spec> {
        FenSdhW::new(self, 21)
    }
    #[doc = "Bit 22 - 1: enable sdio device function 0: disable"]
    #[inline(always)]
    pub fn fen_sdd(&mut self) -> FenSddW<'_, RegFenGrp0Spec> {
        FenSddW::new(self, 22)
    }
    #[doc = "Bits 23:27 - 1: enable PWM TIMER function 0: disable \\[4:1\\]:pwm \\[0\\]:pulse"]
    #[inline(always)]
    pub fn fen_pwm(&mut self) -> FenPwmW<'_, RegFenGrp0Spec> {
        FenPwmW::new(self, 23)
    }
    #[doc = "Bit 28 - 1: enable pke module 0: reset No use"]
    #[inline(always)]
    pub fn fen_pke(&mut self) -> FenPkeW<'_, RegFenGrp0Spec> {
        FenPkeW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fen_grp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fen_grp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFenGrp0Spec;
impl crate::RegisterSpec for RegFenGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fen_grp0::R`](R) reader structure"]
impl crate::Readable for RegFenGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fen_grp0::W`](W) writer structure"]
impl crate::Writable for RegFenGrp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FEN_GRP0 to value 0x0060_0000"]
impl crate::Resettable for RegFenGrp0Spec {
    const RESET_VALUE: u32 = 0x0060_0000;
}
