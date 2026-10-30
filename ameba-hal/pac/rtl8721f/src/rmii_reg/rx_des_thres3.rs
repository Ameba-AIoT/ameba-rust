#[doc = "Register `RX_DES_THRES3` reader"]
pub type R = crate::R<RxDesThres3Spec>;
#[doc = "Register `RX_DES_THRES3` writer"]
pub type W = crate::W<RxDesThres3Spec>;
#[doc = "Field `Rx_Pse_Des_Thres_off3` reader - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
pub type RxPseDesThresOff3R = crate::FieldReader<u16>;
#[doc = "Field `Rx_Pse_Des_Thres_off3` writer - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
pub type RxPseDesThresOff3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `Rx_Pse_Des_Thres_on3` reader - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
pub type RxPseDesThresOn3R = crate::FieldReader<u16>;
#[doc = "Field `Rx_Pse_Des_Thres_on3` writer - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
pub type RxPseDesThresOn3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off3(&self) -> RxPseDesThresOff3R {
        RxPseDesThresOff3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on3(&self) -> RxPseDesThresOn3R {
        RxPseDesThresOn3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_ should be less than (EthrntRxCPU_DES_NUM_ - X). X is the maximum rx descriptors will be used for single ingress packet."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off3(&mut self) -> RxPseDesThresOff3W<'_, RxDesThres3Spec> {
        RxPseDesThresOff3W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Specifies the difference between EthrnetRxCPU_Des_Num2 and the descriptor of queue currently in use by Ethernet Module in which flow control will be assert."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on3(&mut self) -> RxPseDesThresOn3W<'_, RxDesThres3Spec> {
        RxPseDesThresOn3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_des_thres3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_des_thres3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDesThres3Spec;
impl crate::RegisterSpec for RxDesThres3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_des_thres3::R`](R) reader structure"]
impl crate::Readable for RxDesThres3Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_des_thres3::W`](W) writer structure"]
impl crate::Writable for RxDesThres3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_DES_THRES3 to value 0"]
impl crate::Resettable for RxDesThres3Spec {}
