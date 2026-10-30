#[doc = "Register `ANAPAR_XTAL_AAC_ON_1` reader"]
pub type R = crate::R<AnaparXtalAacOn1Spec>;
#[doc = "Register `ANAPAR_XTAL_AAC_ON_1` writer"]
pub type W = crate::W<AnaparXtalAacOn1Spec>;
#[doc = "Field `XAAC_LPOW` reader - AACK low power mode, 1:gating input clk"]
pub type XaacLpowR = crate::BitReader;
#[doc = "Field `XAAC_LPOW` writer - AACK low power mode, 1:gating input clk"]
pub type XaacLpowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XAAC_LUT_MANUAL_EN` reader - 1: enable LUT data: GM_MANUAL<4:0> , sel: AAC_MODE<1:0>"]
pub type XaacLutManualEnR = crate::BitReader;
#[doc = "Field `XAAC_LUT_MANUAL_EN` writer - 1: enable LUT data: GM_MANUAL<4:0> , sel: AAC_MODE<1:0>"]
pub type XaacLutManualEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PK_START_AR` reader - AACK peak sel start (star>end)"]
pub type PkStartArR = crate::FieldReader;
#[doc = "Field `PK_START_AR` writer - AACK peak sel start (star>end)"]
pub type PkStartArW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PK_END_AR` reader - AAC peak sel end (star>end)"]
pub type PkEndArR = crate::FieldReader;
#[doc = "Field `PK_END_AR` writer - AAC peak sel end (star>end)"]
pub type PkEndArW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XAAC_BUSY` reader - for XAAC debug"]
pub type XaacBusyR = crate::BitReader;
#[doc = "Field `XAAC_DBG` reader - for XAAC debug"]
pub type XaacDbgR = crate::FieldReader;
#[doc = "Field `XAAC_STATE` reader - for XAAC debug"]
pub type XaacStateR = crate::FieldReader;
#[doc = "Field `XAAC_DONE` reader - for XAAC debug"]
pub type XaacDoneR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - AACK low power mode, 1:gating input clk"]
    #[inline(always)]
    pub fn xaac_lpow(&self) -> XaacLpowR {
        XaacLpowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enable LUT data: GM_MANUAL<4:0> , sel: AAC_MODE<1:0>"]
    #[inline(always)]
    pub fn xaac_lut_manual_en(&self) -> XaacLutManualEnR {
        XaacLutManualEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - AACK peak sel start (star>end)"]
    #[inline(always)]
    pub fn pk_start_ar(&self) -> PkStartArR {
        PkStartArR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - AAC peak sel end (star>end)"]
    #[inline(always)]
    pub fn pk_end_ar(&self) -> PkEndArR {
        PkEndArR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - for XAAC debug"]
    #[inline(always)]
    pub fn xaac_busy(&self) -> XaacBusyR {
        XaacBusyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - for XAAC debug"]
    #[inline(always)]
    pub fn xaac_dbg(&self) -> XaacDbgR {
        XaacDbgR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - for XAAC debug"]
    #[inline(always)]
    pub fn xaac_state(&self) -> XaacStateR {
        XaacStateR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - for XAAC debug"]
    #[inline(always)]
    pub fn xaac_done(&self) -> XaacDoneR {
        XaacDoneR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - AACK low power mode, 1:gating input clk"]
    #[inline(always)]
    pub fn xaac_lpow(&mut self) -> XaacLpowW<'_, AnaparXtalAacOn1Spec> {
        XaacLpowW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enable LUT data: GM_MANUAL<4:0> , sel: AAC_MODE<1:0>"]
    #[inline(always)]
    pub fn xaac_lut_manual_en(&mut self) -> XaacLutManualEnW<'_, AnaparXtalAacOn1Spec> {
        XaacLutManualEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - AACK peak sel start (star>end)"]
    #[inline(always)]
    pub fn pk_start_ar(&mut self) -> PkStartArW<'_, AnaparXtalAacOn1Spec> {
        PkStartArW::new(self, 2)
    }
    #[doc = "Bits 4:5 - AAC peak sel end (star>end)"]
    #[inline(always)]
    pub fn pk_end_ar(&mut self) -> PkEndArW<'_, AnaparXtalAacOn1Spec> {
        PkEndArW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_aac_on_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_aac_on_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalAacOn1Spec;
impl crate::RegisterSpec for AnaparXtalAacOn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_aac_on_1::R`](R) reader structure"]
impl crate::Readable for AnaparXtalAacOn1Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_aac_on_1::W`](W) writer structure"]
impl crate::Writable for AnaparXtalAacOn1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_AAC_ON_1 to value 0x3c"]
impl crate::Resettable for AnaparXtalAacOn1Spec {
    const RESET_VALUE: u32 = 0x3c;
}
