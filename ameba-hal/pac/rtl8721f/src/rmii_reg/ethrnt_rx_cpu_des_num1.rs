#[doc = "Register `EthrntRxCPU_Des_Num1` reader"]
pub type R = crate::R<EthrntRxCpuDesNum1Spec>;
#[doc = "Register `EthrntRxCPU_Des_Num1` writer"]
pub type W = crate::W<EthrntRxCpuDesNum1Spec>;
#[doc = "Field `Rx_Pse_Des_Thres_on_1_11_8` reader - This is the Rx_Pse_Des_Thres_on1\\[11:8\\]."]
pub type RxPseDesThresOn1_11_8R = crate::FieldReader;
#[doc = "Field `Rx_Pse_Des_Thres_on_1_11_8` writer - This is the Rx_Pse_Des_Thres_on1\\[11:8\\]."]
pub type RxPseDesThresOn1_11_8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EthrntRxCPU_Des_Num_1_11_8` reader - This is the EthrntRxCPU_Des_Num1\\[11:8\\]."]
pub type EthrntRxCpuDesNum1_11_8R = crate::FieldReader;
#[doc = "Field `EthrntRxCPU_Des_Num_1_11_8` writer - This is the EthrntRxCPU_Des_Num1\\[11:8\\]."]
pub type EthrntRxCpuDesNum1_11_8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Rx_Pse_Des_Thres_off_1_7_0` reader - Specifies the difference between EthrnetRxCPU_Des_Num1 and the descriptor of 1st queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_1 should be less than (EthrntRxCPU_DES_NUM_1 - X). X is the maximum rx descriptors will be used for single ingress packet."]
pub type RxPseDesThresOff1_7_0R = crate::FieldReader;
#[doc = "Field `Rx_Pse_Des_Thres_off_1_7_0` writer - Specifies the difference between EthrnetRxCPU_Des_Num1 and the descriptor of 1st queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_1 should be less than (EthrntRxCPU_DES_NUM_1 - X). X is the maximum rx descriptors will be used for single ingress packet."]
pub type RxPseDesThresOff1_7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Rx_Pse_Des_Thres_on_1_7_0` reader - Specifies the difference between EthrnetRxCPU_Des_Num1 and the descriptor of 1st queue currently in use by Ethernet Module in which flow control will be assert."]
pub type RxPseDesThresOn1_7_0R = crate::FieldReader;
#[doc = "Field `Rx_Pse_Des_Thres_on_1_7_0` writer - Specifies the difference between EthrnetRxCPU_Des_Num1 and the descriptor of 1st queue currently in use by Ethernet Module in which flow control will be assert."]
pub type RxPseDesThresOn1_7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EthrntRxCPU_Des_Num_1_7_0` reader - Indicate the number of descriptor of 1st queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
pub type EthrntRxCpuDesNum1_7_0R = crate::FieldReader;
#[doc = "Field `EthrntRxCPU_Des_Num_1_7_0` writer - Indicate the number of descriptor of 1st queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
pub type EthrntRxCpuDesNum1_7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - This is the Rx_Pse_Des_Thres_on1\\[11:8\\]."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on_1_11_8(&self) -> RxPseDesThresOn1_11_8R {
        RxPseDesThresOn1_11_8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This is the EthrntRxCPU_Des_Num1\\[11:8\\]."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num_1_11_8(&self) -> EthrntRxCpuDesNum1_11_8R {
        EthrntRxCpuDesNum1_11_8R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Specifies the difference between EthrnetRxCPU_Des_Num1 and the descriptor of 1st queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_1 should be less than (EthrntRxCPU_DES_NUM_1 - X). X is the maximum rx descriptors will be used for single ingress packet."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off_1_7_0(&self) -> RxPseDesThresOff1_7_0R {
        RxPseDesThresOff1_7_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Specifies the difference between EthrnetRxCPU_Des_Num1 and the descriptor of 1st queue currently in use by Ethernet Module in which flow control will be assert."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on_1_7_0(&self) -> RxPseDesThresOn1_7_0R {
        RxPseDesThresOn1_7_0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicate the number of descriptor of 1st queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num_1_7_0(&self) -> EthrntRxCpuDesNum1_7_0R {
        EthrntRxCpuDesNum1_7_0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This is the Rx_Pse_Des_Thres_on1\\[11:8\\]."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on_1_11_8(
        &mut self,
    ) -> RxPseDesThresOn1_11_8W<'_, EthrntRxCpuDesNum1Spec> {
        RxPseDesThresOn1_11_8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - This is the EthrntRxCPU_Des_Num1\\[11:8\\]."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num_1_11_8(
        &mut self,
    ) -> EthrntRxCpuDesNum1_11_8W<'_, EthrntRxCpuDesNum1Spec> {
        EthrntRxCpuDesNum1_11_8W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Specifies the difference between EthrnetRxCPU_Des_Num1 and the descriptor of 1st queue currently in use by Ethernet Module in which flow control will be de-assert. Rx_Pse_Des_Thress_off_1 should be less than (EthrntRxCPU_DES_NUM_1 - X). X is the maximum rx descriptors will be used for single ingress packet."]
    #[inline(always)]
    pub fn rx_pse_des_thres_off_1_7_0(
        &mut self,
    ) -> RxPseDesThresOff1_7_0W<'_, EthrntRxCpuDesNum1Spec> {
        RxPseDesThresOff1_7_0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Specifies the difference between EthrnetRxCPU_Des_Num1 and the descriptor of 1st queue currently in use by Ethernet Module in which flow control will be assert."]
    #[inline(always)]
    pub fn rx_pse_des_thres_on_1_7_0(
        &mut self,
    ) -> RxPseDesThresOn1_7_0W<'_, EthrntRxCpuDesNum1Spec> {
        RxPseDesThresOn1_7_0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicate the number of descriptor of 1st queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num_1_7_0(
        &mut self,
    ) -> EthrntRxCpuDesNum1_7_0W<'_, EthrntRxCpuDesNum1Spec> {
        EthrntRxCpuDesNum1_7_0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ethrnt_rx_cpu_des_num1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ethrnt_rx_cpu_des_num1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EthrntRxCpuDesNum1Spec;
impl crate::RegisterSpec for EthrntRxCpuDesNum1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ethrnt_rx_cpu_des_num1::R`](R) reader structure"]
impl crate::Readable for EthrntRxCpuDesNum1Spec {}
#[doc = "`write(|w| ..)` method takes [`ethrnt_rx_cpu_des_num1::W`](W) writer structure"]
impl crate::Writable for EthrntRxCpuDesNum1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EthrntRxCPU_Des_Num1 to value 0"]
impl crate::Resettable for EthrntRxCpuDesNum1Spec {}
