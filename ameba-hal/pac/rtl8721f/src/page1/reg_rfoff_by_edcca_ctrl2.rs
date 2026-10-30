#[doc = "Register `REG_RFOFF_BY_EDCCA_CTRL2` reader"]
pub type R = crate::R<RegRfoffByEdccaCtrl2Spec>;
#[doc = "Register `REG_RFOFF_BY_EDCCA_CTRL2` writer"]
pub type W = crate::W<RegRfoffByEdccaCtrl2Spec>;
#[doc = "Field `BIT_EDCCA_CHK_AREA2_START` reader - EDCCA_IDLE_AREA2 start time,start form TBTT,unit:256us,default:15.104ms"]
pub type BitEdccaChkArea2StartR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA2_START` writer - EDCCA_IDLE_AREA2 start time,start form TBTT,unit:256us,default:15.104ms"]
pub type BitEdccaChkArea2StartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EDCCA_CHK_AREA2_TO_THRES` reader - EDCCA_IDLE timeout threshold in AREA2,unit:4us,default:16us"]
pub type BitEdccaChkArea2ToThresR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA2_TO_THRES` writer - EDCCA_IDLE timeout threshold in AREA2,unit:4us,default:16us"]
pub type BitEdccaChkArea2ToThresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EDCCA_CHK_AREA2_CNT_THRES` reader - EDCCA_IDLE timeout counter treshold in AREA2,when timeout counter greater than threshold, generate interrupt"]
pub type BitEdccaChkArea2CntThresR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA2_CNT_THRES` writer - EDCCA_IDLE timeout counter treshold in AREA2,when timeout counter greater than threshold, generate interrupt"]
pub type BitEdccaChkArea2CntThresW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EDCCA_STATUS` reader - indicate edcca or edcca|cca status of channel"]
pub type EdccaStatusR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - EDCCA_IDLE_AREA2 start time,start form TBTT,unit:256us,default:15.104ms"]
    #[inline(always)]
    pub fn bit_edcca_chk_area2_start(&self) -> BitEdccaChkArea2StartR {
        BitEdccaChkArea2StartR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EDCCA_IDLE timeout threshold in AREA2,unit:4us,default:16us"]
    #[inline(always)]
    pub fn bit_edcca_chk_area2_to_thres(&self) -> BitEdccaChkArea2ToThresR {
        BitEdccaChkArea2ToThresR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - EDCCA_IDLE timeout counter treshold in AREA2,when timeout counter greater than threshold, generate interrupt"]
    #[inline(always)]
    pub fn bit_edcca_chk_area2_cnt_thres(&self) -> BitEdccaChkArea2CntThresR {
        BitEdccaChkArea2CntThresR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - indicate edcca or edcca|cca status of channel"]
    #[inline(always)]
    pub fn edcca_status(&self) -> EdccaStatusR {
        EdccaStatusR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - EDCCA_IDLE_AREA2 start time,start form TBTT,unit:256us,default:15.104ms"]
    #[inline(always)]
    pub fn bit_edcca_chk_area2_start(
        &mut self,
    ) -> BitEdccaChkArea2StartW<'_, RegRfoffByEdccaCtrl2Spec> {
        BitEdccaChkArea2StartW::new(self, 0)
    }
    #[doc = "Bits 8:15 - EDCCA_IDLE timeout threshold in AREA2,unit:4us,default:16us"]
    #[inline(always)]
    pub fn bit_edcca_chk_area2_to_thres(
        &mut self,
    ) -> BitEdccaChkArea2ToThresW<'_, RegRfoffByEdccaCtrl2Spec> {
        BitEdccaChkArea2ToThresW::new(self, 8)
    }
    #[doc = "Bits 16:19 - EDCCA_IDLE timeout counter treshold in AREA2,when timeout counter greater than threshold, generate interrupt"]
    #[inline(always)]
    pub fn bit_edcca_chk_area2_cnt_thres(
        &mut self,
    ) -> BitEdccaChkArea2CntThresW<'_, RegRfoffByEdccaCtrl2Spec> {
        BitEdccaChkArea2CntThresW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rfoff_by_edcca_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rfoff_by_edcca_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRfoffByEdccaCtrl2Spec;
impl crate::RegisterSpec for RegRfoffByEdccaCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rfoff_by_edcca_ctrl2::R`](R) reader structure"]
impl crate::Readable for RegRfoffByEdccaCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rfoff_by_edcca_ctrl2::W`](W) writer structure"]
impl crate::Writable for RegRfoffByEdccaCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RFOFF_BY_EDCCA_CTRL2 to value 0x0001_043b"]
impl crate::Resettable for RegRfoffByEdccaCtrl2Spec {
    const RESET_VALUE: u32 = 0x0001_043b;
}
