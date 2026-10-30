#[doc = "Register `REG_P2_BACKUP_TSFTR_H` reader"]
pub type R = crate::R<RegP2BackupTsftrHSpec>;
#[doc = "Register `REG_P2_BACKUP_TSFTR_H` writer"]
pub type W = crate::W<RegP2BackupTsftrHSpec>;
#[doc = "Field `P2_BACKUP_TSF_TIMER_H` reader - high 32bits of port2's backup tsft (NAN)"]
pub type P2BackupTsfTimerHR = crate::FieldReader<u32>;
#[doc = "Field `P2_BACKUP_TSF_TIMER_H` writer - high 32bits of port2's backup tsft (NAN)"]
pub type P2BackupTsfTimerHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high 32bits of port2's backup tsft (NAN)"]
    #[inline(always)]
    pub fn p2_backup_tsf_timer_h(&self) -> P2BackupTsfTimerHR {
        P2BackupTsfTimerHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high 32bits of port2's backup tsft (NAN)"]
    #[inline(always)]
    pub fn p2_backup_tsf_timer_h(&mut self) -> P2BackupTsfTimerHW<'_, RegP2BackupTsftrHSpec> {
        P2BackupTsfTimerHW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p2_backup_tsftr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p2_backup_tsftr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP2BackupTsftrHSpec;
impl crate::RegisterSpec for RegP2BackupTsftrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p2_backup_tsftr_h::R`](R) reader structure"]
impl crate::Readable for RegP2BackupTsftrHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p2_backup_tsftr_h::W`](W) writer structure"]
impl crate::Writable for RegP2BackupTsftrHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P2_BACKUP_TSFTR_H to value 0"]
impl crate::Resettable for RegP2BackupTsftrHSpec {}
