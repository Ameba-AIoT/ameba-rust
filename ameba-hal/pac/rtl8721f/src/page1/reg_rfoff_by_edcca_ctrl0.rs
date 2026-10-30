#[doc = "Register `REG_RFOFF_BY_EDCCA_CTRL0` reader"]
pub type R = crate::R<RegRfoffByEdccaCtrl0Spec>;
#[doc = "Register `REG_RFOFF_BY_EDCCA_CTRL0` writer"]
pub type W = crate::W<RegRfoffByEdccaCtrl0Spec>;
#[doc = "Field `BIT_EDCCA_CHK_AREA0_START` reader - EDCCA_IDLE_area0 end time,start form TBTT,unit:256us,default:1.024ms"]
pub type BitEdccaChkArea0StartR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA0_START` writer - EDCCA_IDLE_area0 end time,start form TBTT,unit:256us,default:1.024ms"]
pub type BitEdccaChkArea0StartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EDCCA_CHK_AREA0_TO_THRES` reader - EDCCA_IDLE timeout threshold in area0,unit:4us,default:16us"]
pub type BitEdccaChkArea0ToThresR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA0_TO_THRES` writer - EDCCA_IDLE timeout threshold in area0,unit:4us,default:16us"]
pub type BitEdccaChkArea0ToThresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EDCCA_CHK_AREA0_CNT_THRES` reader - EDCCA_IDLE timeout counter treshold in area0,when timeout counter greater than threshold, generate interrupt"]
pub type BitEdccaChkArea0CntThresR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA0_CNT_THRES` writer - EDCCA_IDLE timeout counter treshold in area0,when timeout counter greater than threshold, generate interrupt"]
pub type BitEdccaChkArea0CntThresW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EDCCA_IDLE_TIMER` reader - edcca idle timeout timer,unit:4us"]
pub type EdccaIdleTimerR = crate::FieldReader;
#[doc = "Field `EDCCA_ILDE_TO_CNT` reader - indicate how many times edcca idle timeout happens"]
pub type EdccaIldeToCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - EDCCA_IDLE_area0 end time,start form TBTT,unit:256us,default:1.024ms"]
    #[inline(always)]
    pub fn bit_edcca_chk_area0_start(&self) -> BitEdccaChkArea0StartR {
        BitEdccaChkArea0StartR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EDCCA_IDLE timeout threshold in area0,unit:4us,default:16us"]
    #[inline(always)]
    pub fn bit_edcca_chk_area0_to_thres(&self) -> BitEdccaChkArea0ToThresR {
        BitEdccaChkArea0ToThresR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - EDCCA_IDLE timeout counter treshold in area0,when timeout counter greater than threshold, generate interrupt"]
    #[inline(always)]
    pub fn bit_edcca_chk_area0_cnt_thres(&self) -> BitEdccaChkArea0CntThresR {
        BitEdccaChkArea0CntThresR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - edcca idle timeout timer,unit:4us"]
    #[inline(always)]
    pub fn edcca_idle_timer(&self) -> EdccaIdleTimerR {
        EdccaIdleTimerR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - indicate how many times edcca idle timeout happens"]
    #[inline(always)]
    pub fn edcca_ilde_to_cnt(&self) -> EdccaIldeToCntR {
        EdccaIldeToCntR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EDCCA_IDLE_area0 end time,start form TBTT,unit:256us,default:1.024ms"]
    #[inline(always)]
    pub fn bit_edcca_chk_area0_start(
        &mut self,
    ) -> BitEdccaChkArea0StartW<'_, RegRfoffByEdccaCtrl0Spec> {
        BitEdccaChkArea0StartW::new(self, 0)
    }
    #[doc = "Bits 8:15 - EDCCA_IDLE timeout threshold in area0,unit:4us,default:16us"]
    #[inline(always)]
    pub fn bit_edcca_chk_area0_to_thres(
        &mut self,
    ) -> BitEdccaChkArea0ToThresW<'_, RegRfoffByEdccaCtrl0Spec> {
        BitEdccaChkArea0ToThresW::new(self, 8)
    }
    #[doc = "Bits 16:19 - EDCCA_IDLE timeout counter treshold in area0,when timeout counter greater than threshold, generate interrupt"]
    #[inline(always)]
    pub fn bit_edcca_chk_area0_cnt_thres(
        &mut self,
    ) -> BitEdccaChkArea0CntThresW<'_, RegRfoffByEdccaCtrl0Spec> {
        BitEdccaChkArea0CntThresW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rfoff_by_edcca_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rfoff_by_edcca_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRfoffByEdccaCtrl0Spec;
impl crate::RegisterSpec for RegRfoffByEdccaCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rfoff_by_edcca_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegRfoffByEdccaCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rfoff_by_edcca_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegRfoffByEdccaCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RFOFF_BY_EDCCA_CTRL0 to value 0x0003_0404"]
impl crate::Resettable for RegRfoffByEdccaCtrl0Spec {
    const RESET_VALUE: u32 = 0x0003_0404;
}
