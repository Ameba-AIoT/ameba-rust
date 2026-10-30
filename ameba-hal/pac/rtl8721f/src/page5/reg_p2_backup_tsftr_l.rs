#[doc = "Register `REG_P2_BACKUP_TSFTR_L` reader"]
pub type R = crate::R<RegP2BackupTsftrLSpec>;
#[doc = "Register `REG_P2_BACKUP_TSFTR_L` writer"]
pub type W = crate::W<RegP2BackupTsftrLSpec>;
#[doc = "Field `P2_BACKUP_TSF_TIMER_L` reader - low 32bits of port2's backup tsft (NAN)"]
pub type P2BackupTsfTimerLR = crate::FieldReader<u32>;
#[doc = "Field `P2_BACKUP_TSF_TIMER_L` writer - low 32bits of port2's backup tsft (NAN)"]
pub type P2BackupTsfTimerLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bits of port2's backup tsft (NAN)"]
    #[inline(always)]
    pub fn p2_backup_tsf_timer_l(&self) -> P2BackupTsfTimerLR {
        P2BackupTsfTimerLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32bits of port2's backup tsft (NAN)"]
    #[inline(always)]
    pub fn p2_backup_tsf_timer_l(&mut self) -> P2BackupTsfTimerLW<'_, RegP2BackupTsftrLSpec> {
        P2BackupTsfTimerLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p2_backup_tsftr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p2_backup_tsftr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP2BackupTsftrLSpec;
impl crate::RegisterSpec for RegP2BackupTsftrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p2_backup_tsftr_l::R`](R) reader structure"]
impl crate::Readable for RegP2BackupTsftrLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p2_backup_tsftr_l::W`](W) writer structure"]
impl crate::Writable for RegP2BackupTsftrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P2_BACKUP_TSFTR_L to value 0x01"]
impl crate::Resettable for RegP2BackupTsftrLSpec {
    const RESET_VALUE: u32 = 0x01;
}
