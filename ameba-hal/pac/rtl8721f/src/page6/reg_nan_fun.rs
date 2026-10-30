#[doc = "Register `REG_NAN_FUN` reader"]
pub type R = crate::R<RegNanFunSpec>;
#[doc = "Register `REG_NAN_FUN` writer"]
pub type W = crate::W<RegNanFunSpec>;
#[doc = "Field `r_WMAC_RXNANPSR_EN` reader - parse enable of rx NAN sync beacon"]
pub type RWmacRxnanpsrEnR = crate::BitReader;
#[doc = "Field `r_WMAC_RXNANPSR_EN` writer - parse enable of rx NAN sync beacon"]
pub type RWmacRxnanpsrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_NAN_ENTRY_SIZE` reader - size of one NAN beacon report entry, unit is 8byte"]
pub type RNanEntrySizeR = crate::FieldReader;
#[doc = "Field `r_NAN_ENTRY_SIZE` writer - size of one NAN beacon report entry, unit is 8byte"]
pub type RNanEntrySizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_TXRPT_NAN_ENTRY_NUM` reader - number of NAN beacon report entry in txrptbuffer"]
pub type RTxrptNanEntryNumR = crate::FieldReader;
#[doc = "Field `r_TXRPT_NAN_ENTRY_NUM` writer - number of NAN beacon report entry in txrptbuffer"]
pub type RTxrptNanEntryNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_WMAC_ATTRI_IDi_VLD` reader - valid flag bitmap of r_WMAC_ATTRI_IDi"]
pub type RWmacAttriIdiVldR = crate::FieldReader;
#[doc = "Field `r_WMAC_ATTRI_IDi_VLD` writer - valid flag bitmap of r_WMAC_ATTRI_IDi"]
pub type RWmacAttriIdiVldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_WMAC_ATTRI_ID0` reader - the NAN attribute whose ID is r_WMAC_ATTRI_ID0 will be written into NAN beacon report"]
pub type RWmacAttriId0R = crate::FieldReader;
#[doc = "Field `r_WMAC_ATTRI_ID0` writer - the NAN attribute whose ID is r_WMAC_ATTRI_ID0 will be written into NAN beacon report"]
pub type RWmacAttriId0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_WMAC_ATTRI_ID1` reader - the NAN attribute whose ID is r_WMAC_ATTRI_ID1 will be written into NAN beacon report"]
pub type RWmacAttriId1R = crate::FieldReader;
#[doc = "Field `r_WMAC_ATTRI_ID1` writer - the NAN attribute whose ID is r_WMAC_ATTRI_ID1 will be written into NAN beacon report"]
pub type RWmacAttriId1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - parse enable of rx NAN sync beacon"]
    #[inline(always)]
    pub fn r_wmac_rxnanpsr_en(&self) -> RWmacRxnanpsrEnR {
        RWmacRxnanpsrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - size of one NAN beacon report entry, unit is 8byte"]
    #[inline(always)]
    pub fn r_nan_entry_size(&self) -> RNanEntrySizeR {
        RNanEntrySizeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - number of NAN beacon report entry in txrptbuffer"]
    #[inline(always)]
    pub fn r_txrpt_nan_entry_num(&self) -> RTxrptNanEntryNumR {
        RTxrptNanEntryNumR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12 - valid flag bitmap of r_WMAC_ATTRI_IDi"]
    #[inline(always)]
    pub fn r_wmac_attri_idi_vld(&self) -> RWmacAttriIdiVldR {
        RWmacAttriIdiVldR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - the NAN attribute whose ID is r_WMAC_ATTRI_ID0 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id0(&self) -> RWmacAttriId0R {
        RWmacAttriId0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - the NAN attribute whose ID is r_WMAC_ATTRI_ID1 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id1(&self) -> RWmacAttriId1R {
        RWmacAttriId1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - parse enable of rx NAN sync beacon"]
    #[inline(always)]
    pub fn r_wmac_rxnanpsr_en(&mut self) -> RWmacRxnanpsrEnW<'_, RegNanFunSpec> {
        RWmacRxnanpsrEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - size of one NAN beacon report entry, unit is 8byte"]
    #[inline(always)]
    pub fn r_nan_entry_size(&mut self) -> RNanEntrySizeW<'_, RegNanFunSpec> {
        RNanEntrySizeW::new(self, 1)
    }
    #[doc = "Bits 4:6 - number of NAN beacon report entry in txrptbuffer"]
    #[inline(always)]
    pub fn r_txrpt_nan_entry_num(&mut self) -> RTxrptNanEntryNumW<'_, RegNanFunSpec> {
        RTxrptNanEntryNumW::new(self, 4)
    }
    #[doc = "Bits 8:12 - valid flag bitmap of r_WMAC_ATTRI_IDi"]
    #[inline(always)]
    pub fn r_wmac_attri_idi_vld(&mut self) -> RWmacAttriIdiVldW<'_, RegNanFunSpec> {
        RWmacAttriIdiVldW::new(self, 8)
    }
    #[doc = "Bits 16:23 - the NAN attribute whose ID is r_WMAC_ATTRI_ID0 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id0(&mut self) -> RWmacAttriId0W<'_, RegNanFunSpec> {
        RWmacAttriId0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - the NAN attribute whose ID is r_WMAC_ATTRI_ID1 will be written into NAN beacon report"]
    #[inline(always)]
    pub fn r_wmac_attri_id1(&mut self) -> RWmacAttriId1W<'_, RegNanFunSpec> {
        RWmacAttriId1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_fun::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_fun::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanFunSpec;
impl crate::RegisterSpec for RegNanFunSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_fun::R`](R) reader structure"]
impl crate::Readable for RegNanFunSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_fun::W`](W) writer structure"]
impl crate::Writable for RegNanFunSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_FUN to value 0x04"]
impl crate::Resettable for RegNanFunSpec {
    const RESET_VALUE: u32 = 0x04;
}
