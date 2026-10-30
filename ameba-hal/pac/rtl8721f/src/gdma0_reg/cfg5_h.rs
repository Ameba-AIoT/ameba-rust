#[doc = "Register `CFG5_H` reader"]
pub type R = crate::R<Cfg5HSpec>;
#[doc = "Register `CFG5_H` writer"]
pub type W = crate::W<Cfg5HSpec>;
#[doc = "Field `CFG5_H_FCMODE` reader - Flow Control Mode. Determines when source transaction requests are serviced when the Destination Peripheral is the flow controller. 0 = Source transaction requests are servied when they occur. Data pre-fetching is enabled. 1 = Source transaction requests are not serviced until a destination transaction request occurs. Data pre-fetching is disabled. Note: Please keep this bit 0 when Transfer Type and Flow Control is not \"destination peripheral is the flow controller and transfer type is peripheral to peripheral\" (TT_FC!=7)."]
pub type Cfg5HFcmodeR = crate::BitReader;
#[doc = "Field `CFG5_H_FCMODE` writer - Flow Control Mode. Determines when source transaction requests are serviced when the Destination Peripheral is the flow controller. 0 = Source transaction requests are servied when they occur. Data pre-fetching is enabled. 1 = Source transaction requests are not serviced until a destination transaction request occurs. Data pre-fetching is disabled. Note: Please keep this bit 0 when Transfer Type and Flow Control is not \"destination peripheral is the flow controller and transfer type is peripheral to peripheral\" (TT_FC!=7)."]
pub type Cfg5HFcmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG5_H_PROTCTL` reader - Protection Control.Set the bit 35 to 0 will enable the channel secure DMA transfer. Set the bit 35 to 1 will enable the channel non-secure DMAC transfer. Note: if user not define DMAC_SECURE_MODE, it only can be readed."]
pub type Cfg5HProtctlR = crate::BitReader;
#[doc = "Field `CFG5_H_PROTCTL` writer - Protection Control.Set the bit 35 to 0 will enable the channel secure DMA transfer. Set the bit 35 to 1 will enable the channel non-secure DMAC transfer. Note: if user not define DMAC_SECURE_MODE, it only can be readed."]
pub type Cfg5HProtctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG5_H_SRC_PER` reader - Assigns a hardware handshaking interface(0 - DMAC_NUM_HS_INT-1) to the source of channel x if the CFGx.HS_SEL_SRC field is 0; otherwise, this field is ignored. The channel can then communicate with the source peripheral connected to that interface through the assigned hardware handshaking interface. NOTE1: For correct RTK_DMAC operation, only one peripheral (source or destination) should be assigned to the same handshaking interface. NOTE2: This field does not exist if the configuration parameter DMAC_NUM_HS_INT is set to 0."]
pub type Cfg5HSrcPerR = crate::FieldReader;
#[doc = "Field `CFG5_H_SRC_PER` writer - Assigns a hardware handshaking interface(0 - DMAC_NUM_HS_INT-1) to the source of channel x if the CFGx.HS_SEL_SRC field is 0; otherwise, this field is ignored. The channel can then communicate with the source peripheral connected to that interface through the assigned hardware handshaking interface. NOTE1: For correct RTK_DMAC operation, only one peripheral (source or destination) should be assigned to the same handshaking interface. NOTE2: This field does not exist if the configuration parameter DMAC_NUM_HS_INT is set to 0."]
pub type Cfg5HSrcPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG5_H_DEST_PER` reader - Assigns a hardware handshaking interface(0 - DMAC_NUM_HS_INT-1) to the destination of channel x if the FGx.HS_SEL_DST field is 0; otherwise, this field is ignored. The channel can then communicate with the destinationperipheral connected to that interface through the assignedhardware handshaking interface. NOTE1: For correct DMA operation, only one peripheral (sourceor destination) should be assigned to the same handshaking interface NOTE2: This field does not exist if the configuration parameterDMAC_NUM_HS_INT is set to 0."]
pub type Cfg5HDestPerR = crate::FieldReader;
#[doc = "Field `CFG5_H_DEST_PER` writer - Assigns a hardware handshaking interface(0 - DMAC_NUM_HS_INT-1) to the destination of channel x if the FGx.HS_SEL_DST field is 0; otherwise, this field is ignored. The channel can then communicate with the destinationperipheral connected to that interface through the assignedhardware handshaking interface. NOTE1: For correct DMA operation, only one peripheral (sourceor destination) should be assigned to the same handshaking interface NOTE2: This field does not exist if the configuration parameterDMAC_NUM_HS_INT is set to 0."]
pub type Cfg5HDestPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG5_H_Extended_SRC_PER1` reader - The extened 4th bit of SRC_PER when hardware handshake interfaces number is configured to 32."]
pub type Cfg5HExtendedSrcPer1R = crate::BitReader;
#[doc = "Field `CFG5_H_Extended_SRC_PER1` writer - The extened 4th bit of SRC_PER when hardware handshake interfaces number is configured to 32."]
pub type Cfg5HExtendedSrcPer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG5_H_Extended_DEST_PER1` reader - The extened 4th bit of DEST_PER when hardware handshake interfaces number is configured to 32."]
pub type Cfg5HExtendedDestPer1R = crate::BitReader;
#[doc = "Field `CFG5_H_Extended_DEST_PER1` writer - The extened 4th bit of DEST_PER when hardware handshake interfaces number is configured to 32."]
pub type Cfg5HExtendedDestPer1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flow Control Mode. Determines when source transaction requests are serviced when the Destination Peripheral is the flow controller. 0 = Source transaction requests are servied when they occur. Data pre-fetching is enabled. 1 = Source transaction requests are not serviced until a destination transaction request occurs. Data pre-fetching is disabled. Note: Please keep this bit 0 when Transfer Type and Flow Control is not \"destination peripheral is the flow controller and transfer type is peripheral to peripheral\" (TT_FC!=7)."]
    #[inline(always)]
    pub fn cfg5_h_fcmode(&self) -> Cfg5HFcmodeR {
        Cfg5HFcmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Protection Control.Set the bit 35 to 0 will enable the channel secure DMA transfer. Set the bit 35 to 1 will enable the channel non-secure DMAC transfer. Note: if user not define DMAC_SECURE_MODE, it only can be readed."]
    #[inline(always)]
    pub fn cfg5_h_protctl(&self) -> Cfg5HProtctlR {
        Cfg5HProtctlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 7:10 - Assigns a hardware handshaking interface(0 - DMAC_NUM_HS_INT-1) to the source of channel x if the CFGx.HS_SEL_SRC field is 0; otherwise, this field is ignored. The channel can then communicate with the source peripheral connected to that interface through the assigned hardware handshaking interface. NOTE1: For correct RTK_DMAC operation, only one peripheral (source or destination) should be assigned to the same handshaking interface. NOTE2: This field does not exist if the configuration parameter DMAC_NUM_HS_INT is set to 0."]
    #[inline(always)]
    pub fn cfg5_h_src_per(&self) -> Cfg5HSrcPerR {
        Cfg5HSrcPerR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - Assigns a hardware handshaking interface(0 - DMAC_NUM_HS_INT-1) to the destination of channel x if the FGx.HS_SEL_DST field is 0; otherwise, this field is ignored. The channel can then communicate with the destinationperipheral connected to that interface through the assignedhardware handshaking interface. NOTE1: For correct DMA operation, only one peripheral (sourceor destination) should be assigned to the same handshaking interface NOTE2: This field does not exist if the configuration parameterDMAC_NUM_HS_INT is set to 0."]
    #[inline(always)]
    pub fn cfg5_h_dest_per(&self) -> Cfg5HDestPerR {
        Cfg5HDestPerR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - The extened 4th bit of SRC_PER when hardware handshake interfaces number is configured to 32."]
    #[inline(always)]
    pub fn cfg5_h_extended_src_per1(&self) -> Cfg5HExtendedSrcPer1R {
        Cfg5HExtendedSrcPer1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The extened 4th bit of DEST_PER when hardware handshake interfaces number is configured to 32."]
    #[inline(always)]
    pub fn cfg5_h_extended_dest_per1(&self) -> Cfg5HExtendedDestPer1R {
        Cfg5HExtendedDestPer1R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Mode. Determines when source transaction requests are serviced when the Destination Peripheral is the flow controller. 0 = Source transaction requests are servied when they occur. Data pre-fetching is enabled. 1 = Source transaction requests are not serviced until a destination transaction request occurs. Data pre-fetching is disabled. Note: Please keep this bit 0 when Transfer Type and Flow Control is not \"destination peripheral is the flow controller and transfer type is peripheral to peripheral\" (TT_FC!=7)."]
    #[inline(always)]
    pub fn cfg5_h_fcmode(&mut self) -> Cfg5HFcmodeW<'_, Cfg5HSpec> {
        Cfg5HFcmodeW::new(self, 0)
    }
    #[doc = "Bit 3 - Protection Control.Set the bit 35 to 0 will enable the channel secure DMA transfer. Set the bit 35 to 1 will enable the channel non-secure DMAC transfer. Note: if user not define DMAC_SECURE_MODE, it only can be readed."]
    #[inline(always)]
    pub fn cfg5_h_protctl(&mut self) -> Cfg5HProtctlW<'_, Cfg5HSpec> {
        Cfg5HProtctlW::new(self, 3)
    }
    #[doc = "Bits 7:10 - Assigns a hardware handshaking interface(0 - DMAC_NUM_HS_INT-1) to the source of channel x if the CFGx.HS_SEL_SRC field is 0; otherwise, this field is ignored. The channel can then communicate with the source peripheral connected to that interface through the assigned hardware handshaking interface. NOTE1: For correct RTK_DMAC operation, only one peripheral (source or destination) should be assigned to the same handshaking interface. NOTE2: This field does not exist if the configuration parameter DMAC_NUM_HS_INT is set to 0."]
    #[inline(always)]
    pub fn cfg5_h_src_per(&mut self) -> Cfg5HSrcPerW<'_, Cfg5HSpec> {
        Cfg5HSrcPerW::new(self, 7)
    }
    #[doc = "Bits 11:14 - Assigns a hardware handshaking interface(0 - DMAC_NUM_HS_INT-1) to the destination of channel x if the FGx.HS_SEL_DST field is 0; otherwise, this field is ignored. The channel can then communicate with the destinationperipheral connected to that interface through the assignedhardware handshaking interface. NOTE1: For correct DMA operation, only one peripheral (sourceor destination) should be assigned to the same handshaking interface NOTE2: This field does not exist if the configuration parameterDMAC_NUM_HS_INT is set to 0."]
    #[inline(always)]
    pub fn cfg5_h_dest_per(&mut self) -> Cfg5HDestPerW<'_, Cfg5HSpec> {
        Cfg5HDestPerW::new(self, 11)
    }
    #[doc = "Bit 15 - The extened 4th bit of SRC_PER when hardware handshake interfaces number is configured to 32."]
    #[inline(always)]
    pub fn cfg5_h_extended_src_per1(&mut self) -> Cfg5HExtendedSrcPer1W<'_, Cfg5HSpec> {
        Cfg5HExtendedSrcPer1W::new(self, 15)
    }
    #[doc = "Bit 16 - The extened 4th bit of DEST_PER when hardware handshake interfaces number is configured to 32."]
    #[inline(always)]
    pub fn cfg5_h_extended_dest_per1(&mut self) -> Cfg5HExtendedDestPer1W<'_, Cfg5HSpec> {
        Cfg5HExtendedDestPer1W::new(self, 16)
    }
}
#[doc = "Channel 5 high-32 bits Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg5_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg5_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg5HSpec;
impl crate::RegisterSpec for Cfg5HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg5_h::R`](R) reader structure"]
impl crate::Readable for Cfg5HSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg5_h::W`](W) writer structure"]
impl crate::Writable for Cfg5HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG5_H to value 0x08"]
impl crate::Resettable for Cfg5HSpec {
    const RESET_VALUE: u32 = 0x08;
}
