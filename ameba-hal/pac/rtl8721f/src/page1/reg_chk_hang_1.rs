#[doc = "Register `REG_CHK_HANG_1` reader"]
pub type R = crate::R<RegChkHang1Spec>;
#[doc = "Register `REG_CHK_HANG_1` writer"]
pub type W = crate::W<RegChkHang1Spec>;
#[doc = "Field `r_state1_timeout_thres` reader - timeout threshold of state0_timeout_timer, unit:us"]
pub type RState1TimeoutThresR = crate::FieldReader<u16>;
#[doc = "Field `r_state1_timeout_thres` writer - timeout threshold of state0_timeout_timer, unit:us"]
pub type RState1TimeoutThresW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_state1_chk_hang` reader - when r_state1_chk_enable is set, if state_1 is the value is equal to r_state1_chk_hang, starts state1_timeout_timer"]
pub type RState1ChkHangR = crate::FieldReader;
#[doc = "Field `r_state1_chk_hang` writer - when r_state1_chk_enable is set, if state_1 is the value is equal to r_state1_chk_hang, starts state1_timeout_timer"]
pub type RState1ChkHangW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_state1_chk_enable` reader - function enable of check hang of state_1"]
pub type RState1ChkEnableR = crate::BitReader;
#[doc = "Field `r_state1_chk_enable` writer - function enable of check hang of state_1"]
pub type RState1ChkEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - timeout threshold of state0_timeout_timer, unit:us"]
    #[inline(always)]
    pub fn r_state1_timeout_thres(&self) -> RState1TimeoutThresR {
        RState1TimeoutThresR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - when r_state1_chk_enable is set, if state_1 is the value is equal to r_state1_chk_hang, starts state1_timeout_timer"]
    #[inline(always)]
    pub fn r_state1_chk_hang(&self) -> RState1ChkHangR {
        RState1ChkHangR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - function enable of check hang of state_1"]
    #[inline(always)]
    pub fn r_state1_chk_enable(&self) -> RState1ChkEnableR {
        RState1ChkEnableR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - timeout threshold of state0_timeout_timer, unit:us"]
    #[inline(always)]
    pub fn r_state1_timeout_thres(&mut self) -> RState1TimeoutThresW<'_, RegChkHang1Spec> {
        RState1TimeoutThresW::new(self, 0)
    }
    #[doc = "Bits 12:19 - when r_state1_chk_enable is set, if state_1 is the value is equal to r_state1_chk_hang, starts state1_timeout_timer"]
    #[inline(always)]
    pub fn r_state1_chk_hang(&mut self) -> RState1ChkHangW<'_, RegChkHang1Spec> {
        RState1ChkHangW::new(self, 12)
    }
    #[doc = "Bit 20 - function enable of check hang of state_1"]
    #[inline(always)]
    pub fn r_state1_chk_enable(&mut self) -> RState1ChkEnableW<'_, RegChkHang1Spec> {
        RState1ChkEnableW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_chk_hang_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_chk_hang_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChkHang1Spec;
impl crate::RegisterSpec for RegChkHang1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_chk_hang_1::R`](R) reader structure"]
impl crate::Readable for RegChkHang1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_chk_hang_1::W`](W) writer structure"]
impl crate::Writable for RegChkHang1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CHK_HANG_1 to value 0"]
impl crate::Resettable for RegChkHang1Spec {}
