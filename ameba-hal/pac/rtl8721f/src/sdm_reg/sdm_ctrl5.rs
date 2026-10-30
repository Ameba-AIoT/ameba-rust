#[doc = "Register `SDM_CTRL5` reader"]
pub type R = crate::R<SdmCtrl5Spec>;
#[doc = "Register `SDM_CTRL5` writer"]
pub type W = crate::W<SdmCtrl5Spec>;
#[doc = "Field `TIME_LOSS_REG` reader - Source clock time loss every cycle"]
pub type TimeLossRegR = crate::FieldReader<u32>;
#[doc = "Field `TIME_LOSS_REG` writer - Source clock time loss every cycle"]
pub type TimeLossRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source clock time loss every cycle"]
    #[inline(always)]
    pub fn time_loss_reg(&self) -> TimeLossRegR {
        TimeLossRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source clock time loss every cycle"]
    #[inline(always)]
    pub fn time_loss_reg(&mut self) -> TimeLossRegW<'_, SdmCtrl5Spec> {
        TimeLossRegW::new(self, 0)
    }
}
#[doc = "Time loss Set\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_ctrl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_ctrl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmCtrl5Spec;
impl crate::RegisterSpec for SdmCtrl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_ctrl5::R`](R) reader structure"]
impl crate::Readable for SdmCtrl5Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm_ctrl5::W`](W) writer structure"]
impl crate::Writable for SdmCtrl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDM_CTRL5 to value 0"]
impl crate::Resettable for SdmCtrl5Spec {}
