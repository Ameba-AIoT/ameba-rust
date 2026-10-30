#[doc = "Register `REG_OBFF_CTRL` reader"]
pub type R = crate::R<RegObffCtrlSpec>;
#[doc = "Register `REG_OBFF_CTRL` writer"]
pub type W = crate::W<RegObffCtrlSpec>;
#[doc = "Field `BIT_OBFF_CTRL` reader - "]
pub type BitObffCtrlR = crate::FieldReader<u32>;
#[doc = "Field `BIT_OBFF_CTRL` writer - "]
pub type BitObffCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `BIT_LTR_ACT_FLAG` reader - "]
pub type BitLtrActFlagR = crate::BitReader;
#[doc = "Field `BIT_LTR_IDLE_FLAG` reader - "]
pub type BitLtrIdleFlagR = crate::BitReader;
#[doc = "Field `BIT_LTR_CTRL` reader - "]
pub type BitLtrCtrlR = crate::FieldReader;
#[doc = "Field `BIT_LTR_CTRL` writer - "]
pub type BitLtrCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn bit_obff_ctrl(&self) -> BitObffCtrlR {
        BitObffCtrlR::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bit_ltr_act_flag(&self) -> BitLtrActFlagR {
        BitLtrActFlagR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bit_ltr_idle_flag(&self) -> BitLtrIdleFlagR {
        BitLtrIdleFlagR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn bit_ltr_ctrl(&self) -> BitLtrCtrlR {
        BitLtrCtrlR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn bit_obff_ctrl(&mut self) -> BitObffCtrlW<'_, RegObffCtrlSpec> {
        BitObffCtrlW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn bit_ltr_ctrl(&mut self) -> BitLtrCtrlW<'_, RegObffCtrlSpec> {
        BitLtrCtrlW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_obff_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_obff_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegObffCtrlSpec;
impl crate::RegisterSpec for RegObffCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_obff_ctrl::R`](R) reader structure"]
impl crate::Readable for RegObffCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_obff_ctrl::W`](W) writer structure"]
impl crate::Writable for RegObffCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OBFF_CTRL to value 0x4a00_4010"]
impl crate::Resettable for RegObffCtrlSpec {
    const RESET_VALUE: u32 = 0x4a00_4010;
}
