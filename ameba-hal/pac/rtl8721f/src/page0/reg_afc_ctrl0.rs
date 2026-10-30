#[doc = "Register `REG_AFC_CTRL0` reader"]
pub type R = crate::R<RegAfcCtrl0Spec>;
#[doc = "Register `REG_AFC_CTRL0` writer"]
pub type W = crate::W<RegAfcCtrl0Spec>;
#[doc = "Field `BIT_WL_XTAL_REQ_TRG` writer - trigger switch XTAL mode according to 0x30\\[6:4\\] when wlan is active, write 1 to generate one pulse"]
pub type BitWlXtalReqTrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WL_XTAL_REQ_LPS` reader - 000: xtal off 001: xtal lps mode gating 40M 010: xtal lps mode enable 40M 011: xtal normal mode 100:xtal hp mode"]
pub type BitWlXtalReqLpsR = crate::FieldReader;
#[doc = "Field `BIT_WL_XTAL_REQ_LPS` writer - 000: xtal off 001: xtal lps mode gating 40M 010: xtal lps mode enable 40M 011: xtal normal mode 100:xtal hp mode"]
pub type BitWlXtalReqLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_WL_XTAL_REQ_ACT` reader - 000: xtal off 001: xtal lps mode gating 40M 010: xtal lps mode enable 40M 011: xtal normal mode 100:xtal hp mode"]
pub type BitWlXtalReqActR = crate::FieldReader;
#[doc = "Field `BIT_WL_XTAL_REQ_ACT` writer - 000: xtal off 001: xtal lps mode gating 40M 010: xtal lps mode enable 40M 011: xtal normal mode 100:xtal hp mode"]
pub type BitWlXtalReqActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_WL_XTAL_REQ` reader - the xtal request mode of wifi"]
pub type BitWlXtalReqR = crate::FieldReader;
#[doc = "Field `BIT_POW_AFE_REQ` reader - AFE power on request to afe_ctrl 1:request to turn on AFE pwr 0:turn off AFE pwr"]
pub type BitPowAfeReqR = crate::BitReader;
#[doc = "Field `BIT_POW_AFE_REQ` writer - AFE power on request to afe_ctrl 1:request to turn on AFE pwr 0:turn off AFE pwr"]
pub type BitPowAfeReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 1:3 - 000: xtal off 001: xtal lps mode gating 40M 010: xtal lps mode enable 40M 011: xtal normal mode 100:xtal hp mode"]
    #[inline(always)]
    pub fn bit_wl_xtal_req_lps(&self) -> BitWlXtalReqLpsR {
        BitWlXtalReqLpsR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - 000: xtal off 001: xtal lps mode gating 40M 010: xtal lps mode enable 40M 011: xtal normal mode 100:xtal hp mode"]
    #[inline(always)]
    pub fn bit_wl_xtal_req_act(&self) -> BitWlXtalReqActR {
        BitWlXtalReqActR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - the xtal request mode of wifi"]
    #[inline(always)]
    pub fn bit_wl_xtal_req(&self) -> BitWlXtalReqR {
        BitWlXtalReqR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 13 - AFE power on request to afe_ctrl 1:request to turn on AFE pwr 0:turn off AFE pwr"]
    #[inline(always)]
    pub fn bit_pow_afe_req(&self) -> BitPowAfeReqR {
        BitPowAfeReqR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - trigger switch XTAL mode according to 0x30\\[6:4\\] when wlan is active, write 1 to generate one pulse"]
    #[inline(always)]
    pub fn bit_wl_xtal_req_trg(&mut self) -> BitWlXtalReqTrgW<'_, RegAfcCtrl0Spec> {
        BitWlXtalReqTrgW::new(self, 0)
    }
    #[doc = "Bits 0:7 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegAfcCtrl0Spec> {
        DummyW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 000: xtal off 001: xtal lps mode gating 40M 010: xtal lps mode enable 40M 011: xtal normal mode 100:xtal hp mode"]
    #[inline(always)]
    pub fn bit_wl_xtal_req_lps(&mut self) -> BitWlXtalReqLpsW<'_, RegAfcCtrl0Spec> {
        BitWlXtalReqLpsW::new(self, 1)
    }
    #[doc = "Bits 4:6 - 000: xtal off 001: xtal lps mode gating 40M 010: xtal lps mode enable 40M 011: xtal normal mode 100:xtal hp mode"]
    #[inline(always)]
    pub fn bit_wl_xtal_req_act(&mut self) -> BitWlXtalReqActW<'_, RegAfcCtrl0Spec> {
        BitWlXtalReqActW::new(self, 4)
    }
    #[doc = "Bit 13 - AFE power on request to afe_ctrl 1:request to turn on AFE pwr 0:turn off AFE pwr"]
    #[inline(always)]
    pub fn bit_pow_afe_req(&mut self) -> BitPowAfeReqW<'_, RegAfcCtrl0Spec> {
        BitPowAfeReqW::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_afc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_afc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAfcCtrl0Spec;
impl crate::RegisterSpec for RegAfcCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_afc_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegAfcCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_afc_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegAfcCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AFC_CTRL0 to value 0x40"]
impl crate::Resettable for RegAfcCtrl0Spec {
    const RESET_VALUE: u32 = 0x40;
}
