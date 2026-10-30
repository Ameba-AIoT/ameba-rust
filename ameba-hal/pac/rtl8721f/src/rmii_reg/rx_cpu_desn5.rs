#[doc = "Register `RX_CPU_DESN5` reader"]
pub type R = crate::R<RxCpuDesn5Spec>;
#[doc = "Register `RX_CPU_DESN5` writer"]
pub type W = crate::W<RxCpuDesn5Spec>;
#[doc = "Field `EthrntRxCPU_Des_Num5` reader - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
pub type EthrntRxCpuDesNum5R = crate::FieldReader<u16>;
#[doc = "Field `EthrntRxCPU_Des_Num5` writer - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
pub type EthrntRxCpuDesNum5W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num5(&self) -> EthrntRxCpuDesNum5R {
        EthrntRxCpuDesNum5R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num5(&mut self) -> EthrntRxCpuDesNum5W<'_, RxCpuDesn5Spec> {
        EthrntRxCpuDesNum5W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cpu_desn5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_cpu_desn5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCpuDesn5Spec;
impl crate::RegisterSpec for RxCpuDesn5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_cpu_desn5::R`](R) reader structure"]
impl crate::Readable for RxCpuDesn5Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_cpu_desn5::W`](W) writer structure"]
impl crate::Writable for RxCpuDesn5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CPU_DESN5 to value 0"]
impl crate::Resettable for RxCpuDesn5Spec {}
