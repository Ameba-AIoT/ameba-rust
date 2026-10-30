#[doc = "Register `REG_CKE_GRP0` reader"]
pub type R = crate::R<RegCkeGrp0Spec>;
#[doc = "Register `REG_CKE_GRP0` writer"]
pub type W = crate::W<RegCkeGrp0Spec>;
#[doc = "Field `cke_lpon` reader - 1.Enable lpon clk 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
pub type CkeLponR = crate::BitReader;
#[doc = "Field `cke_lpon` writer - 1.Enable lpon clk 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
pub type CkeLponW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_lperi` reader - 1: Enable peri_lclk clock all 0: Disable"]
pub type CkeLperiR = crate::BitReader;
#[doc = "Field `cke_lperi` writer - 1: Enable peri_lclk clock all 0: Disable"]
pub type CkeLperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_hperi` reader - 1: Enable HPERI pll divider clock 0: Disable"]
pub type CkeHperiR = crate::BitReader;
#[doc = "Field `cke_hperi` writer - 1: Enable HPERI pll divider clock 0: Disable"]
pub type CkeHperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_cpu1` reader - 1: Enable lp KM4 cpu1 clock 0: Disable"]
pub type CkeCpu1R = crate::BitReader;
#[doc = "Field `cke_cpu1` writer - 1: Enable lp KM4 cpu1 clock 0: Disable"]
pub type CkeCpu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_cpu0` reader - 1: Enable hp KM4 cpu0 clock 0: Disable"]
pub type CkeCpu0R = crate::BitReader;
#[doc = "Field `cke_cpu0` writer - 1: Enable hp KM4 cpu0 clock 0: Disable"]
pub type CkeCpu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_ipc` reader - 1: Enable IPC clock 0: Disable"]
pub type CkeIpcR = crate::BitReader;
#[doc = "Field `cke_ipc` writer - 1: Enable IPC clock 0: Disable"]
pub type CkeIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_dmac` reader - 1: Enable GDMA0 clock 0: Disable"]
pub type CkeDmacR = crate::BitReader;
#[doc = "Field `cke_dmac` writer - 1: Enable GDMA0 clock 0: Disable"]
pub type CkeDmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_otpc` reader - 1: Enable OTPC clock 0: Disable"]
pub type CkeOtpcR = crate::BitReader;
#[doc = "Field `cke_otpc` writer - 1: Enable OTPC clock 0: Disable"]
pub type CkeOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_crypto` reader - 1: Enable ipsec (AES + HMAC) clock 0: Disable"]
pub type CkeCryptoR = crate::BitReader;
#[doc = "Field `cke_crypto` writer - 1: Enable ipsec (AES + HMAC) clock 0: Disable"]
pub type CkeCryptoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_lcdc` reader - 1: Enable lcdc pll clock 0: Disable Note: lcdc/mipi all use this cke"]
pub type CkeLcdcR = crate::BitReader;
#[doc = "Field `cke_lcdc` writer - 1: Enable lcdc pll clock 0: Disable Note: lcdc/mipi all use this cke"]
pub type CkeLcdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_spic` reader - 1: Enable FLASH pll divider clock 0: Disable"]
pub type CkeSpicR = crate::BitReader;
#[doc = "Field `cke_spic` writer - 1: Enable FLASH pll divider clock 0: Disable"]
pub type CkeSpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_rom` reader - 1: Enable rom clock 0: Disable"]
pub type CkeRomR = crate::BitReader;
#[doc = "Field `cke_rom` writer - 1: Enable rom clock 0: Disable"]
pub type CkeRomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_sram` reader - 1: Enable sram clock 0: Disable"]
pub type CkeSramR = crate::BitReader;
#[doc = "Field `cke_sram` writer - 1: Enable sram clock 0: Disable"]
pub type CkeSramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_psram` reader - 1: Enable PSRAM pll divider clock 0: Disable"]
pub type CkePsramR = crate::BitReader;
#[doc = "Field `cke_psram` writer - 1: Enable PSRAM pll divider clock 0: Disable"]
pub type CkePsramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_spi` reader - 1: Enable spi clock 0: Disable"]
pub type CkeSpiR = crate::FieldReader;
#[doc = "Field `cke_spi` writer - 1: Enable spi clock 0: Disable"]
pub type CkeSpiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cke_can` reader - 1: Enable can0/1 bus clock 0: Disable"]
pub type CkeCanR = crate::FieldReader;
#[doc = "Field `cke_can` writer - 1: Enable can0/1 bus clock 0: Disable"]
pub type CkeCanW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cke_trng` reader - 1: Enable trng bus clock 0: Disable"]
pub type CkeTrngR = crate::BitReader;
#[doc = "Field `cke_trng` writer - 1: Enable trng bus clock 0: Disable"]
pub type CkeTrngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_mjpeg` reader - 1: Enable mjpeg clock 0: Disable"]
pub type CkeMjpegR = crate::BitReader;
#[doc = "Field `cke_mjpeg` writer - 1: Enable mjpeg clock 0: Disable"]
pub type CkeMjpegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_sport` reader - 1: Enable sport clock 0: Disable"]
pub type CkeSportR = crate::BitReader;
#[doc = "Field `cke_sport` writer - 1: Enable sport clock 0: Disable"]
pub type CkeSportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_shperi` reader - 1: Enable shperi clock 0: Disable"]
pub type CkeShperiR = crate::BitReader;
#[doc = "Field `cke_shperi` writer - 1: Enable shperi clock 0: Disable"]
pub type CkeShperiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_sdd` reader - 1: Enable sdio dev clock 0: Disable"]
pub type CkeSddR = crate::BitReader;
#[doc = "Field `cke_sdd` writer - 1: Enable sdio dev clock 0: Disable"]
pub type CkeSddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_sdh` reader - 1: Enable sdio host clock 0: Disable"]
pub type CkeSdhR = crate::BitReader;
#[doc = "Field `cke_sdh` writer - 1: Enable sdio host clock 0: Disable"]
pub type CkeSdhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_pwm` reader - 1: Enable PWM TIMER clock 0: Disable \\[1\\] : pwm timer \\[0\\] : pulse timer"]
pub type CkePwmR = crate::FieldReader;
#[doc = "Field `cke_pwm` writer - 1: Enable PWM TIMER clock 0: Disable \\[1\\] : pwm timer \\[0\\] : pulse timer"]
pub type CkePwmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `cke_pke` reader - 1: enable pke clk 0: reset No use"]
pub type CkePkeR = crate::BitReader;
#[doc = "Field `cke_pke` writer - 1: enable pke clk 0: reset No use"]
pub type CkePkeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1.Enable lpon clk 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
    #[inline(always)]
    pub fn cke_lpon(&self) -> CkeLponR {
        CkeLponR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable peri_lclk clock all 0: Disable"]
    #[inline(always)]
    pub fn cke_lperi(&self) -> CkeLperiR {
        CkeLperiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable HPERI pll divider clock 0: Disable"]
    #[inline(always)]
    pub fn cke_hperi(&self) -> CkeHperiR {
        CkeHperiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable lp KM4 cpu1 clock 0: Disable"]
    #[inline(always)]
    pub fn cke_cpu1(&self) -> CkeCpu1R {
        CkeCpu1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable hp KM4 cpu0 clock 0: Disable"]
    #[inline(always)]
    pub fn cke_cpu0(&self) -> CkeCpu0R {
        CkeCpu0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Enable IPC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ipc(&self) -> CkeIpcR {
        CkeIpcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Enable GDMA0 clock 0: Disable"]
    #[inline(always)]
    pub fn cke_dmac(&self) -> CkeDmacR {
        CkeDmacR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Enable OTPC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_otpc(&self) -> CkeOtpcR {
        CkeOtpcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable ipsec (AES + HMAC) clock 0: Disable"]
    #[inline(always)]
    pub fn cke_crypto(&self) -> CkeCryptoR {
        CkeCryptoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Enable lcdc pll clock 0: Disable Note: lcdc/mipi all use this cke"]
    #[inline(always)]
    pub fn cke_lcdc(&self) -> CkeLcdcR {
        CkeLcdcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Enable FLASH pll divider clock 0: Disable"]
    #[inline(always)]
    pub fn cke_spic(&self) -> CkeSpicR {
        CkeSpicR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Enable rom clock 0: Disable"]
    #[inline(always)]
    pub fn cke_rom(&self) -> CkeRomR {
        CkeRomR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Enable sram clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sram(&self) -> CkeSramR {
        CkeSramR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Enable PSRAM pll divider clock 0: Disable"]
    #[inline(always)]
    pub fn cke_psram(&self) -> CkePsramR {
        CkePsramR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 1: Enable spi clock 0: Disable"]
    #[inline(always)]
    pub fn cke_spi(&self) -> CkeSpiR {
        CkeSpiR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 1: Enable can0/1 bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_can(&self) -> CkeCanR {
        CkeCanR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 1: Enable trng bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_trng(&self) -> CkeTrngR {
        CkeTrngR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Enable mjpeg clock 0: Disable"]
    #[inline(always)]
    pub fn cke_mjpeg(&self) -> CkeMjpegR {
        CkeMjpegR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Enable sport clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sport(&self) -> CkeSportR {
        CkeSportR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Enable shperi clock 0: Disable"]
    #[inline(always)]
    pub fn cke_shperi(&self) -> CkeShperiR {
        CkeShperiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Enable sdio dev clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sdd(&self) -> CkeSddR {
        CkeSddR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Enable sdio host clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sdh(&self) -> CkeSdhR {
        CkeSdhR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 1: Enable PWM TIMER clock 0: Disable \\[1\\] : pwm timer \\[0\\] : pulse timer"]
    #[inline(always)]
    pub fn cke_pwm(&self) -> CkePwmR {
        CkePwmR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 1: enable pke clk 0: reset No use"]
    #[inline(always)]
    pub fn cke_pke(&self) -> CkePkeR {
        CkePkeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1.Enable lpon clk 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
    #[inline(always)]
    pub fn cke_lpon(&mut self) -> CkeLponW<'_, RegCkeGrp0Spec> {
        CkeLponW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable peri_lclk clock all 0: Disable"]
    #[inline(always)]
    pub fn cke_lperi(&mut self) -> CkeLperiW<'_, RegCkeGrp0Spec> {
        CkeLperiW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Enable HPERI pll divider clock 0: Disable"]
    #[inline(always)]
    pub fn cke_hperi(&mut self) -> CkeHperiW<'_, RegCkeGrp0Spec> {
        CkeHperiW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Enable lp KM4 cpu1 clock 0: Disable"]
    #[inline(always)]
    pub fn cke_cpu1(&mut self) -> CkeCpu1W<'_, RegCkeGrp0Spec> {
        CkeCpu1W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Enable hp KM4 cpu0 clock 0: Disable"]
    #[inline(always)]
    pub fn cke_cpu0(&mut self) -> CkeCpu0W<'_, RegCkeGrp0Spec> {
        CkeCpu0W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Enable IPC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ipc(&mut self) -> CkeIpcW<'_, RegCkeGrp0Spec> {
        CkeIpcW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Enable GDMA0 clock 0: Disable"]
    #[inline(always)]
    pub fn cke_dmac(&mut self) -> CkeDmacW<'_, RegCkeGrp0Spec> {
        CkeDmacW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Enable OTPC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_otpc(&mut self) -> CkeOtpcW<'_, RegCkeGrp0Spec> {
        CkeOtpcW::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Enable ipsec (AES + HMAC) clock 0: Disable"]
    #[inline(always)]
    pub fn cke_crypto(&mut self) -> CkeCryptoW<'_, RegCkeGrp0Spec> {
        CkeCryptoW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Enable lcdc pll clock 0: Disable Note: lcdc/mipi all use this cke"]
    #[inline(always)]
    pub fn cke_lcdc(&mut self) -> CkeLcdcW<'_, RegCkeGrp0Spec> {
        CkeLcdcW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Enable FLASH pll divider clock 0: Disable"]
    #[inline(always)]
    pub fn cke_spic(&mut self) -> CkeSpicW<'_, RegCkeGrp0Spec> {
        CkeSpicW::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Enable rom clock 0: Disable"]
    #[inline(always)]
    pub fn cke_rom(&mut self) -> CkeRomW<'_, RegCkeGrp0Spec> {
        CkeRomW::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Enable sram clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sram(&mut self) -> CkeSramW<'_, RegCkeGrp0Spec> {
        CkeSramW::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Enable PSRAM pll divider clock 0: Disable"]
    #[inline(always)]
    pub fn cke_psram(&mut self) -> CkePsramW<'_, RegCkeGrp0Spec> {
        CkePsramW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 1: Enable spi clock 0: Disable"]
    #[inline(always)]
    pub fn cke_spi(&mut self) -> CkeSpiW<'_, RegCkeGrp0Spec> {
        CkeSpiW::new(self, 14)
    }
    #[doc = "Bits 16:17 - 1: Enable can0/1 bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_can(&mut self) -> CkeCanW<'_, RegCkeGrp0Spec> {
        CkeCanW::new(self, 16)
    }
    #[doc = "Bit 18 - 1: Enable trng bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_trng(&mut self) -> CkeTrngW<'_, RegCkeGrp0Spec> {
        CkeTrngW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Enable mjpeg clock 0: Disable"]
    #[inline(always)]
    pub fn cke_mjpeg(&mut self) -> CkeMjpegW<'_, RegCkeGrp0Spec> {
        CkeMjpegW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Enable sport clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sport(&mut self) -> CkeSportW<'_, RegCkeGrp0Spec> {
        CkeSportW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Enable shperi clock 0: Disable"]
    #[inline(always)]
    pub fn cke_shperi(&mut self) -> CkeShperiW<'_, RegCkeGrp0Spec> {
        CkeShperiW::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Enable sdio dev clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sdd(&mut self) -> CkeSddW<'_, RegCkeGrp0Spec> {
        CkeSddW::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Enable sdio host clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sdh(&mut self) -> CkeSdhW<'_, RegCkeGrp0Spec> {
        CkeSdhW::new(self, 23)
    }
    #[doc = "Bits 24:28 - 1: Enable PWM TIMER clock 0: Disable \\[1\\] : pwm timer \\[0\\] : pulse timer"]
    #[inline(always)]
    pub fn cke_pwm(&mut self) -> CkePwmW<'_, RegCkeGrp0Spec> {
        CkePwmW::new(self, 24)
    }
    #[doc = "Bit 29 - 1: enable pke clk 0: reset No use"]
    #[inline(always)]
    pub fn cke_pke(&mut self) -> CkePkeW<'_, RegCkeGrp0Spec> {
        CkePkeW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cke_grp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cke_grp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkeGrp0Spec;
impl crate::RegisterSpec for RegCkeGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cke_grp0::R`](R) reader structure"]
impl crate::Readable for RegCkeGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_cke_grp0::W`](W) writer structure"]
impl crate::Writable for RegCkeGrp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKE_GRP0 to value 0x20e0_1a06"]
impl crate::Resettable for RegCkeGrp0Spec {
    const RESET_VALUE: u32 = 0x20e0_1a06;
}
