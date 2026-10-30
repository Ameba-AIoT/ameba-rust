#[doc = "Register `REG_ACMHWCTRL` reader"]
pub type R = crate::R<RegAcmhwctrlSpec>;
#[doc = "Register `REG_ACMHWCTRL` writer"]
pub type W = crate::W<RegAcmhwctrlSpec>;
#[doc = "Field `BIT_ACMHWEN` reader - Set this bit to enable hardware to take control of ACM. 1: Hardware Control 0: Software Control Default: 0h."]
pub type BitAcmhwenR = crate::BitReader;
#[doc = "Field `BIT_ACMHWEN` writer - Set this bit to enable hardware to take control of ACM. 1: Hardware Control 0: Software Control Default: 0h."]
pub type BitAcmhwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VOQ_ACM_EN` reader - Set this bit to enable ACM of VO queue. 1: Enable 0: Disable Default: 0h."]
pub type BitVoqAcmEnR = crate::BitReader;
#[doc = "Field `BIT_VOQ_ACM_EN` writer - Set this bit to enable ACM of VO queue. 1: Enable 0: Disable Default: 0h."]
pub type BitVoqAcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VIQ_ACM_EN` reader - Set this bit to enable ACM of VI queue. 1: Enable 0: Disable Default: 0h."]
pub type BitViqAcmEnR = crate::BitReader;
#[doc = "Field `BIT_VIQ_ACM_EN` writer - Set this bit to enable ACM of VI queue. 1: Enable 0: Disable Default: 0h."]
pub type BitViqAcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BEQ_ACM_EN` reader - Set this bit to enable ACM of BE queue. 1: Enable 0: Disable Default: 0h"]
pub type BitBeqAcmEnR = crate::BitReader;
#[doc = "Field `BIT_BEQ_ACM_EN` writer - Set this bit to enable ACM of BE queue. 1: Enable 0: Disable Default: 0h"]
pub type BitBeqAcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VOQ_ACM_STATUS` reader - This bit indicates if the used_time >= admitted_time of AC VO when HW ACM. 0: VOQ ACM Not Exceed. 1: VOQ ACM Exceed. Default: 0h"]
pub type BitVoqAcmStatusR = crate::BitReader;
#[doc = "Field `BIT_VIQ_ACM_STATUS` reader - This bit indicates if the used_time >= admitted_time of AC VI when HW ACM. 0: VIQ ACM Not Exceed. 1: VIQ ACM Exceed. Default: 0h"]
pub type BitViqAcmStatusR = crate::BitReader;
#[doc = "Field `BIT_BEQ_ACM_STATUS` reader - This bit indicates if the used_time >= admitted_time of AC BE when HW ACM. 0: BEQ ACM Not Exceed. 1: BEQ ACM Exceed. Default: 0h"]
pub type BitBeqAcmStatusR = crate::BitReader;
#[doc = "Field `BIT_VO_ACM_RESET_USED_TIME` writer - When HwEn is enabled, software set this bit to inform hardware to reset used_time to 0. Read this bit is always 0."]
pub type BitVoAcmResetUsedTimeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VI_ACM_RESET_USED_TIME` writer - When HwEn is enabled, software set this bit to inform hardware to reset used_time to 0. Read this bit is always 0."]
pub type BitViAcmResetUsedTimeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BE_ACM_RESET_USED_TIME` writer - When HwEn is enabled, software set this bit to inform hardware to reset used_time to 0. Read this bit is always 0."]
pub type BitBeAcmResetUsedTimeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_AVGPERIOD` reader - In unit of 32 us. It is the value Dot11EDCAAveragingPeriod in SPEC. HW will recalculate used_time every this period. Max is 2 sec"]
pub type BitAvgperiodR = crate::FieldReader<u16>;
#[doc = "Field `BIT_AVGPERIOD` writer - In unit of 32 us. It is the value Dot11EDCAAveragingPeriod in SPEC. HW will recalculate used_time every this period. Max is 2 sec"]
pub type BitAvgperiodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable hardware to take control of ACM. 1: Hardware Control 0: Software Control Default: 0h."]
    #[inline(always)]
    pub fn bit_acmhwen(&self) -> BitAcmhwenR {
        BitAcmhwenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable ACM of VO queue. 1: Enable 0: Disable Default: 0h."]
    #[inline(always)]
    pub fn bit_voq_acm_en(&self) -> BitVoqAcmEnR {
        BitVoqAcmEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable ACM of VI queue. 1: Enable 0: Disable Default: 0h."]
    #[inline(always)]
    pub fn bit_viq_acm_en(&self) -> BitViqAcmEnR {
        BitViqAcmEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable ACM of BE queue. 1: Enable 0: Disable Default: 0h"]
    #[inline(always)]
    pub fn bit_beq_acm_en(&self) -> BitBeqAcmEnR {
        BitBeqAcmEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates if the used_time >= admitted_time of AC VO when HW ACM. 0: VOQ ACM Not Exceed. 1: VOQ ACM Exceed. Default: 0h"]
    #[inline(always)]
    pub fn bit_voq_acm_status(&self) -> BitVoqAcmStatusR {
        BitVoqAcmStatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit indicates if the used_time >= admitted_time of AC VI when HW ACM. 0: VIQ ACM Not Exceed. 1: VIQ ACM Exceed. Default: 0h"]
    #[inline(always)]
    pub fn bit_viq_acm_status(&self) -> BitViqAcmStatusR {
        BitViqAcmStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit indicates if the used_time >= admitted_time of AC BE when HW ACM. 0: BEQ ACM Not Exceed. 1: BEQ ACM Exceed. Default: 0h"]
    #[inline(always)]
    pub fn bit_beq_acm_status(&self) -> BitBeqAcmStatusR {
        BitBeqAcmStatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - In unit of 32 us. It is the value Dot11EDCAAveragingPeriod in SPEC. HW will recalculate used_time every this period. Max is 2 sec"]
    #[inline(always)]
    pub fn bit_avgperiod(&self) -> BitAvgperiodR {
        BitAvgperiodR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable hardware to take control of ACM. 1: Hardware Control 0: Software Control Default: 0h."]
    #[inline(always)]
    pub fn bit_acmhwen(&mut self) -> BitAcmhwenW<'_, RegAcmhwctrlSpec> {
        BitAcmhwenW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable ACM of VO queue. 1: Enable 0: Disable Default: 0h."]
    #[inline(always)]
    pub fn bit_voq_acm_en(&mut self) -> BitVoqAcmEnW<'_, RegAcmhwctrlSpec> {
        BitVoqAcmEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable ACM of VI queue. 1: Enable 0: Disable Default: 0h."]
    #[inline(always)]
    pub fn bit_viq_acm_en(&mut self) -> BitViqAcmEnW<'_, RegAcmhwctrlSpec> {
        BitViqAcmEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable ACM of BE queue. 1: Enable 0: Disable Default: 0h"]
    #[inline(always)]
    pub fn bit_beq_acm_en(&mut self) -> BitBeqAcmEnW<'_, RegAcmhwctrlSpec> {
        BitBeqAcmEnW::new(self, 3)
    }
    #[doc = "Bit 8 - When HwEn is enabled, software set this bit to inform hardware to reset used_time to 0. Read this bit is always 0."]
    #[inline(always)]
    pub fn bit_vo_acm_reset_used_time(&mut self) -> BitVoAcmResetUsedTimeW<'_, RegAcmhwctrlSpec> {
        BitVoAcmResetUsedTimeW::new(self, 8)
    }
    #[doc = "Bit 9 - When HwEn is enabled, software set this bit to inform hardware to reset used_time to 0. Read this bit is always 0."]
    #[inline(always)]
    pub fn bit_vi_acm_reset_used_time(&mut self) -> BitViAcmResetUsedTimeW<'_, RegAcmhwctrlSpec> {
        BitViAcmResetUsedTimeW::new(self, 9)
    }
    #[doc = "Bit 10 - When HwEn is enabled, software set this bit to inform hardware to reset used_time to 0. Read this bit is always 0."]
    #[inline(always)]
    pub fn bit_be_acm_reset_used_time(&mut self) -> BitBeAcmResetUsedTimeW<'_, RegAcmhwctrlSpec> {
        BitBeAcmResetUsedTimeW::new(self, 10)
    }
    #[doc = "Bits 16:31 - In unit of 32 us. It is the value Dot11EDCAAveragingPeriod in SPEC. HW will recalculate used_time every this period. Max is 2 sec"]
    #[inline(always)]
    pub fn bit_avgperiod(&mut self) -> BitAvgperiodW<'_, RegAcmhwctrlSpec> {
        BitAvgperiodW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_acmhwctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_acmhwctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAcmhwctrlSpec;
impl crate::RegisterSpec for RegAcmhwctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_acmhwctrl::R`](R) reader structure"]
impl crate::Readable for RegAcmhwctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_acmhwctrl::W`](W) writer structure"]
impl crate::Writable for RegAcmhwctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ACMHWCTRL to value 0"]
impl crate::Resettable for RegAcmhwctrlSpec {}
