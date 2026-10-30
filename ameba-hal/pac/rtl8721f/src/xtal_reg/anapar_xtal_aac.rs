#[doc = "Register `ANAPAR_XTAL_AAC` reader"]
pub type R = crate::R<AnaparXtalAacSpec>;
#[doc = "Register `ANAPAR_XTAL_AAC` writer"]
pub type W = crate::W<AnaparXtalAacSpec>;
#[doc = "Field `reset_n_aac` reader - AAC digital circuit reset (low to high) Default=0, only released when EN_AAC is set (not used)"]
pub type ResetNAacR = crate::BitReader;
#[doc = "Field `reset_n_aac` writer - AAC digital circuit reset (low to high) Default=0, only released when EN_AAC is set (not used)"]
pub type ResetNAacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET_PLUS` reader - 0: aac calibrated gm result -offset , 1:aac calibrated gm result + offset"]
pub type OffsetPlusR = crate::BitReader;
#[doc = "Field `OFFSET_PLUS` writer - 0: aac calibrated gm result -offset , 1:aac calibrated gm result + offset"]
pub type OffsetPlusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XAAC_GM_OFFSET` reader - aac calibrated gm result +/- offset"]
pub type XaacGmOffsetR = crate::FieldReader;
#[doc = "Field `XAAC_GM_OFFSET` writer - aac calibrated gm result +/- offset"]
pub type XaacGmOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GM_STEP` reader - aac gm searching step. 0:step 1, 1:step 2"]
pub type GmStepR = crate::BitReader;
#[doc = "Field `GM_STEP` writer - aac gm searching step. 0:step 1, 1:step 2"]
pub type GmStepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GM_INIT` reader - aac gm initial setting"]
pub type GmInitR = crate::FieldReader;
#[doc = "Field `GM_INIT` writer - aac gm initial setting"]
pub type GmInitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `XTAL_CK_SET` reader - aac settling cycles (4M CLK) 000: 64us 001: 128us 010: 256us 011: 512us 100: 1024us 101: 2048us 110: 4096us 111: 8192us"]
pub type XtalCkSetR = crate::FieldReader;
#[doc = "Field `XTAL_CK_SET` writer - aac settling cycles (4M CLK) 000: 64us 001: 128us 010: 256us 011: 512us 100: 1024us 101: 2048us 110: 4096us 111: 8192us"]
pub type XtalCkSetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GM_STUP` reader - aac gm start up setting (when pkdet=0, returns to max gm)"]
pub type GmStupR = crate::FieldReader;
#[doc = "Field `GM_STUP` writer - aac gm start up setting (when pkdet=0, returns to max gm)"]
pub type GmStupW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EN_XTAL_AAC_DIGI` reader - 1:enable aac digital circuit"]
pub type EnXtalAacDigiR = crate::BitReader;
#[doc = "Field `EN_XTAL_AAC_DIGI` writer - 1:enable aac digital circuit"]
pub type EnXtalAacDigiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_AAC` reader - Internal MAC signal (currently floating) 0: do not perform AAC (default), 1: perform AAC on power on"]
pub type EnXtalAacR = crate::BitReader;
#[doc = "Field `EN_XTAL_AAC` writer - Internal MAC signal (currently floating) 0: do not perform AAC (default), 1: perform AAC on power on"]
pub type EnXtalAacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_AAC_TRIG` reader - Internal MAC signal (currently floating) Set by pmc and clr by HW finish calibration"]
pub type EnXtalAacTrigR = crate::BitReader;
#[doc = "Field `EN_XTAL_AAC_TRIG` writer - Internal MAC signal (currently floating) Set by pmc and clr by HW finish calibration"]
pub type EnXtalAacTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XAACK_AAC_MODE` reader - 11: XTAL_AACK_PK_SEL=<11>'s XTAL_GM_OUT3 10: XTAL_AACK_PK_SEL=<10>'s XTAL_GM_OUT2 01: XTAL_AACK_PK_SEL=<01>'s XTAL_GM_OUT1 00: XTAL_AACK_PK_SEL=<00>'s XTAL_GM_OUT0"]
pub type XaackAacModeR = crate::FieldReader;
#[doc = "Field `XAACK_AAC_MODE` writer - 11: XTAL_AACK_PK_SEL=<11>'s XTAL_GM_OUT3 10: XTAL_AACK_PK_SEL=<10>'s XTAL_GM_OUT2 01: XTAL_AACK_PK_SEL=<01>'s XTAL_GM_OUT1 00: XTAL_AACK_PK_SEL=<00>'s XTAL_GM_OUT0"]
pub type XaackAacModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - AAC digital circuit reset (low to high) Default=0, only released when EN_AAC is set (not used)"]
    #[inline(always)]
    pub fn reset_n_aac(&self) -> ResetNAacR {
        ResetNAacR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: aac calibrated gm result -offset , 1:aac calibrated gm result + offset"]
    #[inline(always)]
    pub fn offset_plus(&self) -> OffsetPlusR {
        OffsetPlusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - aac calibrated gm result +/- offset"]
    #[inline(always)]
    pub fn xaac_gm_offset(&self) -> XaacGmOffsetR {
        XaacGmOffsetR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - aac gm searching step. 0:step 1, 1:step 2"]
    #[inline(always)]
    pub fn gm_step(&self) -> GmStepR {
        GmStepR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - aac gm initial setting"]
    #[inline(always)]
    pub fn gm_init(&self) -> GmInitR {
        GmInitR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:16 - aac settling cycles (4M CLK) 000: 64us 001: 128us 010: 256us 011: 512us 100: 1024us 101: 2048us 110: 4096us 111: 8192us"]
    #[inline(always)]
    pub fn xtal_ck_set(&self) -> XtalCkSetR {
        XtalCkSetR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:21 - aac gm start up setting (when pkdet=0, returns to max gm)"]
    #[inline(always)]
    pub fn gm_stup(&self) -> GmStupR {
        GmStupR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - 1:enable aac digital circuit"]
    #[inline(always)]
    pub fn en_xtal_aac_digi(&self) -> EnXtalAacDigiR {
        EnXtalAacDigiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Internal MAC signal (currently floating) 0: do not perform AAC (default), 1: perform AAC on power on"]
    #[inline(always)]
    pub fn en_xtal_aac(&self) -> EnXtalAacR {
        EnXtalAacR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Internal MAC signal (currently floating) Set by pmc and clr by HW finish calibration"]
    #[inline(always)]
    pub fn en_xtal_aac_trig(&self) -> EnXtalAacTrigR {
        EnXtalAacTrigR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 11: XTAL_AACK_PK_SEL=<11>'s XTAL_GM_OUT3 10: XTAL_AACK_PK_SEL=<10>'s XTAL_GM_OUT2 01: XTAL_AACK_PK_SEL=<01>'s XTAL_GM_OUT1 00: XTAL_AACK_PK_SEL=<00>'s XTAL_GM_OUT0"]
    #[inline(always)]
    pub fn xaack_aac_mode(&self) -> XaackAacModeR {
        XaackAacModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - AAC digital circuit reset (low to high) Default=0, only released when EN_AAC is set (not used)"]
    #[inline(always)]
    pub fn reset_n_aac(&mut self) -> ResetNAacW<'_, AnaparXtalAacSpec> {
        ResetNAacW::new(self, 1)
    }
    #[doc = "Bit 2 - 0: aac calibrated gm result -offset , 1:aac calibrated gm result + offset"]
    #[inline(always)]
    pub fn offset_plus(&mut self) -> OffsetPlusW<'_, AnaparXtalAacSpec> {
        OffsetPlusW::new(self, 2)
    }
    #[doc = "Bits 3:7 - aac calibrated gm result +/- offset"]
    #[inline(always)]
    pub fn xaac_gm_offset(&mut self) -> XaacGmOffsetW<'_, AnaparXtalAacSpec> {
        XaacGmOffsetW::new(self, 3)
    }
    #[doc = "Bit 8 - aac gm searching step. 0:step 1, 1:step 2"]
    #[inline(always)]
    pub fn gm_step(&mut self) -> GmStepW<'_, AnaparXtalAacSpec> {
        GmStepW::new(self, 8)
    }
    #[doc = "Bits 9:13 - aac gm initial setting"]
    #[inline(always)]
    pub fn gm_init(&mut self) -> GmInitW<'_, AnaparXtalAacSpec> {
        GmInitW::new(self, 9)
    }
    #[doc = "Bits 14:16 - aac settling cycles (4M CLK) 000: 64us 001: 128us 010: 256us 011: 512us 100: 1024us 101: 2048us 110: 4096us 111: 8192us"]
    #[inline(always)]
    pub fn xtal_ck_set(&mut self) -> XtalCkSetW<'_, AnaparXtalAacSpec> {
        XtalCkSetW::new(self, 14)
    }
    #[doc = "Bits 17:21 - aac gm start up setting (when pkdet=0, returns to max gm)"]
    #[inline(always)]
    pub fn gm_stup(&mut self) -> GmStupW<'_, AnaparXtalAacSpec> {
        GmStupW::new(self, 17)
    }
    #[doc = "Bit 27 - 1:enable aac digital circuit"]
    #[inline(always)]
    pub fn en_xtal_aac_digi(&mut self) -> EnXtalAacDigiW<'_, AnaparXtalAacSpec> {
        EnXtalAacDigiW::new(self, 27)
    }
    #[doc = "Bit 28 - Internal MAC signal (currently floating) 0: do not perform AAC (default), 1: perform AAC on power on"]
    #[inline(always)]
    pub fn en_xtal_aac(&mut self) -> EnXtalAacW<'_, AnaparXtalAacSpec> {
        EnXtalAacW::new(self, 28)
    }
    #[doc = "Bit 29 - Internal MAC signal (currently floating) Set by pmc and clr by HW finish calibration"]
    #[inline(always)]
    pub fn en_xtal_aac_trig(&mut self) -> EnXtalAacTrigW<'_, AnaparXtalAacSpec> {
        EnXtalAacTrigW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 11: XTAL_AACK_PK_SEL=<11>'s XTAL_GM_OUT3 10: XTAL_AACK_PK_SEL=<10>'s XTAL_GM_OUT2 01: XTAL_AACK_PK_SEL=<01>'s XTAL_GM_OUT1 00: XTAL_AACK_PK_SEL=<00>'s XTAL_GM_OUT0"]
    #[inline(always)]
    pub fn xaack_aac_mode(&mut self) -> XaackAacModeW<'_, AnaparXtalAacSpec> {
        XaackAacModeW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_aac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_aac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalAacSpec;
impl crate::RegisterSpec for AnaparXtalAacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_aac::R`](R) reader structure"]
impl crate::Readable for AnaparXtalAacSpec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_aac::W`](W) writer structure"]
impl crate::Writable for AnaparXtalAacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_AAC to value 0xc00e_4e04"]
impl crate::Resettable for AnaparXtalAacSpec {
    const RESET_VALUE: u32 = 0xc00e_4e04;
}
