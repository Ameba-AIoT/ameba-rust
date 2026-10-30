#[doc = "Register `REG_TWT_CTRL` reader"]
pub type R = crate::R<RegTwtCtrlSpec>;
#[doc = "Register `REG_TWT_CTRL` writer"]
pub type W = crate::W<RegTwtCtrlSpec>;
#[doc = "Field `BIT_RX_MORE_TF_EN` reader - Enable trigger frame more TF field interrupt"]
pub type BitRxMoreTfEnR = crate::BitReader;
#[doc = "Field `BIT_RX_MORE_TF_EN` writer - Enable trigger frame more TF field interrupt"]
pub type BitRxMoreTfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RX_ACKMD_EN` reader - Enable ACK/BA/MU-BA frame more data interrupt"]
pub type BitRxAckmdEnR = crate::BitReader;
#[doc = "Field `BIT_RX_ACKMD_EN` writer - Enable ACK/BA/MU-BA frame more data interrupt"]
pub type BitRxAckmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable trigger frame more TF field interrupt"]
    #[inline(always)]
    pub fn bit_rx_more_tf_en(&self) -> BitRxMoreTfEnR {
        BitRxMoreTfEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ACK/BA/MU-BA frame more data interrupt"]
    #[inline(always)]
    pub fn bit_rx_ackmd_en(&self) -> BitRxAckmdEnR {
        BitRxAckmdEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable trigger frame more TF field interrupt"]
    #[inline(always)]
    pub fn bit_rx_more_tf_en(&mut self) -> BitRxMoreTfEnW<'_, RegTwtCtrlSpec> {
        BitRxMoreTfEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable ACK/BA/MU-BA frame more data interrupt"]
    #[inline(always)]
    pub fn bit_rx_ackmd_en(&mut self) -> BitRxAckmdEnW<'_, RegTwtCtrlSpec> {
        BitRxAckmdEnW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_twt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_twt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTwtCtrlSpec;
impl crate::RegisterSpec for RegTwtCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_twt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTwtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_twt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTwtCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TWT_CTRL to value 0"]
impl crate::Resettable for RegTwtCtrlSpec {}
