#[doc = "Register `RX_DES_THRES6` reader"]
pub type R = crate::R<RxDesThres6Spec>;
#[doc = "Register `RX_DES_THRES6` writer"]
pub type W = crate::W<RxDesThres6Spec>;
#[doc = "Field `Rx_Pse_Des_Thres_off6` reader - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
pub type RxPseDesThresOff6R = crate::FieldReader<u16>;
#[doc = "Field `Rx_Pse_Des_Thres_off6` writer - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
pub type RxPseDesThresOff6W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `Rx_Pse_Des_Thres_on6` reader - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
pub type RxPseDesThresOn6R = crate::FieldReader<u16>;
#[doc = "Field `Rx_Pse_Des_Thres_on6` writer - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
pub type RxPseDesThresOn6W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off6(&self) -> RxPseDesThresOff6R {
        RxPseDesThresOff6R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on6(&self) -> RxPseDesThresOn6R {
        RxPseDesThresOn6R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off6(&mut self) -> RxPseDesThresOff6W<'_, RxDesThres6Spec> {
        RxPseDesThresOff6W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on6(&mut self) -> RxPseDesThresOn6W<'_, RxDesThres6Spec> {
        RxPseDesThresOn6W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_des_thres6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_des_thres6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDesThres6Spec;
impl crate::RegisterSpec for RxDesThres6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_des_thres6::R`](R) reader structure"]
impl crate::Readable for RxDesThres6Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_des_thres6::W`](W) writer structure"]
impl crate::Writable for RxDesThres6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_DES_THRES6 to value 0"]
impl crate::Resettable for RxDesThres6Spec {}
