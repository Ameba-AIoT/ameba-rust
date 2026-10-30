#[doc = "Register `IPC_IMR` reader"]
pub type R = crate::R<IpcImrSpec>;
#[doc = "Register `IPC_IMR` writer"]
pub type W = crate::W<IpcImrSpec>;
#[doc = "Field `imr_rx0_full_mask` reader - 0: Mask Rx Channel x full interrupt of CPU1 received from CPU0 1: Unmask Rx Channel x full interrupt of CPU1 received from CPU0"]
pub type ImrRx0FullMaskR = crate::FieldReader<u16>;
#[doc = "Field `imr_rx0_full_mask` writer - 0: Mask Rx Channel x full interrupt of CPU1 received from CPU0 1: Unmask Rx Channel x full interrupt of CPU1 received from CPU0"]
pub type ImrRx0FullMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `imr_tx0_empty_mask` reader - 0: Mask Tx Channel x empty interrupt of CPU1 transmit to CPU0 1: Unmask Tx Channel x empty interrupt of CPU1 transmit to CPU0"]
pub type ImrTx0EmptyMaskR = crate::FieldReader<u16>;
#[doc = "Field `imr_tx0_empty_mask` writer - 0: Mask Tx Channel x empty interrupt of CPU1 transmit to CPU0 1: Unmask Tx Channel x empty interrupt of CPU1 transmit to CPU0"]
pub type ImrTx0EmptyMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 0: Mask Rx Channel x full interrupt of CPU1 received from CPU0 1: Unmask Rx Channel x full interrupt of CPU1 received from CPU0"]
    #[inline(always)]
    pub fn imr_rx0_full_mask(&self) -> ImrRx0FullMaskR {
        ImrRx0FullMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 0: Mask Tx Channel x empty interrupt of CPU1 transmit to CPU0 1: Unmask Tx Channel x empty interrupt of CPU1 transmit to CPU0"]
    #[inline(always)]
    pub fn imr_tx0_empty_mask(&self) -> ImrTx0EmptyMaskR {
        ImrTx0EmptyMaskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 0: Mask Rx Channel x full interrupt of CPU1 received from CPU0 1: Unmask Rx Channel x full interrupt of CPU1 received from CPU0"]
    #[inline(always)]
    pub fn imr_rx0_full_mask(&mut self) -> ImrRx0FullMaskW<'_, IpcImrSpec> {
        ImrRx0FullMaskW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 0: Mask Tx Channel x empty interrupt of CPU1 transmit to CPU0 1: Unmask Tx Channel x empty interrupt of CPU1 transmit to CPU0"]
    #[inline(always)]
    pub fn imr_tx0_empty_mask(&mut self) -> ImrTx0EmptyMaskW<'_, IpcImrSpec> {
        ImrTx0EmptyMaskW::new(self, 16)
    }
}
#[doc = "Interrupt Empty Full Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcImrSpec;
impl crate::RegisterSpec for IpcImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_imr::R`](R) reader structure"]
impl crate::Readable for IpcImrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc_imr::W`](W) writer structure"]
impl crate::Writable for IpcImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC_IMR to value 0"]
impl crate::Resettable for IpcImrSpec {}
