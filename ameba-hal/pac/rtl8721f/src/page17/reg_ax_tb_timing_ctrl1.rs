#[doc = "Register `REG_AX_TB_TIMING_CTRL1` reader"]
pub type R = crate::R<RegAxTbTimingCtrl1Spec>;
#[doc = "Register `REG_AX_TB_TIMING_CTRL1` writer"]
pub type W = crate::W<RegAxTbTimingCtrl1Spec>;
#[doc = "Field `BIT_TB_SIFS_REF` reader - This register sets the SIFS time required for tb response when receiving trigger frame. It is in unit of 100ns.(default:12.6us)"]
pub type BitTbSifsRefR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TB_SIFS_REF` writer - This register sets the SIFS time required for tb response when receiving trigger frame. It is in unit of 100ns.(default:12.6us)"]
pub type BitTbSifsRefW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_TB_SIFS_USE_NORMAL` reader - 1:choose 0x179c sifs when rx trigger 0:choose 0x17A0\\[11:0\\] sifs when rx trigger"]
pub type BitTbSifsUseNormalR = crate::BitReader;
#[doc = "Field `BIT_TB_SIFS_USE_NORMAL` writer - 1:choose 0x179c sifs when rx trigger 0:choose 0x17A0\\[11:0\\] sifs when rx trigger"]
pub type BitTbSifsUseNormalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - This register sets the SIFS time required for tb response when receiving trigger frame. It is in unit of 100ns.(default:12.6us)"]
    #[inline(always)]
    pub fn bit_tb_sifs_ref(&self) -> BitTbSifsRefR {
        BitTbSifsRefR::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 12 - 1:choose 0x179c sifs when rx trigger 0:choose 0x17A0\\[11:0\\] sifs when rx trigger"]
    #[inline(always)]
    pub fn bit_tb_sifs_use_normal(&self) -> BitTbSifsUseNormalR {
        BitTbSifsUseNormalR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register sets the SIFS time required for tb response when receiving trigger frame. It is in unit of 100ns.(default:12.6us)"]
    #[inline(always)]
    pub fn bit_tb_sifs_ref(&mut self) -> BitTbSifsRefW<'_, RegAxTbTimingCtrl1Spec> {
        BitTbSifsRefW::new(self, 0)
    }
    #[doc = "Bit 12 - 1:choose 0x179c sifs when rx trigger 0:choose 0x17A0\\[11:0\\] sifs when rx trigger"]
    #[inline(always)]
    pub fn bit_tb_sifs_use_normal(&mut self) -> BitTbSifsUseNormalW<'_, RegAxTbTimingCtrl1Spec> {
        BitTbSifsUseNormalW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_tb_timing_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_tb_timing_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxTbTimingCtrl1Spec;
impl crate::RegisterSpec for RegAxTbTimingCtrl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_ax_tb_timing_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegAxTbTimingCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_tb_timing_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegAxTbTimingCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_TB_TIMING_CTRL1 to value 0x7e"]
impl crate::Resettable for RegAxTbTimingCtrl1Spec {
    const RESET_VALUE: u16 = 0x7e;
}
