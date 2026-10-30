#[doc = "Register `REG_FEN_GRP1` reader"]
pub type R = crate::R<RegFenGrp1Spec>;
#[doc = "Register `REG_FEN_GRP1` writer"]
pub type W = crate::W<RegFenGrp1Spec>;
#[doc = "Field `fen_wlon` reader - WLON function enable 1: enable 0: disable"]
pub type FenWlonR = crate::BitReader;
#[doc = "Field `fen_wlon` writer - WLON function enable 1: enable 0: disable"]
pub type FenWlonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_bton` reader - BTON function enable 1: enable 0: disable"]
pub type FenBtonR = crate::BitReader;
#[doc = "Field `fen_bton` writer - BTON function enable 1: enable 0: disable"]
pub type FenBtonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_wlafe_ctrl` reader - WL RFAFE control function enable 1: enable 0: disable"]
pub type FenWlafeCtrlR = crate::BitReader;
#[doc = "Field `fen_wlafe_ctrl` writer - WL RFAFE control function enable 1: enable 0: disable"]
pub type FenWlafeCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_sic` reader - 1: enable SIC function 0: reset"]
pub type FenSicR = crate::BitReader;
#[doc = "Field `fen_sic` writer - 1: enable SIC function 0: reset"]
pub type FenSicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_gpio` reader - SGPIO function enable 1: enable 0: disable"]
pub type FenGpioR = crate::BitReader;
#[doc = "Field `fen_gpio` writer - SGPIO function enable 1: enable 0: disable"]
pub type FenGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_loguart` reader - Uartlog function enable 1: enable 0: disable"]
pub type FenLoguartR = crate::BitReader;
#[doc = "Field `fen_loguart` writer - Uartlog function enable 1: enable 0: disable"]
pub type FenLoguartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_uart` reader - Uart function enable (Note uart2 if for BT) 1: enable 0: disable"]
pub type FenUartR = crate::FieldReader;
#[doc = "Field `fen_uart` writer - Uart function enable (Note uart2 if for BT) 1: enable 0: disable"]
pub type FenUartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fen_i2c` reader - I2C1 function enable 1: enable 0: disable"]
pub type FenI2cR = crate::FieldReader;
#[doc = "Field `fen_i2c` writer - I2C1 function enable 1: enable 0: disable"]
pub type FenI2cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fen_ltim` reader - Basic Timer0-3 function enable 1: enable 0: disable"]
pub type FenLtimR = crate::FieldReader;
#[doc = "Field `fen_ltim` writer - Basic Timer0-3 function enable 1: enable 0: disable"]
pub type FenLtimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fen_dtim` reader - Debug timer function enable 1: enable 0: disable"]
pub type FenDtimR = crate::BitReader;
#[doc = "Field `fen_dtim` writer - Debug timer function enable 1: enable 0: disable"]
pub type FenDtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_shrm` reader - BT share memory function enable 1: enable 0: disable Note:this is same as PD_SHRM power reset"]
pub type FenShrmR = crate::BitReader;
#[doc = "Field `fen_shrm` writer - BT share memory function enable 1: enable 0: disable Note:this is same as PD_SHRM power reset"]
pub type FenShrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_usb` reader - usb core function enable 1: enable 0: disable"]
pub type FenUsbR = crate::BitReader;
#[doc = "Field `fen_usb` writer - usb core function enable 1: enable 0: disable"]
pub type FenUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_adc` reader - lp adc function enable 1: enable 0: disable"]
pub type FenAdcR = crate::BitReader;
#[doc = "Field `fen_adc` writer - lp adc function enable 1: enable 0: disable"]
pub type FenAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_thm` reader - thermal function enable 1: enable 0: disable"]
pub type FenThmR = crate::BitReader;
#[doc = "Field `fen_thm` writer - thermal function enable 1: enable 0: disable"]
pub type FenThmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fen_ptim` reader - pmc_timer function enable 1: enable 0: disable"]
pub type FenPtimR = crate::FieldReader;
#[doc = "Field `fen_ptim` writer - pmc_timer function enable 1: enable 0: disable"]
pub type FenPtimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fen_irda` reader - 1: enable irda function 0: disable"]
pub type FenIrdaR = crate::BitReader;
#[doc = "Field `fen_irda` writer - 1: enable irda function 0: disable"]
pub type FenIrdaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WLON function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_wlon(&self) -> FenWlonR {
        FenWlonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BTON function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_bton(&self) -> FenBtonR {
        FenBtonR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WL RFAFE control function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_wlafe_ctrl(&self) -> FenWlafeCtrlR {
        FenWlafeCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: enable SIC function 0: reset"]
    #[inline(always)]
    pub fn fen_sic(&self) -> FenSicR {
        FenSicR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SGPIO function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_gpio(&self) -> FenGpioR {
        FenGpioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Uartlog function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_loguart(&self) -> FenLoguartR {
        FenLoguartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Uart function enable (Note uart2 if for BT) 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_uart(&self) -> FenUartR {
        FenUartR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - I2C1 function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_i2c(&self) -> FenI2cR {
        FenI2cR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Basic Timer0-3 function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_ltim(&self) -> FenLtimR {
        FenLtimR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Debug timer function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_dtim(&self) -> FenDtimR {
        FenDtimR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BT share memory function enable 1: enable 0: disable Note:this is same as PD_SHRM power reset"]
    #[inline(always)]
    pub fn fen_shrm(&self) -> FenShrmR {
        FenShrmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - usb core function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_usb(&self) -> FenUsbR {
        FenUsbR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - lp adc function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_adc(&self) -> FenAdcR {
        FenAdcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - thermal function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_thm(&self) -> FenThmR {
        FenThmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - pmc_timer function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_ptim(&self) -> FenPtimR {
        FenPtimR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 28 - 1: enable irda function 0: disable"]
    #[inline(always)]
    pub fn fen_irda(&self) -> FenIrdaR {
        FenIrdaR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WLON function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_wlon(&mut self) -> FenWlonW<'_, RegFenGrp1Spec> {
        FenWlonW::new(self, 0)
    }
    #[doc = "Bit 1 - BTON function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_bton(&mut self) -> FenBtonW<'_, RegFenGrp1Spec> {
        FenBtonW::new(self, 1)
    }
    #[doc = "Bit 2 - WL RFAFE control function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_wlafe_ctrl(&mut self) -> FenWlafeCtrlW<'_, RegFenGrp1Spec> {
        FenWlafeCtrlW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: enable SIC function 0: reset"]
    #[inline(always)]
    pub fn fen_sic(&mut self) -> FenSicW<'_, RegFenGrp1Spec> {
        FenSicW::new(self, 3)
    }
    #[doc = "Bit 4 - SGPIO function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_gpio(&mut self) -> FenGpioW<'_, RegFenGrp1Spec> {
        FenGpioW::new(self, 4)
    }
    #[doc = "Bit 5 - Uartlog function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_loguart(&mut self) -> FenLoguartW<'_, RegFenGrp1Spec> {
        FenLoguartW::new(self, 5)
    }
    #[doc = "Bits 6:9 - Uart function enable (Note uart2 if for BT) 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_uart(&mut self) -> FenUartW<'_, RegFenGrp1Spec> {
        FenUartW::new(self, 6)
    }
    #[doc = "Bits 10:11 - I2C1 function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_i2c(&mut self) -> FenI2cW<'_, RegFenGrp1Spec> {
        FenI2cW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Basic Timer0-3 function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_ltim(&mut self) -> FenLtimW<'_, RegFenGrp1Spec> {
        FenLtimW::new(self, 12)
    }
    #[doc = "Bit 20 - Debug timer function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_dtim(&mut self) -> FenDtimW<'_, RegFenGrp1Spec> {
        FenDtimW::new(self, 20)
    }
    #[doc = "Bit 21 - BT share memory function enable 1: enable 0: disable Note:this is same as PD_SHRM power reset"]
    #[inline(always)]
    pub fn fen_shrm(&mut self) -> FenShrmW<'_, RegFenGrp1Spec> {
        FenShrmW::new(self, 21)
    }
    #[doc = "Bit 22 - usb core function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_usb(&mut self) -> FenUsbW<'_, RegFenGrp1Spec> {
        FenUsbW::new(self, 22)
    }
    #[doc = "Bit 23 - lp adc function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_adc(&mut self) -> FenAdcW<'_, RegFenGrp1Spec> {
        FenAdcW::new(self, 23)
    }
    #[doc = "Bit 24 - thermal function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_thm(&mut self) -> FenThmW<'_, RegFenGrp1Spec> {
        FenThmW::new(self, 24)
    }
    #[doc = "Bits 25:26 - pmc_timer function enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn fen_ptim(&mut self) -> FenPtimW<'_, RegFenGrp1Spec> {
        FenPtimW::new(self, 25)
    }
    #[doc = "Bit 28 - 1: enable irda function 0: disable"]
    #[inline(always)]
    pub fn fen_irda(&mut self) -> FenIrdaW<'_, RegFenGrp1Spec> {
        FenIrdaW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fen_grp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fen_grp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFenGrp1Spec;
impl crate::RegisterSpec for RegFenGrp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fen_grp1::R`](R) reader structure"]
impl crate::Readable for RegFenGrp1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fen_grp1::W`](W) writer structure"]
impl crate::Writable for RegFenGrp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FEN_GRP1 to value 0x0150_0000"]
impl crate::Resettable for RegFenGrp1Spec {
    const RESET_VALUE: u32 = 0x0150_0000;
}
