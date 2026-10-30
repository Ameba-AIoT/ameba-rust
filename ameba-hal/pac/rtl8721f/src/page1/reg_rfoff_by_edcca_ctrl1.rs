#[doc = "Register `REG_RFOFF_BY_EDCCA_CTRL1` reader"]
pub type R = crate::R<RegRfoffByEdccaCtrl1Spec>;
#[doc = "Register `REG_RFOFF_BY_EDCCA_CTRL1` writer"]
pub type W = crate::W<RegRfoffByEdccaCtrl1Spec>;
#[doc = "Field `BIT_EDCCA_CHK_AREA1_START` reader - EDCCA_IDLE_area1 start time,start form TBTT,unit:256us,default:9.216ms"]
pub type BitEdccaChkArea1StartR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA1_START` writer - EDCCA_IDLE_area1 start time,start form TBTT,unit:256us,default:9.216ms"]
pub type BitEdccaChkArea1StartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EDCCA_CHK_AREA1_TO_THRES` reader - EDCCA_IDLE timeout threshold in area1,unit:4us,default:16us"]
pub type BitEdccaChkArea1ToThresR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA1_TO_THRES` writer - EDCCA_IDLE timeout threshold in area1,unit:4us,default:16us"]
pub type BitEdccaChkArea1ToThresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EDCCA_CHK_AREA1_CNT_THRES` reader - EDCCA_IDLE timeout counter treshold in area1,when timeout counter greater than threshold, generate interrupt"]
pub type BitEdccaChkArea1CntThresR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_CHK_AREA1_CNT_THRES` writer - EDCCA_IDLE timeout counter treshold in area1,when timeout counter greater than threshold, generate interrupt"]
pub type BitEdccaChkArea1CntThresW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_RFOFF_EDCCA_SEL` reader - 1:sel edcca| cca as instruction 0:sel edcca as instruction"]
pub type BitRfoffEdccaSelR = crate::BitReader;
#[doc = "Field `BIT_RFOFF_EDCCA_SEL` writer - 1:sel edcca| cca as instruction 0:sel edcca as instruction"]
pub type BitRfoffEdccaSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RFOFF_EDCCA_EN` reader - enable function of rf off by edcca idle timeout"]
pub type BitRfoffEdccaEnR = crate::BitReader;
#[doc = "Field `BIT_RFOFF_EDCCA_EN` writer - enable function of rf off by edcca idle timeout"]
pub type BitRfoffEdccaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - EDCCA_IDLE_area1 start time,start form TBTT,unit:256us,default:9.216ms"]
    #[inline(always)]
    pub fn bit_edcca_chk_area1_start(&self) -> BitEdccaChkArea1StartR {
        BitEdccaChkArea1StartR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EDCCA_IDLE timeout threshold in area1,unit:4us,default:16us"]
    #[inline(always)]
    pub fn bit_edcca_chk_area1_to_thres(&self) -> BitEdccaChkArea1ToThresR {
        BitEdccaChkArea1ToThresR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - EDCCA_IDLE timeout counter treshold in area1,when timeout counter greater than threshold, generate interrupt"]
    #[inline(always)]
    pub fn bit_edcca_chk_area1_cnt_thres(&self) -> BitEdccaChkArea1CntThresR {
        BitEdccaChkArea1CntThresR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - 1:sel edcca| cca as instruction 0:sel edcca as instruction"]
    #[inline(always)]
    pub fn bit_rfoff_edcca_sel(&self) -> BitRfoffEdccaSelR {
        BitRfoffEdccaSelR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable function of rf off by edcca idle timeout"]
    #[inline(always)]
    pub fn bit_rfoff_edcca_en(&self) -> BitRfoffEdccaEnR {
        BitRfoffEdccaEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - EDCCA_IDLE_area1 start time,start form TBTT,unit:256us,default:9.216ms"]
    #[inline(always)]
    pub fn bit_edcca_chk_area1_start(
        &mut self,
    ) -> BitEdccaChkArea1StartW<'_, RegRfoffByEdccaCtrl1Spec> {
        BitEdccaChkArea1StartW::new(self, 0)
    }
    #[doc = "Bits 8:15 - EDCCA_IDLE timeout threshold in area1,unit:4us,default:16us"]
    #[inline(always)]
    pub fn bit_edcca_chk_area1_to_thres(
        &mut self,
    ) -> BitEdccaChkArea1ToThresW<'_, RegRfoffByEdccaCtrl1Spec> {
        BitEdccaChkArea1ToThresW::new(self, 8)
    }
    #[doc = "Bits 16:19 - EDCCA_IDLE timeout counter treshold in area1,when timeout counter greater than threshold, generate interrupt"]
    #[inline(always)]
    pub fn bit_edcca_chk_area1_cnt_thres(
        &mut self,
    ) -> BitEdccaChkArea1CntThresW<'_, RegRfoffByEdccaCtrl1Spec> {
        BitEdccaChkArea1CntThresW::new(self, 16)
    }
    #[doc = "Bit 30 - 1:sel edcca| cca as instruction 0:sel edcca as instruction"]
    #[inline(always)]
    pub fn bit_rfoff_edcca_sel(&mut self) -> BitRfoffEdccaSelW<'_, RegRfoffByEdccaCtrl1Spec> {
        BitRfoffEdccaSelW::new(self, 30)
    }
    #[doc = "Bit 31 - enable function of rf off by edcca idle timeout"]
    #[inline(always)]
    pub fn bit_rfoff_edcca_en(&mut self) -> BitRfoffEdccaEnW<'_, RegRfoffByEdccaCtrl1Spec> {
        BitRfoffEdccaEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rfoff_by_edcca_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rfoff_by_edcca_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRfoffByEdccaCtrl1Spec;
impl crate::RegisterSpec for RegRfoffByEdccaCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rfoff_by_edcca_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegRfoffByEdccaCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rfoff_by_edcca_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegRfoffByEdccaCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RFOFF_BY_EDCCA_CTRL1 to value 0x0002_0424"]
impl crate::Resettable for RegRfoffByEdccaCtrl1Spec {
    const RESET_VALUE: u32 = 0x0002_0424;
}
