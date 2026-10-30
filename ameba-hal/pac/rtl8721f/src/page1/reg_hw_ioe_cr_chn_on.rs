#[doc = "Register `REG_HW_IOE_CR_CHN_ON` reader"]
pub type R = crate::R<RegHwIoeCrChnOnSpec>;
#[doc = "Register `REG_HW_IOE_CR_CHN_ON` writer"]
pub type W = crate::W<RegHwIoeCrChnOnSpec>;
#[doc = "Field `BIT_HIOE_STR_ADDR_CHN_ON` reader - HIOE command start address in txpktbuffer, unit:8byte"]
pub type BitHioeStrAddrChnOnR = crate::FieldReader<u16>;
#[doc = "Field `BIT_HIOE_STR_ADDR_CHN_ON` writer - HIOE command start address in txpktbuffer, unit:8byte"]
pub type BitHioeStrAddrChnOnW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HIOE_END_ADDR_CHN_ON` reader - HIOE command end address in txpktbuffer, unit:8byte"]
pub type BitHioeEndAddrChnOnR = crate::FieldReader<u16>;
#[doc = "Field `BIT_HIOE_END_ADDR_CHN_ON` writer - HIOE command end address in txpktbuffer, unit:8byte"]
pub type BitHioeEndAddrChnOnW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HIOE_RESTORE_REQ_CHN_ON` reader - WLOFF REG restore request, clear by hw"]
pub type BitHioeRestoreReqChnOnR = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_HIOE_ACTIVE_CHN_ON` reader - indicate HIOE engine is active, used for wlan power on flow"]
pub type BitHioeActiveChnOnR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - HIOE command start address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_str_addr_chn_on(&self) -> BitHioeStrAddrChnOnR {
        BitHioeStrAddrChnOnR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - HIOE command end address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_end_addr_chn_on(&self) -> BitHioeEndAddrChnOnR {
        BitHioeEndAddrChnOnR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 25 - WLOFF REG restore request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_restore_req_chn_on(&self) -> BitHioeRestoreReqChnOnR {
        BitHioeRestoreReqChnOnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 27:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - indicate HIOE engine is active, used for wlan power on flow"]
    #[inline(always)]
    pub fn bit_hioe_active_chn_on(&self) -> BitHioeActiveChnOnR {
        BitHioeActiveChnOnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - HIOE command start address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_str_addr_chn_on(&mut self) -> BitHioeStrAddrChnOnW<'_, RegHwIoeCrChnOnSpec> {
        BitHioeStrAddrChnOnW::new(self, 0)
    }
    #[doc = "Bits 12:23 - HIOE command end address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_end_addr_chn_on(&mut self) -> BitHioeEndAddrChnOnW<'_, RegHwIoeCrChnOnSpec> {
        BitHioeEndAddrChnOnW::new(self, 12)
    }
    #[doc = "Bits 27:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegHwIoeCrChnOnSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "HW IO Engine Configure Register_CHN_ON\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hw_ioe_cr_chn_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hw_ioe_cr_chn_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHwIoeCrChnOnSpec;
impl crate::RegisterSpec for RegHwIoeCrChnOnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hw_ioe_cr_chn_on::R`](R) reader structure"]
impl crate::Readable for RegHwIoeCrChnOnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_hw_ioe_cr_chn_on::W`](W) writer structure"]
impl crate::Writable for RegHwIoeCrChnOnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HW_IOE_CR_CHN_ON to value 0"]
impl crate::Resettable for RegHwIoeCrChnOnSpec {}
