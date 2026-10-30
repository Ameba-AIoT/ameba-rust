#[doc = "Register `REG_CHK_HANG_0` reader"]
pub type R = crate::R<RegChkHang0Spec>;
#[doc = "Register `REG_CHK_HANG_0` writer"]
pub type W = crate::W<RegChkHang0Spec>;
#[doc = "Field `r_state0_timeout_thres` reader - timeout threshold of state0_timeout_timer, unit:us"]
pub type RState0TimeoutThresR = crate::FieldReader<u16>;
#[doc = "Field `r_state0_timeout_thres` writer - timeout threshold of state0_timeout_timer, unit:us"]
pub type RState0TimeoutThresW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_state0_chk_hang` reader - when r_state0_chk_enable is set, if state_0 is the value is equal to r_state0_chk_hang, starts state0_timeout_timer"]
pub type RState0ChkHangR = crate::FieldReader;
#[doc = "Field `r_state0_chk_hang` writer - when r_state0_chk_enable is set, if state_0 is the value is equal to r_state0_chk_hang, starts state0_timeout_timer"]
pub type RState0ChkHangW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_state0_chk_enable` reader - function enable of check hang of state_0"]
pub type RState0ChkEnableR = crate::BitReader;
#[doc = "Field `r_state0_chk_enable` writer - function enable of check hang of state_0"]
pub type RState0ChkEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - timeout threshold of state0_timeout_timer, unit:us"]
    #[inline(always)]
    pub fn r_state0_timeout_thres(&self) -> RState0TimeoutThresR {
        RState0TimeoutThresR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - when r_state0_chk_enable is set, if state_0 is the value is equal to r_state0_chk_hang, starts state0_timeout_timer"]
    #[inline(always)]
    pub fn r_state0_chk_hang(&self) -> RState0ChkHangR {
        RState0ChkHangR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - function enable of check hang of state_0"]
    #[inline(always)]
    pub fn r_state0_chk_enable(&self) -> RState0ChkEnableR {
        RState0ChkEnableR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - timeout threshold of state0_timeout_timer, unit:us"]
    #[inline(always)]
    pub fn r_state0_timeout_thres(&mut self) -> RState0TimeoutThresW<'_, RegChkHang0Spec> {
        RState0TimeoutThresW::new(self, 0)
    }
    #[doc = "Bits 12:19 - when r_state0_chk_enable is set, if state_0 is the value is equal to r_state0_chk_hang, starts state0_timeout_timer"]
    #[inline(always)]
    pub fn r_state0_chk_hang(&mut self) -> RState0ChkHangW<'_, RegChkHang0Spec> {
        RState0ChkHangW::new(self, 12)
    }
    #[doc = "Bit 20 - function enable of check hang of state_0"]
    #[inline(always)]
    pub fn r_state0_chk_enable(&mut self) -> RState0ChkEnableW<'_, RegChkHang0Spec> {
        RState0ChkEnableW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_chk_hang_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_chk_hang_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChkHang0Spec;
impl crate::RegisterSpec for RegChkHang0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_chk_hang_0::R`](R) reader structure"]
impl crate::Readable for RegChkHang0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_chk_hang_0::W`](W) writer structure"]
impl crate::Writable for RegChkHang0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CHK_HANG_0 to value 0"]
impl crate::Resettable for RegChkHang0Spec {}
