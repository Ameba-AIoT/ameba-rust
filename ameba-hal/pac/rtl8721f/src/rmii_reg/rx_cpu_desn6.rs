#[doc = "Register `RX_CPU_DESN6` reader"]
pub type R = crate::R<RxCpuDesn6Spec>;
#[doc = "Register `RX_CPU_DESN6` writer"]
pub type W = crate::W<RxCpuDesn6Spec>;
#[doc = "Field `EthrntRxCPU_Des_Num6` reader - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
pub type EthrntRxCpuDesNum6R = crate::FieldReader<u16>;
#[doc = "Field `EthrntRxCPU_Des_Num6` writer - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
pub type EthrntRxCpuDesNum6W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num6(&self) -> EthrntRxCpuDesNum6R {
        EthrntRxCpuDesNum6R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num6(&mut self) -> EthrntRxCpuDesNum6W<'_, RxCpuDesn6Spec> {
        EthrntRxCpuDesNum6W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cpu_desn6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_cpu_desn6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCpuDesn6Spec;
impl crate::RegisterSpec for RxCpuDesn6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_cpu_desn6::R`](R) reader structure"]
impl crate::Readable for RxCpuDesn6Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_cpu_desn6::W`](W) writer structure"]
impl crate::Writable for RxCpuDesn6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CPU_DESN6 to value 0"]
impl crate::Resettable for RxCpuDesn6Spec {}
