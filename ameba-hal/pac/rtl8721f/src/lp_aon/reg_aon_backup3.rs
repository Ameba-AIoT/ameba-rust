#[doc = "Register `REG_AON_BACKUP3` reader"]
pub type R = crate::R<RegAonBackup3Spec>;
#[doc = "Register `REG_AON_BACKUP3` writer"]
pub type W = crate::W<RegAonBackup3Spec>;
#[doc = "Field `aon_bk3` reader - "]
pub type AonBk3R = crate::FieldReader<u32>;
#[doc = "Field `aon_bk3` writer - "]
pub type AonBk3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aon_bk3(&self) -> AonBk3R {
        AonBk3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aon_bk3(&mut self) -> AonBk3W<'_, RegAonBackup3Spec> {
        AonBk3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aon_backup3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aon_backup3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAonBackup3Spec;
impl crate::RegisterSpec for RegAonBackup3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aon_backup3::R`](R) reader structure"]
impl crate::Readable for RegAonBackup3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_aon_backup3::W`](W) writer structure"]
impl crate::Writable for RegAonBackup3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AON_BACKUP3 to value 0"]
impl crate::Resettable for RegAonBackup3Spec {}
