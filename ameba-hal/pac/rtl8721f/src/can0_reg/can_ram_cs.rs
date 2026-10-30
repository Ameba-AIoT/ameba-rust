#[doc = "Register `CAN_RAM_CS` reader"]
pub type R = crate::R<CanRamCsSpec>;
#[doc = "Register `CAN_RAM_CS` writer"]
pub type W = crate::W<CanRamCsSpec>;
#[doc = "Field `can_ram_dlc` reader - data length"]
pub type CanRamDlcR = crate::FieldReader;
#[doc = "Field `can_ram_dlc` writer - data length"]
pub type CanRamDlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `can_ram_lost` reader - data have lost in the read buffer, means more than one frame have beed received since last read; only avaliable when rx FIFO is disabled"]
pub type CanRamLostR = crate::BitReader;
#[doc = "Field `can_ram_lost` writer - data have lost in the read buffer, means more than one frame have beed received since last read; only avaliable when rx FIFO is disabled"]
pub type CanRamLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_rxtx` reader - can frame RX or TX, 0 is for RX, 1 is for TX"]
pub type CanRamRxtxR = crate::BitReader;
#[doc = "Field `can_ram_rxtx` writer - can frame RX or TX, 0 is for RX, 1 is for TX"]
pub type CanRamRxtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_autoreply` reader - set the message buffer with remote frame auto reply function"]
pub type CanRamAutoreplyR = crate::BitReader;
#[doc = "Field `can_ram_autoreply` writer - set the message buffer with remote frame auto reply function"]
pub type CanRamAutoreplyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_edl` reader - FD frame edl bit, indicate it's a FD frame"]
pub type CanRamEdlR = crate::BitReader;
#[doc = "Field `can_ram_edl` writer - FD frame edl bit, indicate it's a FD frame"]
pub type CanRamEdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_brs` reader - FD frame brs bit, indicate switch bit timing"]
pub type CanRamBrsR = crate::BitReader;
#[doc = "Field `can_ram_brs` writer - FD frame brs bit, indicate switch bit timing"]
pub type CanRamBrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_esi` reader - FD frame esi bit, indicate error passive of message sending node, read bit, should always write 0, hardware automatic fill"]
pub type CanRamEsiR = crate::BitReader;
#[doc = "Field `can_ram_esi` writer - FD frame esi bit, indicate error passive of message sending node, read bit, should always write 0, hardware automatic fill"]
pub type CanRamEsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_timestamp` reader - the received message time stamp"]
pub type CanRamTimestampR = crate::FieldReader<u16>;
#[doc = "Field `can_ram_timestamp` writer - the received message time stamp"]
pub type CanRamTimestampW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - data length"]
    #[inline(always)]
    pub fn can_ram_dlc(&self) -> CanRamDlcR {
        CanRamDlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - data have lost in the read buffer, means more than one frame have beed received since last read; only avaliable when rx FIFO is disabled"]
    #[inline(always)]
    pub fn can_ram_lost(&self) -> CanRamLostR {
        CanRamLostR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - can frame RX or TX, 0 is for RX, 1 is for TX"]
    #[inline(always)]
    pub fn can_ram_rxtx(&self) -> CanRamRxtxR {
        CanRamRxtxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - set the message buffer with remote frame auto reply function"]
    #[inline(always)]
    pub fn can_ram_autoreply(&self) -> CanRamAutoreplyR {
        CanRamAutoreplyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FD frame edl bit, indicate it's a FD frame"]
    #[inline(always)]
    pub fn can_ram_edl(&self) -> CanRamEdlR {
        CanRamEdlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FD frame brs bit, indicate switch bit timing"]
    #[inline(always)]
    pub fn can_ram_brs(&self) -> CanRamBrsR {
        CanRamBrsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FD frame esi bit, indicate error passive of message sending node, read bit, should always write 0, hardware automatic fill"]
    #[inline(always)]
    pub fn can_ram_esi(&self) -> CanRamEsiR {
        CanRamEsiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:31 - the received message time stamp"]
    #[inline(always)]
    pub fn can_ram_timestamp(&self) -> CanRamTimestampR {
        CanRamTimestampR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - data length"]
    #[inline(always)]
    pub fn can_ram_dlc(&mut self) -> CanRamDlcW<'_, CanRamCsSpec> {
        CanRamDlcW::new(self, 0)
    }
    #[doc = "Bit 4 - data have lost in the read buffer, means more than one frame have beed received since last read; only avaliable when rx FIFO is disabled"]
    #[inline(always)]
    pub fn can_ram_lost(&mut self) -> CanRamLostW<'_, CanRamCsSpec> {
        CanRamLostW::new(self, 4)
    }
    #[doc = "Bit 5 - can frame RX or TX, 0 is for RX, 1 is for TX"]
    #[inline(always)]
    pub fn can_ram_rxtx(&mut self) -> CanRamRxtxW<'_, CanRamCsSpec> {
        CanRamRxtxW::new(self, 5)
    }
    #[doc = "Bit 6 - set the message buffer with remote frame auto reply function"]
    #[inline(always)]
    pub fn can_ram_autoreply(&mut self) -> CanRamAutoreplyW<'_, CanRamCsSpec> {
        CanRamAutoreplyW::new(self, 6)
    }
    #[doc = "Bit 8 - FD frame edl bit, indicate it's a FD frame"]
    #[inline(always)]
    pub fn can_ram_edl(&mut self) -> CanRamEdlW<'_, CanRamCsSpec> {
        CanRamEdlW::new(self, 8)
    }
    #[doc = "Bit 9 - FD frame brs bit, indicate switch bit timing"]
    #[inline(always)]
    pub fn can_ram_brs(&mut self) -> CanRamBrsW<'_, CanRamCsSpec> {
        CanRamBrsW::new(self, 9)
    }
    #[doc = "Bit 10 - FD frame esi bit, indicate error passive of message sending node, read bit, should always write 0, hardware automatic fill"]
    #[inline(always)]
    pub fn can_ram_esi(&mut self) -> CanRamEsiW<'_, CanRamCsSpec> {
        CanRamEsiW::new(self, 10)
    }
    #[doc = "Bits 16:31 - the received message time stamp"]
    #[inline(always)]
    pub fn can_ram_timestamp(&mut self) -> CanRamTimestampW<'_, CanRamCsSpec> {
        CanRamTimestampW::new(self, 16)
    }
}
#[doc = "CAN message type control\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamCsSpec;
impl crate::RegisterSpec for CanRamCsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_cs::R`](R) reader structure"]
impl crate::Readable for CanRamCsSpec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_cs::W`](W) writer structure"]
impl crate::Writable for CanRamCsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_CS to value 0"]
impl crate::Resettable for CanRamCsSpec {}
