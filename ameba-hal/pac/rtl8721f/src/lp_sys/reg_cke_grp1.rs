#[doc = "Register `REG_CKE_GRP1` reader"]
pub type R = crate::R<RegCkeGrp1Spec>;
#[doc = "Register `REG_CKE_GRP1` writer"]
pub type W = crate::W<RegCkeGrp1Spec>;
#[doc = "Field `cke_wlon` reader - 1: Enable WLON clock 0: Disable"]
pub type CkeWlonR = crate::BitReader;
#[doc = "Field `cke_wlon` writer - 1: Enable WLON clock 0: Disable"]
pub type CkeWlonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_bton` reader - 1: Enable BTON clock 0: Disable"]
pub type CkeBtonR = crate::BitReader;
#[doc = "Field `cke_bton` writer - 1: Enable BTON clock 0: Disable"]
pub type CkeBtonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_wdg` reader - 1: Enable wdg bus clock 0: Disable"]
pub type CkeWdgR = crate::BitReader;
#[doc = "Field `cke_wdg` writer - 1: Enable wdg bus clock 0: Disable"]
pub type CkeWdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_sic` reader - 1: Enable SIC clock 0: Disable"]
pub type CkeSicR = crate::BitReader;
#[doc = "Field `cke_sic` writer - 1: Enable SIC clock 0: Disable"]
pub type CkeSicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_gpio` reader - 1: Enable GPIO clock 0: Disable"]
pub type CkeGpioR = crate::BitReader;
#[doc = "Field `cke_gpio` writer - 1: Enable GPIO clock 0: Disable"]
pub type CkeGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_loguart` reader - 1: Enable Uartlog clock 0: Disable"]
pub type CkeLoguartR = crate::BitReader;
#[doc = "Field `cke_loguart` writer - 1: Enable Uartlog clock 0: Disable"]
pub type CkeLoguartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_uart` reader - 1: Enable Uart clock 0: Disable"]
pub type CkeUartR = crate::FieldReader;
#[doc = "Field `cke_uart` writer - 1: Enable Uart clock 0: Disable"]
pub type CkeUartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cke_i2c` reader - 1: Enable HS I2C0 clock 0: Disable \\[1\\]: i2c1 \\[0\\]: i2c0"]
pub type CkeI2cR = crate::FieldReader;
#[doc = "Field `cke_i2c` writer - 1: Enable HS I2C0 clock 0: Disable \\[1\\]: i2c1 \\[0\\]: i2c0"]
pub type CkeI2cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cke_ltim` reader - LS basic Timer0-5 clk enable 0: Disable"]
pub type CkeLtimR = crate::FieldReader;
#[doc = "Field `cke_ltim` writer - LS basic Timer0-5 clk enable 0: Disable"]
pub type CkeLtimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cke_ac` reader - 1: Enable PDM(ac) clock 0: Disable"]
pub type CkeAcR = crate::BitReader;
#[doc = "Field `cke_ac` writer - 1: Enable PDM(ac) clock 0: Disable"]
pub type CkeAcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_dtim` reader - 1: Enable LS dtim clk 0: Disable"]
pub type CkeDtimR = crate::BitReader;
#[doc = "Field `cke_dtim` writer - 1: Enable LS dtim clk 0: Disable"]
pub type CkeDtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_gmac` reader - 1: Enable gmac clock 0: Disable"]
pub type CkeGmacR = crate::BitReader;
#[doc = "Field `cke_gmac` writer - 1: Enable gmac clock 0: Disable"]
pub type CkeGmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_usb` reader - 1: Enable usb core clock 0: Disable"]
pub type CkeUsbR = crate::BitReader;
#[doc = "Field `cke_usb` writer - 1: Enable usb core clock 0: Disable"]
pub type CkeUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_adc` reader - 1: Enable LS ADC clock 0: Disable"]
pub type CkeAdcR = crate::BitReader;
#[doc = "Field `cke_adc` writer - 1: Enable LS ADC clock 0: Disable"]
pub type CkeAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_thm` reader - 1: Enable thm clock 0: Disable"]
pub type CkeThmR = crate::BitReader;
#[doc = "Field `cke_thm` writer - 1: Enable thm clock 0: Disable"]
pub type CkeThmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_ptim` reader - 1: Enable pmc timer clock 0: Disable"]
pub type CkePtimR = crate::FieldReader;
#[doc = "Field `cke_ptim` writer - 1: Enable pmc timer clock 0: Disable"]
pub type CkePtimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cke_wmac` reader - 1: Enable WMAC AXI and AHB bus clock 0: Disable"]
pub type CkeWmacR = crate::BitReader;
#[doc = "Field `cke_wmac` writer - 1: Enable WMAC AXI and AHB bus clock 0: Disable"]
pub type CkeWmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_irda` reader - 1: Enable irda clock 0: Disable"]
pub type CkeIrdaR = crate::BitReader;
#[doc = "Field `cke_irda` writer - 1: Enable irda clock 0: Disable"]
pub type CkeIrdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_ppe` reader - 1: Enable ppe clock 0: Disable"]
pub type CkePpeR = crate::BitReader;
#[doc = "Field `cke_ppe` writer - 1: Enable ppe clock 0: Disable"]
pub type CkePpeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Enable WLON clock 0: Disable"]
    #[inline(always)]
    pub fn cke_wlon(&self) -> CkeWlonR {
        CkeWlonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable BTON clock 0: Disable"]
    #[inline(always)]
    pub fn cke_bton(&self) -> CkeBtonR {
        CkeBtonR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable wdg bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_wdg(&self) -> CkeWdgR {
        CkeWdgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable SIC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sic(&self) -> CkeSicR {
        CkeSicR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable GPIO clock 0: Disable"]
    #[inline(always)]
    pub fn cke_gpio(&self) -> CkeGpioR {
        CkeGpioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Enable Uartlog clock 0: Disable"]
    #[inline(always)]
    pub fn cke_loguart(&self) -> CkeLoguartR {
        CkeLoguartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - 1: Enable Uart clock 0: Disable"]
    #[inline(always)]
    pub fn cke_uart(&self) -> CkeUartR {
        CkeUartR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - 1: Enable HS I2C0 clock 0: Disable \\[1\\]: i2c1 \\[0\\]: i2c0"]
    #[inline(always)]
    pub fn cke_i2c(&self) -> CkeI2cR {
        CkeI2cR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - LS basic Timer0-5 clk enable 0: Disable"]
    #[inline(always)]
    pub fn cke_ltim(&self) -> CkeLtimR {
        CkeLtimR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - 1: Enable PDM(ac) clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ac(&self) -> CkeAcR {
        CkeAcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Enable LS dtim clk 0: Disable"]
    #[inline(always)]
    pub fn cke_dtim(&self) -> CkeDtimR {
        CkeDtimR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Enable gmac clock 0: Disable"]
    #[inline(always)]
    pub fn cke_gmac(&self) -> CkeGmacR {
        CkeGmacR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Enable usb core clock 0: Disable"]
    #[inline(always)]
    pub fn cke_usb(&self) -> CkeUsbR {
        CkeUsbR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Enable LS ADC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_adc(&self) -> CkeAdcR {
        CkeAdcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Enable thm clock 0: Disable"]
    #[inline(always)]
    pub fn cke_thm(&self) -> CkeThmR {
        CkeThmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 1: Enable pmc timer clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ptim(&self) -> CkePtimR {
        CkePtimR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - 1: Enable WMAC AXI and AHB bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_wmac(&self) -> CkeWmacR {
        CkeWmacR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Enable irda clock 0: Disable"]
    #[inline(always)]
    pub fn cke_irda(&self) -> CkeIrdaR {
        CkeIrdaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Enable ppe clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ppe(&self) -> CkePpeR {
        CkePpeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable WLON clock 0: Disable"]
    #[inline(always)]
    pub fn cke_wlon(&mut self) -> CkeWlonW<'_, RegCkeGrp1Spec> {
        CkeWlonW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable BTON clock 0: Disable"]
    #[inline(always)]
    pub fn cke_bton(&mut self) -> CkeBtonW<'_, RegCkeGrp1Spec> {
        CkeBtonW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Enable wdg bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_wdg(&mut self) -> CkeWdgW<'_, RegCkeGrp1Spec> {
        CkeWdgW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Enable SIC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_sic(&mut self) -> CkeSicW<'_, RegCkeGrp1Spec> {
        CkeSicW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Enable GPIO clock 0: Disable"]
    #[inline(always)]
    pub fn cke_gpio(&mut self) -> CkeGpioW<'_, RegCkeGrp1Spec> {
        CkeGpioW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Enable Uartlog clock 0: Disable"]
    #[inline(always)]
    pub fn cke_loguart(&mut self) -> CkeLoguartW<'_, RegCkeGrp1Spec> {
        CkeLoguartW::new(self, 5)
    }
    #[doc = "Bits 6:9 - 1: Enable Uart clock 0: Disable"]
    #[inline(always)]
    pub fn cke_uart(&mut self) -> CkeUartW<'_, RegCkeGrp1Spec> {
        CkeUartW::new(self, 6)
    }
    #[doc = "Bits 10:11 - 1: Enable HS I2C0 clock 0: Disable \\[1\\]: i2c1 \\[0\\]: i2c0"]
    #[inline(always)]
    pub fn cke_i2c(&mut self) -> CkeI2cW<'_, RegCkeGrp1Spec> {
        CkeI2cW::new(self, 10)
    }
    #[doc = "Bits 12:15 - LS basic Timer0-5 clk enable 0: Disable"]
    #[inline(always)]
    pub fn cke_ltim(&mut self) -> CkeLtimW<'_, RegCkeGrp1Spec> {
        CkeLtimW::new(self, 12)
    }
    #[doc = "Bit 18 - 1: Enable PDM(ac) clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ac(&mut self) -> CkeAcW<'_, RegCkeGrp1Spec> {
        CkeAcW::new(self, 18)
    }
    #[doc = "Bit 20 - 1: Enable LS dtim clk 0: Disable"]
    #[inline(always)]
    pub fn cke_dtim(&mut self) -> CkeDtimW<'_, RegCkeGrp1Spec> {
        CkeDtimW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Enable gmac clock 0: Disable"]
    #[inline(always)]
    pub fn cke_gmac(&mut self) -> CkeGmacW<'_, RegCkeGrp1Spec> {
        CkeGmacW::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Enable usb core clock 0: Disable"]
    #[inline(always)]
    pub fn cke_usb(&mut self) -> CkeUsbW<'_, RegCkeGrp1Spec> {
        CkeUsbW::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Enable LS ADC clock 0: Disable"]
    #[inline(always)]
    pub fn cke_adc(&mut self) -> CkeAdcW<'_, RegCkeGrp1Spec> {
        CkeAdcW::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Enable thm clock 0: Disable"]
    #[inline(always)]
    pub fn cke_thm(&mut self) -> CkeThmW<'_, RegCkeGrp1Spec> {
        CkeThmW::new(self, 24)
    }
    #[doc = "Bits 25:26 - 1: Enable pmc timer clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ptim(&mut self) -> CkePtimW<'_, RegCkeGrp1Spec> {
        CkePtimW::new(self, 25)
    }
    #[doc = "Bit 27 - 1: Enable WMAC AXI and AHB bus clock 0: Disable"]
    #[inline(always)]
    pub fn cke_wmac(&mut self) -> CkeWmacW<'_, RegCkeGrp1Spec> {
        CkeWmacW::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Enable irda clock 0: Disable"]
    #[inline(always)]
    pub fn cke_irda(&mut self) -> CkeIrdaW<'_, RegCkeGrp1Spec> {
        CkeIrdaW::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Enable ppe clock 0: Disable"]
    #[inline(always)]
    pub fn cke_ppe(&mut self) -> CkePpeW<'_, RegCkeGrp1Spec> {
        CkePpeW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cke_grp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cke_grp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkeGrp1Spec;
impl crate::RegisterSpec for RegCkeGrp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cke_grp1::R`](R) reader structure"]
impl crate::Readable for RegCkeGrp1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_cke_grp1::W`](W) writer structure"]
impl crate::Writable for RegCkeGrp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKE_GRP1 to value 0x0170_0004"]
impl crate::Resettable for RegCkeGrp1Spec {
    const RESET_VALUE: u32 = 0x0170_0004;
}
