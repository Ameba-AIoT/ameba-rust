#[doc = "Register `REG_NAN_CTL` reader"]
pub type R = crate::R<RegNanCtlSpec>;
#[doc = "Register `REG_NAN_CTL` writer"]
pub type W = crate::W<RegNanCtlSpec>;
#[doc = "Field `r_WMAC_ATTRI_ID2` reader - the NAN attribute whose ID is r_WMAC_ATTRI_ID2 will be written into NAN beacon report"]
pub type RWmacAttriId2R = crate::FieldReader;
#[doc = "Field `r_WMAC_ATTRI_ID2` writer - the NAN attribute whose ID is r_WMAC_ATTRI_ID2 will be written into NAN beacon report"]
pub type RWmacAttriId2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_WMAC_ATTRI_ID3` reader - the NAN attribute whose ID is r_WMAC_ATTRI_ID3 will be written into NAN beacon report"]
pub type RWmacAttriId3R = crate::FieldReader;
#[doc = "Field `r_WMAC_ATTRI_ID3` writer - the NAN attribute whose ID is r_WMAC_ATTRI_ID3 will be written into NAN beacon report"]
pub type RWmacAttriId3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_WMAC_ATTRI_ID4` reader - the NAN attribute whose ID is r_WMAC_ATTRI_ID4 will be written into NAN beacon report"]
pub type RWmacAttriId4R = crate::FieldReader;
#[doc = "Field `r_WMAC_ATTRI_ID4` writer - the NAN attribute whose ID is r_WMAC_ATTRI_ID4 will be written into NAN beacon report"]
pub type RWmacAttriId4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_TXRPT_NAN_ENTRY_FLAG` reader - indicate NAN beacon report(i) in txrptbuffer is valid, it is set by HW and clear by FW"]
pub type RTxrptNanEntryFlagR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - the NAN attribute whose ID is r_WMAC_ATTRI_ID2 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id2(&self) -> RWmacAttriId2R {
        RWmacAttriId2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - the NAN attribute whose ID is r_WMAC_ATTRI_ID3 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id3(&self) -> RWmacAttriId3R {
        RWmacAttriId3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - the NAN attribute whose ID is r_WMAC_ATTRI_ID4 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id4(&self) -> RWmacAttriId4R {
        RWmacAttriId4R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - indicate NAN beacon report(i) in txrptbuffer is valid, it is set by HW and clear by FW"]
    #[inline(always)]
    pub fn r_txrpt_nan_entry_flag(&self) -> RTxrptNanEntryFlagR {
        RTxrptNanEntryFlagR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - the NAN attribute whose ID is r_WMAC_ATTRI_ID2 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id2(&mut self) -> RWmacAttriId2W<'_, RegNanCtlSpec> {
        RWmacAttriId2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - the NAN attribute whose ID is r_WMAC_ATTRI_ID3 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id3(&mut self) -> RWmacAttriId3W<'_, RegNanCtlSpec> {
        RWmacAttriId3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - the NAN attribute whose ID is r_WMAC_ATTRI_ID4 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id4(&mut self) -> RWmacAttriId4W<'_, RegNanCtlSpec> {
        RWmacAttriId4W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanCtlSpec;
impl crate::RegisterSpec for RegNanCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_ctl::R`](R) reader structure"]
impl crate::Readable for RegNanCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_ctl::W`](W) writer structure"]
impl crate::Writable for RegNanCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_CTL to value 0"]
impl crate::Resettable for RegNanCtlSpec {}
