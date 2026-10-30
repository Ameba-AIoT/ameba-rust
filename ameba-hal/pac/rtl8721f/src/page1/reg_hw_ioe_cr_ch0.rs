#[doc = "Register `REG_HW_IOE_CR_CH0` reader"]
pub type R = crate::R<RegHwIoeCrCh0Spec>;
#[doc = "Register `REG_HW_IOE_CR_CH0` writer"]
pub type W = crate::W<RegHwIoeCrCh0Spec>;
#[doc = "Field `BIT_HIOE_STR_ADDR_CH0` reader - HIOE command start address in txpktbuffer, unit:8byte"]
pub type BitHioeStrAddrCh0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HIOE_STR_ADDR_CH0` writer - HIOE command start address in txpktbuffer, unit:8byte"]
pub type BitHioeStrAddrCh0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HIOE_END_ADDR_CH0` reader - HIOE command end address in txpktbuffer, unit:8byte"]
pub type BitHioeEndAddrCh0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HIOE_END_ADDR_CH0` writer - HIOE command end address in txpktbuffer, unit:8byte"]
pub type BitHioeEndAddrCh0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_HIOE_BACKUP_REQ_CH0` reader - WLOFF REG backup request, clear by hw"]
pub type BitHioeBackupReqCh0R = crate::BitReader;
#[doc = "Field `BIT_HIOE_BACKUP_REQ_CH0` writer - WLOFF REG backup request, clear by hw"]
pub type BitHioeBackupReqCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIOE_RESTORE_REQ_CH0` reader - WLOFF REG restore request, clear by hw"]
pub type BitHioeRestoreReqCh0R = crate::BitReader;
#[doc = "Field `BIT_HIOE_RESTORE_REQ_CH0` writer - WLOFF REG restore request, clear by hw"]
pub type BitHioeRestoreReqCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_HIOE_ACTIVE_CH0` reader - indicate HIOE engine is active"]
pub type BitHioeActiveCh0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - HIOE command start address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_str_addr_ch0(&self) -> BitHioeStrAddrCh0R {
        BitHioeStrAddrCh0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - HIOE command end address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_end_addr_ch0(&self) -> BitHioeEndAddrCh0R {
        BitHioeEndAddrCh0R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - WLOFF REG backup request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_backup_req_ch0(&self) -> BitHioeBackupReqCh0R {
        BitHioeBackupReqCh0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WLOFF REG restore request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_restore_req_ch0(&self) -> BitHioeRestoreReqCh0R {
        BitHioeRestoreReqCh0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - indicate HIOE engine is active"]
    #[inline(always)]
    pub fn bit_hioe_active_ch0(&self) -> BitHioeActiveCh0R {
        BitHioeActiveCh0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - HIOE command start address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_str_addr_ch0(&mut self) -> BitHioeStrAddrCh0W<'_, RegHwIoeCrCh0Spec> {
        BitHioeStrAddrCh0W::new(self, 0)
    }
    #[doc = "Bits 12:23 - HIOE command end address in txpktbuffer, unit:8byte"]
    #[inline(always)]
    pub fn bit_hioe_end_addr_ch0(&mut self) -> BitHioeEndAddrCh0W<'_, RegHwIoeCrCh0Spec> {
        BitHioeEndAddrCh0W::new(self, 12)
    }
    #[doc = "Bit 24 - WLOFF REG backup request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_backup_req_ch0(&mut self) -> BitHioeBackupReqCh0W<'_, RegHwIoeCrCh0Spec> {
        BitHioeBackupReqCh0W::new(self, 24)
    }
    #[doc = "Bit 25 - WLOFF REG restore request, clear by hw"]
    #[inline(always)]
    pub fn bit_hioe_restore_req_ch0(&mut self) -> BitHioeRestoreReqCh0W<'_, RegHwIoeCrCh0Spec> {
        BitHioeRestoreReqCh0W::new(self, 25)
    }
    #[doc = "Bits 26:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegHwIoeCrCh0Spec> {
        RsvdW::new(self, 26)
    }
}
#[doc = "HW IO Engine Configure Register_CH0\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hw_ioe_cr_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hw_ioe_cr_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHwIoeCrCh0Spec;
impl crate::RegisterSpec for RegHwIoeCrCh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hw_ioe_cr_ch0::R`](R) reader structure"]
impl crate::Readable for RegHwIoeCrCh0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hw_ioe_cr_ch0::W`](W) writer structure"]
impl crate::Writable for RegHwIoeCrCh0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HW_IOE_CR_CH0 to value 0"]
impl crate::Resettable for RegHwIoeCrCh0Spec {}
