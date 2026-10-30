#[doc = "Register `IPC_DATA` reader"]
pub type R = crate::R<IpcDataSpec>;
#[doc = "Register `IPC_DATA` writer"]
pub type W = crate::W<IpcDataSpec>;
#[doc = "Field `rx0_data` reader - Rx data automatically maps data from tx0_data of CPU0"]
pub type Rx0DataR = crate::FieldReader<u16>;
#[doc = "Field `tx0_data` reader - After preparing descriptor, data and corresponding memory, 1: Writing 1 to the Tx data register (tx0_data) channel x bit, and the data will be mapped to the CPU0's RX data register (rx0_data) channel x bit. And the corresponding Rx full status bit in REG_ISR_CPU0 will be set. 0: Writing 0 has no effect. If the corresponding Rx full status bit in REG_ISR_CPU0 (eg.isr_rx0_full_status6) is cleared, the corresponding bit in tx0_data will be cleaned automatically."]
pub type Tx0DataR = crate::FieldReader<u16>;
#[doc = "Field `tx0_data` writer - After preparing descriptor, data and corresponding memory, 1: Writing 1 to the Tx data register (tx0_data) channel x bit, and the data will be mapped to the CPU0's RX data register (rx0_data) channel x bit. And the corresponding Rx full status bit in REG_ISR_CPU0 will be set. 0: Writing 0 has no effect. If the corresponding Rx full status bit in REG_ISR_CPU0 (eg.isr_rx0_full_status6) is cleared, the corresponding bit in tx0_data will be cleaned automatically."]
pub type Tx0DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Rx data automatically maps data from tx0_data of CPU0"]
    #[inline(always)]
    pub fn rx0_data(&self) -> Rx0DataR {
        Rx0DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - After preparing descriptor, data and corresponding memory, 1: Writing 1 to the Tx data register (tx0_data) channel x bit, and the data will be mapped to the CPU0's RX data register (rx0_data) channel x bit. And the corresponding Rx full status bit in REG_ISR_CPU0 will be set. 0: Writing 0 has no effect. If the corresponding Rx full status bit in REG_ISR_CPU0 (eg.isr_rx0_full_status6) is cleared, the corresponding bit in tx0_data will be cleaned automatically."]
    #[inline(always)]
    pub fn tx0_data(&self) -> Tx0DataR {
        Tx0DataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - After preparing descriptor, data and corresponding memory, 1: Writing 1 to the Tx data register (tx0_data) channel x bit, and the data will be mapped to the CPU0's RX data register (rx0_data) channel x bit. And the corresponding Rx full status bit in REG_ISR_CPU0 will be set. 0: Writing 0 has no effect. If the corresponding Rx full status bit in REG_ISR_CPU0 (eg.isr_rx0_full_status6) is cleared, the corresponding bit in tx0_data will be cleaned automatically."]
    #[inline(always)]
    pub fn tx0_data(&mut self) -> Tx0DataW<'_, IpcDataSpec> {
        Tx0DataW::new(self, 16)
    }
}
#[doc = "Tx_Rx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcDataSpec;
impl crate::RegisterSpec for IpcDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_data::R`](R) reader structure"]
impl crate::Readable for IpcDataSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc_data::W`](W) writer structure"]
impl crate::Writable for IpcDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC_DATA to value 0"]
impl crate::Resettable for IpcDataSpec {}
