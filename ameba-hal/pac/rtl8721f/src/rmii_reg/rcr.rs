#[doc = "Register `RCR` reader"]
pub type R = crate::R<RcrSpec>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RcrSpec>;
#[doc = "Field `AAP` reader - Set to 1 to accept all packets with physical destination address, 0 to reject."]
pub type AapR = crate::BitReader;
#[doc = "Field `AAP` writer - Set to 1 to accept all packets with physical destination address, 0 to reject."]
pub type AapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APM` reader - Set to 1 to accept physical match packets, 0 to reject."]
pub type ApmR = crate::BitReader;
#[doc = "Field `APM` writer - Set to 1 to accept physical match packets, 0 to reject."]
pub type ApmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AM` reader - Set to 1 to accept multicast packets, 0 to reject. RC_REG.AB and AM should be enabled simulatanously."]
pub type AmR = crate::BitReader;
#[doc = "Field `AM` writer - Set to 1 to accept multicast packets, 0 to reject. RC_REG.AB and AM should be enabled simulatanously."]
pub type AmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AB` reader - Set to 1 to accept broadcast packets, 0 to reject."]
pub type AbR = crate::BitReader;
#[doc = "Field `AB` writer - Set to 1 to accept broadcast packets, 0 to reject."]
pub type AbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR` reader - Accept Runt: This bit allows the receiver to accept packets that are smaller than 64 bytes. The packet must be at least 8 bytes long to be accepted as a runt. Set to 1 to accept runt packets."]
pub type ArR = crate::BitReader;
#[doc = "Field `AR` writer - Accept Runt: This bit allows the receiver to accept packets that are smaller than 64 bytes. The packet must be at least 8 bytes long to be accepted as a runt. Set to 1 to accept runt packets."]
pub type ArW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AER` reader - Accept Error Packet: When set to 1, all packets with CRC error, alignment error, and/or collided fragments will be accepted. Rxerrc keeps, Rxokc +1, Rx status dsc.CRC =1, Rx status dsc.RCDF =0. When set to 0, all packets with CRC error, alignment error, and/or collided fragments will be rejected. Rxerrc +1, Rxokc keeps, Rx status dsc.CRC =0, Rx status dsc.RCDF =1."]
pub type AerR = crate::BitReader;
#[doc = "Field `AER` writer - Accept Error Packet: When set to 1, all packets with CRC error, alignment error, and/or collided fragments will be accepted. Rxerrc keeps, Rxokc +1, Rx status dsc.CRC =1, Rx status dsc.RCDF =0. When set to 0, all packets with CRC error, alignment error, and/or collided fragments will be rejected. Rxerrc +1, Rxokc keeps, Rx status dsc.CRC =0, Rx status dsc.RCDF =1."]
pub type AerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFlow` reader - Accept flow control : When set to 1, flow control packet will also be received & DMA to rx buffer for debug. Default is 0."]
pub type AflowR = crate::BitReader;
#[doc = "Field `AFlow` writer - Accept flow control : When set to 1, flow control packet will also be received & DMA to rx buffer for debug. Default is 0."]
pub type AflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HomePNA` reader - Home PNA mode"]
pub type HomePnaR = crate::BitReader;
#[doc = "Field `HomePNA` writer - Home PNA mode"]
pub type HomePnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to 1 to accept all packets with physical destination address, 0 to reject."]
    #[inline(always)]
    pub fn aap(&self) -> AapR {
        AapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to 1 to accept physical match packets, 0 to reject."]
    #[inline(always)]
    pub fn apm(&self) -> ApmR {
        ApmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to 1 to accept multicast packets, 0 to reject. RC_REG.AB and AM should be enabled simulatanously."]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to 1 to accept broadcast packets, 0 to reject."]
    #[inline(always)]
    pub fn ab(&self) -> AbR {
        AbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Accept Runt: This bit allows the receiver to accept packets that are smaller than 64 bytes. The packet must be at least 8 bytes long to be accepted as a runt. Set to 1 to accept runt packets."]
    #[inline(always)]
    pub fn ar(&self) -> ArR {
        ArR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Accept Error Packet: When set to 1, all packets with CRC error, alignment error, and/or collided fragments will be accepted. Rxerrc keeps, Rxokc +1, Rx status dsc.CRC =1, Rx status dsc.RCDF =0. When set to 0, all packets with CRC error, alignment error, and/or collided fragments will be rejected. Rxerrc +1, Rxokc keeps, Rx status dsc.CRC =0, Rx status dsc.RCDF =1."]
    #[inline(always)]
    pub fn aer(&self) -> AerR {
        AerR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Accept flow control : When set to 1, flow control packet will also be received & DMA to rx buffer for debug. Default is 0."]
    #[inline(always)]
    pub fn aflow(&self) -> AflowR {
        AflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Home PNA mode"]
    #[inline(always)]
    pub fn home_pna(&self) -> HomePnaR {
        HomePnaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1 to accept all packets with physical destination address, 0 to reject."]
    #[inline(always)]
    pub fn aap(&mut self) -> AapW<'_, RcrSpec> {
        AapW::new(self, 0)
    }
    #[doc = "Bit 1 - Set to 1 to accept physical match packets, 0 to reject."]
    #[inline(always)]
    pub fn apm(&mut self) -> ApmW<'_, RcrSpec> {
        ApmW::new(self, 1)
    }
    #[doc = "Bit 2 - Set to 1 to accept multicast packets, 0 to reject. RC_REG.AB and AM should be enabled simulatanously."]
    #[inline(always)]
    pub fn am(&mut self) -> AmW<'_, RcrSpec> {
        AmW::new(self, 2)
    }
    #[doc = "Bit 3 - Set to 1 to accept broadcast packets, 0 to reject."]
    #[inline(always)]
    pub fn ab(&mut self) -> AbW<'_, RcrSpec> {
        AbW::new(self, 3)
    }
    #[doc = "Bit 4 - Accept Runt: This bit allows the receiver to accept packets that are smaller than 64 bytes. The packet must be at least 8 bytes long to be accepted as a runt. Set to 1 to accept runt packets."]
    #[inline(always)]
    pub fn ar(&mut self) -> ArW<'_, RcrSpec> {
        ArW::new(self, 4)
    }
    #[doc = "Bit 5 - Accept Error Packet: When set to 1, all packets with CRC error, alignment error, and/or collided fragments will be accepted. Rxerrc keeps, Rxokc +1, Rx status dsc.CRC =1, Rx status dsc.RCDF =0. When set to 0, all packets with CRC error, alignment error, and/or collided fragments will be rejected. Rxerrc +1, Rxokc keeps, Rx status dsc.CRC =0, Rx status dsc.RCDF =1."]
    #[inline(always)]
    pub fn aer(&mut self) -> AerW<'_, RcrSpec> {
        AerW::new(self, 5)
    }
    #[doc = "Bit 6 - Accept flow control : When set to 1, flow control packet will also be received & DMA to rx buffer for debug. Default is 0."]
    #[inline(always)]
    pub fn aflow(&mut self) -> AflowW<'_, RcrSpec> {
        AflowW::new(self, 6)
    }
    #[doc = "Bit 7 - Home PNA mode"]
    #[inline(always)]
    pub fn home_pna(&mut self) -> HomePnaW<'_, RcrSpec> {
        HomePnaW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcrSpec;
impl crate::RegisterSpec for RcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RcrSpec {}
