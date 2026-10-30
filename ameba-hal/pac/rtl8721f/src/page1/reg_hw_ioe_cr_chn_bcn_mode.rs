#[doc = "Register `REG_HW_IOE_CR_chn_bcn_mode` reader"]
pub type R = crate::R<RegHwIoeCrChnBcnModeSpec>;
#[doc = "Register `REG_HW_IOE_CR_chn_bcn_mode` writer"]
pub type W = crate::W<RegHwIoeCrChnBcnModeSpec>;
#[doc = "Field `BIT_HIOE_STR_ADDR_chn_bcn_mode` reader - HIOE command start address in txpktbuffer, unit:8byte"]
pub type BitHioeStrAddrChnBcnModeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_HIOE_STR_ADDR_chn_bcn_mode` writer - HIOE command start address in txpktbuffer, unit:8byte"]
pub type BitHioeStrAddrChnBcnModeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HIOE_END_ADDR_chn_bcn_mode` reader - HIOE command end address in txpktbuffer, unit:8byte"]
pub type BitHioeEndAddrChnBcnModeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_HIOE_END_ADDR_chn_bcn_mode` writer - HIOE command end address in txpktbuffer, unit:8byte"]
pub type BitHioeEndAddrChnBcnModeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HIOE_RESTORE_REQ_chn_bcn_mode` reader - WLOFF REG restore request, clear by hw"]
pub type BitHioeRestoreReqChnBcnModeR = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_HIOE_ACTIVE_chn_bcn_mode` reader - indicate HIOE engine is active, used for wlan power off flow"]
pub type BitHioeActiveChnBcnModeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - HIOE command start address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_str_addr_chn_bcn_mode(&self) -> BitHioeStrAddrChnBcnModeR {
        BitHioeStrAddrChnBcnModeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - HIOE command end address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_end_addr_chn_bcn_mode(&self) -> BitHioeEndAddrChnBcnModeR {
        BitHioeEndAddrChnBcnModeR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 25 - WLOFF REG restore request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_restore_req_chn_bcn_mode(&self) -> BitHioeRestoreReqChnBcnModeR {
        BitHioeRestoreReqChnBcnModeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 27:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - indicate HIOE engine is active, used for wlan power off flow"]
    #[inline(always)]
    pub fn bit_hioe_active_chn_bcn_mode(&self) -> BitHioeActiveChnBcnModeR {
        BitHioeActiveChnBcnModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - HIOE command start address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_str_addr_chn_bcn_mode(
        &mut self,
    ) -> BitHioeStrAddrChnBcnModeW<'_, RegHwIoeCrChnBcnModeSpec> {
        BitHioeStrAddrChnBcnModeW::new(self, 0)
    }
    #[doc = "Bits 12:23 - HIOE command end address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_end_addr_chn_bcn_mode(
        &mut self,
    ) -> BitHioeEndAddrChnBcnModeW<'_, RegHwIoeCrChnBcnModeSpec> {
        BitHioeEndAddrChnBcnModeW::new(self, 12)
    }
    #[doc = "Bits 27:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegHwIoeCrChnBcnModeSpec> {
        RsvdW::new(self, 27)
    }
}
#[doc = "HW IO Engine Configure Register_chn_bcn_mode\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hw_ioe_cr_chn_bcn_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hw_ioe_cr_chn_bcn_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHwIoeCrChnBcnModeSpec;
impl crate::RegisterSpec for RegHwIoeCrChnBcnModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hw_ioe_cr_chn_bcn_mode::R`](R) reader structure"]
impl crate::Readable for RegHwIoeCrChnBcnModeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_hw_ioe_cr_chn_bcn_mode::W`](W) writer structure"]
impl crate::Writable for RegHwIoeCrChnBcnModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HW_IOE_CR_chn_bcn_mode to value 0"]
impl crate::Resettable for RegHwIoeCrChnBcnModeSpec {}
