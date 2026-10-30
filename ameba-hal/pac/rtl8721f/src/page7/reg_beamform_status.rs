#[doc = "Register `REG_BEAMFORM_STATUS` reader"]
pub type R = crate::R<RegBeamformStatusSpec>;
#[doc = "Register `REG_BEAMFORM_STATUS` writer"]
pub type W = crate::W<RegBeamformStatusSpec>;
#[doc = "Field `BIT_csi_intrrupt_status` reader - sounding status: '0': fail; '1': success;"]
pub type BitCsiIntrruptStatusR = crate::BitReader;
#[doc = "Field `BIT_vhtht_mimo_ctrl_field` reader - when 'read': \\[24:1\\]: currently received VHT MIMO Control field or \\[16:1\\]: currently received HT MIMO Control field"]
pub type BitVhthtMimoCtrlFieldR = crate::FieldReader<u32>;
#[doc = "Field `BIT_WMAC_CHECK_SOUNDING_SEQ` reader - Set to 1,MAC would check sounding sequence number of received CSI report and transmitted NDPA"]
pub type BitWmacCheckSoundingSeqR = crate::BitReader;
#[doc = "Field `BIT_WMAC_CHECK_SOUNDING_SEQ` writer - Set to 1,MAC would check sounding sequence number of received CSI report and transmitted NDPA"]
pub type BitWmacCheckSoundingSeqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_write_enable` reader - Access flag. '0' for read. '1' for write, 0x72C\\[15:0\\] must be sett before this bit is set"]
pub type BitWriteEnableR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sounding status: '0': fail; '1': success;"]
    #[inline(always)]
    pub fn bit_csi_intrrupt_status(&self) -> BitCsiIntrruptStatusR {
        BitCsiIntrruptStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - when 'read': \\[24:1\\]: currently received VHT MIMO Control field or \\[16:1\\]: currently received HT MIMO Control field"]
    #[inline(always)]
    pub fn bit_vhtht_mimo_ctrl_field(&self) -> BitVhthtMimoCtrlFieldR {
        BitVhthtMimoCtrlFieldR::new((self.bits >> 1) & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - Set to 1,MAC would check sounding sequence number of received CSI report and transmitted NDPA"]
    #[inline(always)]
    pub fn bit_wmac_check_sounding_seq(&self) -> BitWmacCheckSoundingSeqR {
        BitWmacCheckSoundingSeqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Access flag. '0' for read. '1' for write, 0x72C\\[15:0\\] must be sett before this bit is set"]
    #[inline(always)]
    pub fn bit_write_enable(&self) -> BitWriteEnableR {
        BitWriteEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Set to 1,MAC would check sounding sequence number of received CSI report and transmitted NDPA"]
    #[inline(always)]
    pub fn bit_wmac_check_sounding_seq(
        &mut self,
    ) -> BitWmacCheckSoundingSeqW<'_, RegBeamformStatusSpec> {
        BitWmacCheckSoundingSeqW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_beamform_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_beamform_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBeamformStatusSpec;
impl crate::RegisterSpec for RegBeamformStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_beamform_status::R`](R) reader structure"]
impl crate::Readable for RegBeamformStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_beamform_status::W`](W) writer structure"]
impl crate::Writable for RegBeamformStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BEAMFORM_STATUS to value 0"]
impl crate::Resettable for RegBeamformStatusSpec {}
