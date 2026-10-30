#[doc = "Register `ANAPAR_XTAL_OFF_1` reader"]
pub type R = crate::R<AnaparXtalOff1Spec>;
#[doc = "Register `ANAPAR_XTAL_OFF_1` writer"]
pub type W = crate::W<AnaparXtalOff1Spec>;
#[doc = "Field `XTAL_PDCK_MANU` reader - 1: open analog AAC loop for PDCK manual"]
pub type XtalPdckManuR = crate::BitReader;
#[doc = "Field `XTAL_PDCK_MANU` writer - 1: open analog AAC loop for PDCK manual"]
pub type XtalPdckManuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_LPS_BUF_VB` reader - 00: small current, 11: large current"]
pub type XtalLpsBufVbR = crate::FieldReader;
#[doc = "Field `XTAL_LPS_BUF_VB` writer - 00: small current, 11: large current"]
pub type XtalLpsBufVbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_PDSW` reader - XTAL analog AAC peak detector source follower size, 00: small 11: large"]
pub type XtalPdswR = crate::FieldReader;
#[doc = "Field `XTAL_PDSW` writer - XTAL analog AAC peak detector source follower size, 00: small 11: large"]
pub type XtalPdswW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_AAC_CAP` reader - Analog AAC domain pole cap 3pF/step 00: no cap,11: max cap"]
pub type XtalAacCapR = crate::FieldReader;
#[doc = "Field `XTAL_AAC_CAP` writer - Analog AAC domain pole cap 3pF/step 00: no cap,11: max cap"]
pub type XtalAacCapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_AAC_IOFFSET` reader - Analog AAC IOFFSET current 00: off, 01:2uA, 10:4uA, 11:6uA"]
pub type XtalAacIoffsetR = crate::FieldReader;
#[doc = "Field `XTAL_AAC_IOFFSET` writer - Analog AAC IOFFSET current 00: off, 01:2uA, 10:4uA, 11:6uA"]
pub type XtalAacIoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_AAC_TIE_MID` reader - XTAL_ AAC_TIE_MID=1 & EN_XTAL_PDCK_VREF=0 & using XTAL_VREF_SEL<4:0> for lock detect"]
pub type XtalAacTieMidR = crate::BitReader;
#[doc = "Field `XTAL_AAC_TIE_MID` writer - XTAL_ AAC_TIE_MID=1 & EN_XTAL_PDCK_VREF=0 & using XTAL_VREF_SEL<4:0> for lock detect"]
pub type XtalAacTieMidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_EN_LNBUF` reader - 0: separate bias buffer (lower current higher noise) 1: self-bias (higher current lower noise)"]
pub type XtalEnLnbufR = crate::BitReader;
#[doc = "Field `XTAL_EN_LNBUF` writer - 0: separate bias buffer (lower current higher noise) 1: self-bias (higher current lower noise)"]
pub type XtalEnLnbufW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: open analog AAC loop for PDCK manual"]
    #[inline(always)]
    pub fn xtal_pdck_manu(&self) -> XtalPdckManuR {
        XtalPdckManuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 00: small current, 11: large current"]
    #[inline(always)]
    pub fn xtal_lps_buf_vb(&self) -> XtalLpsBufVbR {
        XtalLpsBufVbR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - XTAL analog AAC peak detector source follower size, 00: small 11: large"]
    #[inline(always)]
    pub fn xtal_pdsw(&self) -> XtalPdswR {
        XtalPdswR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Analog AAC domain pole cap 3pF/step 00: no cap,11: max cap"]
    #[inline(always)]
    pub fn xtal_aac_cap(&self) -> XtalAacCapR {
        XtalAacCapR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - Analog AAC IOFFSET current 00: off, 01:2uA, 10:4uA, 11:6uA"]
    #[inline(always)]
    pub fn xtal_aac_ioffset(&self) -> XtalAacIoffsetR {
        XtalAacIoffsetR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - XTAL_ AAC_TIE_MID=1 & EN_XTAL_PDCK_VREF=0 & using XTAL_VREF_SEL<4:0> for lock detect"]
    #[inline(always)]
    pub fn xtal_aac_tie_mid(&self) -> XtalAacTieMidR {
        XtalAacTieMidR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: separate bias buffer (lower current higher noise) 1: self-bias (higher current lower noise)"]
    #[inline(always)]
    pub fn xtal_en_lnbuf(&self) -> XtalEnLnbufR {
        XtalEnLnbufR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: open analog AAC loop for PDCK manual"]
    #[inline(always)]
    pub fn xtal_pdck_manu(&mut self) -> XtalPdckManuW<'_, AnaparXtalOff1Spec> {
        XtalPdckManuW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 00: small current, 11: large current"]
    #[inline(always)]
    pub fn xtal_lps_buf_vb(&mut self) -> XtalLpsBufVbW<'_, AnaparXtalOff1Spec> {
        XtalLpsBufVbW::new(self, 1)
    }
    #[doc = "Bits 3:4 - XTAL analog AAC peak detector source follower size, 00: small 11: large"]
    #[inline(always)]
    pub fn xtal_pdsw(&mut self) -> XtalPdswW<'_, AnaparXtalOff1Spec> {
        XtalPdswW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Analog AAC domain pole cap 3pF/step 00: no cap,11: max cap"]
    #[inline(always)]
    pub fn xtal_aac_cap(&mut self) -> XtalAacCapW<'_, AnaparXtalOff1Spec> {
        XtalAacCapW::new(self, 5)
    }
    #[doc = "Bits 7:8 - Analog AAC IOFFSET current 00: off, 01:2uA, 10:4uA, 11:6uA"]
    #[inline(always)]
    pub fn xtal_aac_ioffset(&mut self) -> XtalAacIoffsetW<'_, AnaparXtalOff1Spec> {
        XtalAacIoffsetW::new(self, 7)
    }
    #[doc = "Bit 9 - XTAL_ AAC_TIE_MID=1 & EN_XTAL_PDCK_VREF=0 & using XTAL_VREF_SEL<4:0> for lock detect"]
    #[inline(always)]
    pub fn xtal_aac_tie_mid(&mut self) -> XtalAacTieMidW<'_, AnaparXtalOff1Spec> {
        XtalAacTieMidW::new(self, 9)
    }
    #[doc = "Bit 10 - 0: separate bias buffer (lower current higher noise) 1: self-bias (higher current lower noise)"]
    #[inline(always)]
    pub fn xtal_en_lnbuf(&mut self) -> XtalEnLnbufW<'_, AnaparXtalOff1Spec> {
        XtalEnLnbufW::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_off_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_off_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalOff1Spec;
impl crate::RegisterSpec for AnaparXtalOff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_off_1::R`](R) reader structure"]
impl crate::Readable for AnaparXtalOff1Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_off_1::W`](W) writer structure"]
impl crate::Writable for AnaparXtalOff1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_OFF_1 to value 0x68"]
impl crate::Resettable for AnaparXtalOff1Spec {
    const RESET_VALUE: u32 = 0x68;
}
