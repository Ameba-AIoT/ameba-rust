#[doc = "Register `REG_AON_BACKUP2` reader"]
pub type R = crate::R<RegAonBackup2Spec>;
#[doc = "Register `REG_AON_BACKUP2` writer"]
pub type W = crate::W<RegAonBackup2Spec>;
#[doc = "Field `aon_bk2` reader - "]
pub type AonBk2R = crate::FieldReader<u32>;
#[doc = "Field `aon_bk2` writer - "]
pub type AonBk2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aon_bk2(&self) -> AonBk2R {
        AonBk2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aon_bk2(&mut self) -> AonBk2W<'_, RegAonBackup2Spec> {
        AonBk2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aon_backup2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aon_backup2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAonBackup2Spec;
impl crate::RegisterSpec for RegAonBackup2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aon_backup2::R`](R) reader structure"]
impl crate::Readable for RegAonBackup2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_aon_backup2::W`](W) writer structure"]
impl crate::Writable for RegAonBackup2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AON_BACKUP2 to value 0"]
impl crate::Resettable for RegAonBackup2Spec {}
