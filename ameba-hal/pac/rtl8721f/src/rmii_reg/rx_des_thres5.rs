#[doc = "Register `RX_DES_THRES5` reader"]
pub type R = crate::R<RxDesThres5Spec>;
#[doc = "Register `RX_DES_THRES5` writer"]
pub type W = crate::W<RxDesThres5Spec>;
#[doc = "Field `Rx_Pse_Des_Thres_off5` reader - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
pub type RxPseDesThresOff5R = crate::FieldReader<u16>;
#[doc = "Field `Rx_Pse_Des_Thres_off5` writer - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
pub type RxPseDesThresOff5W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `Rx_Pse_Des_Thres_on5` reader - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
pub type RxPseDesThresOn5R = crate::FieldReader<u16>;
#[doc = "Field `Rx_Pse_Des_Thres_on5` writer - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
pub type RxPseDesThresOn5W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off5(&self) -> RxPseDesThresOff5R {
        RxPseDesThresOff5R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on5(&self) -> RxPseDesThresOn5R {
        RxPseDesThresOn5R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off5(&mut self) -> RxPseDesThresOff5W<'_, RxDesThres5Spec> {
        RxPseDesThresOff5W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on5(&mut self) -> RxPseDesThresOn5W<'_, RxDesThres5Spec> {
        RxPseDesThresOn5W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_des_thres5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_des_thres5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDesThres5Spec;
impl crate::RegisterSpec for RxDesThres5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_des_thres5::R`](R) reader structure"]
impl crate::Readable for RxDesThres5Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_des_thres5::W`](W) writer structure"]
impl crate::Writable for RxDesThres5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_DES_THRES5 to value 0"]
impl crate::Resettable for RxDesThres5Spec {}
