#[doc = "Register `REG_AX_TB_TIMING_CTRL` reader"]
pub type R = crate::R<RegAxTbTimingCtrlSpec>;
#[doc = "Register `REG_AX_TB_TIMING_CTRL` writer"]
pub type W = crate::W<RegAxTbTimingCtrlSpec>;
#[doc = "Field `BIT_TB_TIMEOUT_REF` reader - This register sets the max time for tb data aggragation, It is in unit of 100ns.(default:12us)"]
pub type BitTbTimeoutRefR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TB_TIMEOUT_REF` writer - This register sets the max time for tb data aggragation, It is in unit of 100ns.(default:12us)"]
pub type BitTbTimeoutRefW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_WMAC_SIFS_BUSY_DELAY` reader - This register sets the sifs_busy delay timer, when CCA fall need delay to check edcca or edcca_per20_bitmap It is in unit of 100ns.(default:0us)"]
pub type BitWmacSifsBusyDelayR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_SIFS_BUSY_DELAY` writer - This register sets the sifs_busy delay timer, when CCA fall need delay to check edcca or edcca_per20_bitmap It is in unit of 100ns.(default:0us)"]
pub type BitWmacSifsBusyDelayW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - This register sets the max time for tb data aggragation, It is in unit of 100ns.(default:12us)"]
    #[inline(always)]
    pub fn bit_tb_timeout_ref(&self) -> BitTbTimeoutRefR {
        BitTbTimeoutRefR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - This register sets the sifs_busy delay timer, when CCA fall need delay to check edcca or edcca_per20_bitmap It is in unit of 100ns.(default:0us)"]
    #[inline(always)]
    pub fn bit_wmac_sifs_busy_delay(&self) -> BitWmacSifsBusyDelayR {
        BitWmacSifsBusyDelayR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register sets the max time for tb data aggragation, It is in unit of 100ns.(default:12us)"]
    #[inline(always)]
    pub fn bit_tb_timeout_ref(&mut self) -> BitTbTimeoutRefW<'_, RegAxTbTimingCtrlSpec> {
        BitTbTimeoutRefW::new(self, 0)
    }
    #[doc = "Bits 12:23 - This register sets the sifs_busy delay timer, when CCA fall need delay to check edcca or edcca_per20_bitmap It is in unit of 100ns.(default:0us)"]
    #[inline(always)]
    pub fn bit_wmac_sifs_busy_delay(&mut self) -> BitWmacSifsBusyDelayW<'_, RegAxTbTimingCtrlSpec> {
        BitWmacSifsBusyDelayW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_tb_timing_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_tb_timing_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxTbTimingCtrlSpec;
impl crate::RegisterSpec for RegAxTbTimingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_tb_timing_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxTbTimingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_tb_timing_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxTbTimingCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_TB_TIMING_CTRL to value 0x78"]
impl crate::Resettable for RegAxTbTimingCtrlSpec {
    const RESET_VALUE: u32 = 0x78;
}
