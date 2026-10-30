#[doc = "Register `RX_CPU_DESN4` reader"]
pub type R = crate::R<RxCpuDesn4Spec>;
#[doc = "Register `RX_CPU_DESN4` writer"]
pub type W = crate::W<RxCpuDesn4Spec>;
#[doc = "Field `EthrntRxCPU_Des_Num4` reader - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
pub type EthrntRxCpuDesNum4R = crate::FieldReader<u16>;
#[doc = "Field `EthrntRxCPU_Des_Num4` writer - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
pub type EthrntRxCpuDesNum4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num4(&self) -> EthrntRxCpuDesNum4R {
        EthrntRxCpuDesNum4R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicate the number of descriptor of queue, which has been finished Rx process and returned to IO by CPU. After ending Rx process, CPU needs to update this field."]
    #[inline(always)]
    pub fn ethrnt_rx_cpu_des_num4(&mut self) -> EthrntRxCpuDesNum4W<'_, RxCpuDesn4Spec> {
        EthrntRxCpuDesNum4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cpu_desn4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_cpu_desn4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCpuDesn4Spec;
impl crate::RegisterSpec for RxCpuDesn4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_cpu_desn4::R`](R) reader structure"]
impl crate::Readable for RxCpuDesn4Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_cpu_desn4::W`](W) writer structure"]
impl crate::Writable for RxCpuDesn4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CPU_DESN4 to value 0"]
impl crate::Resettable for RxCpuDesn4Spec {}
