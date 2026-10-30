#[doc = "Register `REG_HW_IOE_CR_CHN1` reader"]
pub type R = crate::R<RegHwIoeCrChn1Spec>;
#[doc = "Register `REG_HW_IOE_CR_CHN1` writer"]
pub type W = crate::W<RegHwIoeCrChn1Spec>;
#[doc = "Field `BIT_HIOE_STR_ADDR_CHN1` reader - HIOE command start address in txpktbuffer, unit:8byte"]
pub type BitHioeStrAddrChn1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HIOE_STR_ADDR_CHN1` writer - HIOE command start address in txpktbuffer, unit:8byte"]
pub type BitHioeStrAddrChn1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HIOE_END_ADDR_CHN1` reader - HIOE command end address in txpktbuffer, unit:8byte"]
pub type BitHioeEndAddrChn1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HIOE_END_ADDR_CHN1` writer - HIOE command end address in txpktbuffer, unit:8byte"]
pub type BitHioeEndAddrChn1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HIOE_BACKUP_REQ_CHN1` reader - WLOFF REG backup request, clear by hw"]
pub type BitHioeBackupReqChn1R = crate::BitReader;
#[doc = "Field `BIT_HIOE_BACKUP_REQ_CHN1` writer - WLOFF REG backup request, clear by hw"]
pub type BitHioeBackupReqChn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIOE_RESTORE_REQ_CHN1` reader - WLOFF REG restore request, clear by hw"]
pub type BitHioeRestoreReqChn1R = crate::BitReader;
#[doc = "Field `BIT_HIOE_RESTORE_REQ_CHN1` writer - WLOFF REG restore request, clear by hw"]
pub type BitHioeRestoreReqChn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_HIOE_ACTIVE_CHN1` reader - indicate HIOE engine is active"]
pub type BitHioeActiveChn1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - HIOE command start address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_str_addr_chn1(&self) -> BitHioeStrAddrChn1R {
        BitHioeStrAddrChn1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - HIOE command end address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_end_addr_chn1(&self) -> BitHioeEndAddrChn1R {
        BitHioeEndAddrChn1R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - WLOFF REG backup request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_backup_req_chn1(&self) -> BitHioeBackupReqChn1R {
        BitHioeBackupReqChn1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WLOFF REG restore request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_restore_req_chn1(&self) -> BitHioeRestoreReqChn1R {
        BitHioeRestoreReqChn1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - indicate HIOE engine is active"]
    #[inline(always)]
    pub fn bit_hioe_active_chn1(&self) -> BitHioeActiveChn1R {
        BitHioeActiveChn1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - HIOE command start address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_str_addr_chn1(&mut self) -> BitHioeStrAddrChn1W<'_, RegHwIoeCrChn1Spec> {
        BitHioeStrAddrChn1W::new(self, 0)
    }
    #[doc = "Bits 12:23 - HIOE command end address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_end_addr_chn1(&mut self) -> BitHioeEndAddrChn1W<'_, RegHwIoeCrChn1Spec> {
        BitHioeEndAddrChn1W::new(self, 12)
    }
    #[doc = "Bit 24 - WLOFF REG backup request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_backup_req_chn1(&mut self) -> BitHioeBackupReqChn1W<'_, RegHwIoeCrChn1Spec> {
        BitHioeBackupReqChn1W::new(self, 24)
    }
    #[doc = "Bit 25 - WLOFF REG restore request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_restore_req_chn1(&mut self) -> BitHioeRestoreReqChn1W<'_, RegHwIoeCrChn1Spec> {
        BitHioeRestoreReqChn1W::new(self, 25)
    }
    #[doc = "Bits 26:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegHwIoeCrChn1Spec> {
        RsvdW::new(self, 26)
    }
}
#[doc = "HW IO Engine Configure Register_CHN1\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hw_ioe_cr_chn1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hw_ioe_cr_chn1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHwIoeCrChn1Spec;
impl crate::RegisterSpec for RegHwIoeCrChn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hw_ioe_cr_chn1::R`](R) reader structure"]
impl crate::Readable for RegHwIoeCrChn1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hw_ioe_cr_chn1::W`](W) writer structure"]
impl crate::Writable for RegHwIoeCrChn1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HW_IOE_CR_CHN1 to value 0"]
impl crate::Resettable for RegHwIoeCrChn1Spec {}
