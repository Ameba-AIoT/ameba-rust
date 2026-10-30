#[doc = "Register `REG_AON_BACKUP1` reader"]
pub type R = crate::R<RegAonBackup1Spec>;
#[doc = "Register `REG_AON_BACKUP1` writer"]
pub type W = crate::W<RegAonBackup1Spec>;
#[doc = "Field `aon_bk1` reader - "]
pub type AonBk1R = crate::FieldReader<u32>;
#[doc = "Field `aon_bk1` writer - "]
pub type AonBk1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aon_bk1(&self) -> AonBk1R {
        AonBk1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aon_bk1(&mut self) -> AonBk1W<'_, RegAonBackup1Spec> {
        AonBk1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aon_backup1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aon_backup1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAonBackup1Spec;
impl crate::RegisterSpec for RegAonBackup1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aon_backup1::R`](R) reader structure"]
impl crate::Readable for RegAonBackup1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_aon_backup1::W`](W) writer structure"]
impl crate::Writable for RegAonBackup1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AON_BACKUP1 to value 0"]
impl crate::Resettable for RegAonBackup1Spec {}
