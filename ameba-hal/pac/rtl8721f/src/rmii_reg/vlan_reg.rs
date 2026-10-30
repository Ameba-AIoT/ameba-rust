#[doc = "Register `VLAN_REG` reader"]
pub type R = crate::R<VlanRegSpec>;
#[doc = "Register `VLAN_REG` writer"]
pub type W = crate::W<VlanRegSpec>;
#[doc = "Field `TDSC_VLAN_TYPE` reader - 1: tx command descriptor.tx_vlan_action is for s-tag. 0: tx command descriptor.tx_vlan_action is for c-tag."]
pub type TdscVlanTypeR = crate::BitReader;
#[doc = "Field `TDSC_VLAN_TYPE` writer - 1: tx command descriptor.tx_vlan_action is for s-tag. 0: tx command descriptor.tx_vlan_action is for c-tag."]
pub type TdscVlanTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAG_PID` reader - Set the s-tag protocol identifier. This field is valid only when COM_REG.TDSC_VLAN_TYPE is high."]
pub type StagPidR = crate::FieldReader<u16>;
#[doc = "Field `STAG_PID` writer - Set the s-tag protocol identifier. This field is valid only when COM_REG.TDSC_VLAN_TYPE is high."]
pub type StagPidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 15 - 1: tx command descriptor.tx_vlan_action is for s-tag. 0: tx command descriptor.tx_vlan_action is for c-tag."]
    #[inline(always)]
    pub fn tdsc_vlan_type(&self) -> TdscVlanTypeR {
        TdscVlanTypeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Set the s-tag protocol identifier. This field is valid only when COM_REG.TDSC_VLAN_TYPE is high."]
    #[inline(always)]
    pub fn stag_pid(&self) -> StagPidR {
        StagPidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - 1: tx command descriptor.tx_vlan_action is for s-tag. 0: tx command descriptor.tx_vlan_action is for c-tag."]
    #[inline(always)]
    pub fn tdsc_vlan_type(&mut self) -> TdscVlanTypeW<'_, VlanRegSpec> {
        TdscVlanTypeW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Set the s-tag protocol identifier. This field is valid only when COM_REG.TDSC_VLAN_TYPE is high."]
    #[inline(always)]
    pub fn stag_pid(&mut self) -> StagPidW<'_, VlanRegSpec> {
        StagPidW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`vlan_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlan_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VlanRegSpec;
impl crate::RegisterSpec for VlanRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan_reg::R`](R) reader structure"]
impl crate::Readable for VlanRegSpec {}
#[doc = "`write(|w| ..)` method takes [`vlan_reg::W`](W) writer structure"]
impl crate::Writable for VlanRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VLAN_REG to value 0x0008_0000"]
impl crate::Resettable for VlanRegSpec {
    const RESET_VALUE: u32 = 0x0008_0000;
}
