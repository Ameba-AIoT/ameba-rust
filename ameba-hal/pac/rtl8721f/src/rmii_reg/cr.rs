#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RST` writer - Reset: Setting to 1 forces the Ethernet module to a software reset state which disables the transmitter and receiver, reinitializes the FIFOs, triggers interrupt Swint for RISC to reset the system buffer pointer to the initial value Tx/Rx FDP. The values of IDR0-5 and MAR0-7 will have no changes. This bit is 1 during the reset operation, and is cleared to 0 by the Ethernet Module when the reset operation is complete."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxChkSum` reader - Receive checksum offload enable. 1: Enable. 0: Disable."]
pub type RxChkSumR = crate::BitReader;
#[doc = "Field `RxChkSum` writer - Receive checksum offload enable. 1: Enable. 0: Disable."]
pub type RxChkSumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxVLAN` reader - Receive VLAN de-tagging enable.1: Enable. 0: Disable. 1: detag vlan and set rx status dsc.tava = 1 when ingress packet has vlantag. 0: disable detag vlan and rx status dsc.tava = 0."]
pub type RxVlanR = crate::BitReader;
#[doc = "Field `RxVLAN` writer - Receive VLAN de-tagging enable.1: Enable. 0: Disable. 1: detag vlan and set rx status dsc.tava = 1 when ingress packet has vlantag. 0: disable detag vlan and rx status dsc.tava = 0."]
pub type RxVlanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxJumbo` reader - 1: support jumbo packet receiving. The maximum packet size is 16384B(0x4000). 0: Does not support jumbo packet receiving. When rxjumbo sets to 1, the minium rx descriptor ring size is 16."]
pub type RxJumboR = crate::BitReader;
#[doc = "Field `RxJumbo` writer - 1: support jumbo packet receiving. The maximum packet size is 16384B(0x4000). 0: Does not support jumbo packet receiving. When rxjumbo sets to 1, the minium rx descriptor ring size is 16."]
pub type RxJumboW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Dis_lso_err` reader - Disable LSO error"]
pub type DisLsoErrR = crate::BitReader;
#[doc = "Field `Dis_lso_err` writer - Disable LSO error"]
pub type DisLsoErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Tdsc_debug` reader - Tx descriptor debug"]
pub type TdscDebugR = crate::BitReader;
#[doc = "Field `Tdsc_debug` writer - Tx descriptor debug"]
pub type TdscDebugW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Receive checksum offload enable. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn rx_chk_sum(&self) -> RxChkSumR {
        RxChkSumR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive VLAN de-tagging enable.1: Enable. 0: Disable. 1: detag vlan and set rx status dsc.tava = 1 when ingress packet has vlantag. 0: disable detag vlan and rx status dsc.tava = 0."]
    #[inline(always)]
    pub fn rx_vlan(&self) -> RxVlanR {
        RxVlanR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: support jumbo packet receiving. The maximum packet size is 16384B(0x4000). 0: Does not support jumbo packet receiving. When rxjumbo sets to 1, the minium rx descriptor ring size is 16."]
    #[inline(always)]
    pub fn rx_jumbo(&self) -> RxJumboR {
        RxJumboR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable LSO error"]
    #[inline(always)]
    pub fn dis_lso_err(&self) -> DisLsoErrR {
        DisLsoErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx descriptor debug"]
    #[inline(always)]
    pub fn tdsc_debug(&self) -> TdscDebugR {
        TdscDebugR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset: Setting to 1 forces the Ethernet module to a software reset state which disables the transmitter and receiver, reinitializes the FIFOs, triggers interrupt Swint for RISC to reset the system buffer pointer to the initial value Tx/Rx FDP. The values of IDR0-5 and MAR0-7 will have no changes. This bit is 1 during the reset operation, and is cleared to 0 by the Ethernet Module when the reset operation is complete."]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<'_, CrSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive checksum offload enable. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn rx_chk_sum(&mut self) -> RxChkSumW<'_, CrSpec> {
        RxChkSumW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive VLAN de-tagging enable.1: Enable. 0: Disable. 1: detag vlan and set rx status dsc.tava = 1 when ingress packet has vlantag. 0: disable detag vlan and rx status dsc.tava = 0."]
    #[inline(always)]
    pub fn rx_vlan(&mut self) -> RxVlanW<'_, CrSpec> {
        RxVlanW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: support jumbo packet receiving. The maximum packet size is 16384B(0x4000). 0: Does not support jumbo packet receiving. When rxjumbo sets to 1, the minium rx descriptor ring size is 16."]
    #[inline(always)]
    pub fn rx_jumbo(&mut self) -> RxJumboW<'_, CrSpec> {
        RxJumboW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable LSO error"]
    #[inline(always)]
    pub fn dis_lso_err(&mut self) -> DisLsoErrW<'_, CrSpec> {
        DisLsoErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Tx descriptor debug"]
    #[inline(always)]
    pub fn tdsc_debug(&mut self) -> TdscDebugW<'_, CrSpec> {
        TdscDebugW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
