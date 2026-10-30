#[doc = "Register `CTC_FILTER_CTRL1` reader"]
pub type R = crate::R<CtcFilterCtrl1Spec>;
#[doc = "Register `CTC_FILTER_CTRL1` writer"]
pub type W = crate::W<CtcFilterCtrl1Spec>;
#[doc = "Field `iir_k_normal` reader - iir filter coef. the range is 0~11."]
pub type IirKNormalR = crate::FieldReader;
#[doc = "Field `iir_k_normal` writer - iir filter coef. the range is 0~11."]
pub type IirKNormalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `iir_k_acc` reader - iir filter coef. the range is 0~11."]
pub type IirKAccR = crate::FieldReader;
#[doc = "Field `iir_k_acc` writer - iir filter coef. the range is 0~11."]
pub type IirKAccW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `iir_k_acc_en` reader - enable accelerated touch response."]
pub type IirKAccEnR = crate::BitReader;
#[doc = "Field `iir_k_acc_en` writer - enable accelerated touch response."]
pub type IirKAccEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - iir filter coef. the range is 0~11."]
    #[inline(always)]
    pub fn iir_k_normal(&self) -> IirKNormalR {
        IirKNormalR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - iir filter coef. the range is 0~11."]
    #[inline(always)]
    pub fn iir_k_acc(&self) -> IirKAccR {
        IirKAccR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - enable accelerated touch response."]
    #[inline(always)]
    pub fn iir_k_acc_en(&self) -> IirKAccEnR {
        IirKAccEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - iir filter coef. the range is 0~11."]
    #[inline(always)]
    pub fn iir_k_normal(&mut self) -> IirKNormalW<'_, CtcFilterCtrl1Spec> {
        IirKNormalW::new(self, 0)
    }
    #[doc = "Bits 4:7 - iir filter coef. the range is 0~11."]
    #[inline(always)]
    pub fn iir_k_acc(&mut self) -> IirKAccW<'_, CtcFilterCtrl1Spec> {
        IirKAccW::new(self, 4)
    }
    #[doc = "Bit 8 - enable accelerated touch response."]
    #[inline(always)]
    pub fn iir_k_acc_en(&mut self) -> IirKAccEnW<'_, CtcFilterCtrl1Spec> {
        IirKAccEnW::new(self, 8)
    }
}
#[doc = "Captouch Filter Control1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_filter_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_filter_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcFilterCtrl1Spec;
impl crate::RegisterSpec for CtcFilterCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_filter_ctrl1::R`](R) reader structure"]
impl crate::Readable for CtcFilterCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctc_filter_ctrl1::W`](W) writer structure"]
impl crate::Writable for CtcFilterCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_FILTER_CTRL1 to value 0x35"]
impl crate::Resettable for CtcFilterCtrl1Spec {
    const RESET_VALUE: u32 = 0x35;
}
