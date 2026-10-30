#[doc = "Register `REG_HST_FRREM_CTRL` reader"]
pub type R = crate::R<RegHstFrremCtrlSpec>;
#[doc = "Register `REG_HST_FRREM_CTRL` writer"]
pub type W = crate::W<RegHstFrremCtrlSpec>;
#[doc = "Field `hst_frremain_thresh` reader - threshold for checking hst_frrem if hst_frrem(otg_core) > threshold, enable iso endpoint in this frame else, enable iso endpoint in next frame"]
pub type HstFrremainThreshR = crate::FieldReader<u16>;
#[doc = "Field `hst_frremain_thresh` writer - threshold for checking hst_frrem if hst_frrem(otg_core) > threshold, enable iso endpoint in this frame else, enable iso endpoint in next frame"]
pub type HstFrremainThreshW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - threshold for checking hst_frrem if hst_frrem(otg_core) > threshold, enable iso endpoint in this frame else, enable iso endpoint in next frame"]
    #[inline(always)]
    pub fn hst_frremain_thresh(&self) -> HstFrremainThreshR {
        HstFrremainThreshR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - threshold for checking hst_frrem if hst_frrem(otg_core) > threshold, enable iso endpoint in this frame else, enable iso endpoint in next frame"]
    #[inline(always)]
    pub fn hst_frremain_thresh(&mut self) -> HstFrremainThreshW<'_, RegHstFrremCtrlSpec> {
        HstFrremainThreshW::new(self, 0)
    }
}
#[doc = "Host Frame Remain Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hst_frrem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hst_frrem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHstFrremCtrlSpec;
impl crate::RegisterSpec for RegHstFrremCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hst_frrem_ctrl::R`](R) reader structure"]
impl crate::Readable for RegHstFrremCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_hst_frrem_ctrl::W`](W) writer structure"]
impl crate::Writable for RegHstFrremCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HST_FRREM_CTRL to value 0x10"]
impl crate::Resettable for RegHstFrremCtrlSpec {
    const RESET_VALUE: u32 = 0x10;
}
