#[doc = "Register `REG_FE6IMR` reader"]
pub type R = crate::R<RegFe6imrSpec>;
#[doc = "Register `REG_FE6IMR` writer"]
pub type W = crate::W<RegFe6imrSpec>;
#[doc = "Field `BIT_FS_RX_MORE_TF1_INT_EN` reader - Interrupt source from receiving trigger frame with more TF field = 1 not addressed to us"]
pub type BitFsRxMoreTf1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_MORE_TF1_INT_EN` writer - Interrupt source from receiving trigger frame with more TF field = 1 not addressed to us"]
pub type BitFsRxMoreTf1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_MORE_TF0_INT_EN` reader - interrupt source from trigger frame with more TF field = 0 not addressed to us"]
pub type BitFsRxMoreTf0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_MORE_TF0_INT_EN` writer - interrupt source from trigger frame with more TF field = 0 not addressed to us"]
pub type BitFsRxMoreTf0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RXBCMD1_ACK_INT_EN` reader - port 1 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort1Rxbcmd1AckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXBCMD1_ACK_INT_EN` writer - port 1 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort1Rxbcmd1AckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RXBCMD0_ACK_INT_EN` reader - port 1 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort1Rxbcmd0AckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXBCMD0_ACK_INT_EN` writer - port 1 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort1Rxbcmd0AckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RXUCMD1_ACK_INT_EN` reader - port 1 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort1Rxucmd1AckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXUCMD1_ACK_INT_EN` writer - port 1 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort1Rxucmd1AckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RXUCMD0_ACK_INT_EN` reader - port 1 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort1Rxucmd0AckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXUCMD0_ACK_INT_EN` writer - port 1 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort1Rxucmd0AckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT0_RXBCMD1_ACK_INT_EN` reader - port 0 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort0Rxbcmd1AckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_RXBCMD1_ACK_INT_EN` writer - port 0 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort0Rxbcmd1AckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT0_RXBCMD0_ACK_INT_EN` reader - port 0 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort0Rxbcmd0AckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_RXBCMD0_ACK_INT_EN` writer - port 0 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort0Rxbcmd0AckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT0_RXUCMD1_ACK_INT_EN` reader - port 0 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort0Rxucmd1AckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_RXUCMD1_ACK_INT_EN` writer - port 0 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort0Rxucmd1AckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT0_RXUCMD0_ACK_INT_EN` reader - port 0 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort0Rxucmd0AckIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_RXUCMD0_ACK_INT_EN` writer - port 0 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort0Rxucmd0AckIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - Interrupt source from receiving trigger frame with more TF field = 1 not addressed to us"]
    #[inline(always)]
    pub fn bit_fs_rx_more_tf1_int_en(&self) -> BitFsRxMoreTf1IntEnR {
        BitFsRxMoreTf1IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt source from trigger frame with more TF field = 0 not addressed to us"]
    #[inline(always)]
    pub fn bit_fs_rx_more_tf0_int_en(&self) -> BitFsRxMoreTf0IntEnR {
        BitFsRxMoreTf0IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - port 1 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd1_ack_int_en(&self) -> BitFsPort1Rxbcmd1AckIntEnR {
        BitFsPort1Rxbcmd1AckIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - port 1 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd0_ack_int_en(&self) -> BitFsPort1Rxbcmd0AckIntEnR {
        BitFsPort1Rxbcmd0AckIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - port 1 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd1_ack_int_en(&self) -> BitFsPort1Rxucmd1AckIntEnR {
        BitFsPort1Rxucmd1AckIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - port 1 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd0_ack_int_en(&self) -> BitFsPort1Rxucmd0AckIntEnR {
        BitFsPort1Rxucmd0AckIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - port 0 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxbcmd1_ack_int_en(&self) -> BitFsPort0Rxbcmd1AckIntEnR {
        BitFsPort0Rxbcmd1AckIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - port 0 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxbcmd0_ack_int_en(&self) -> BitFsPort0Rxbcmd0AckIntEnR {
        BitFsPort0Rxbcmd0AckIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - port 0 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxucmd1_ack_int_en(&self) -> BitFsPort0Rxucmd1AckIntEnR {
        BitFsPort0Rxucmd1AckIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - port 0 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxucmd0_ack_int_en(&self) -> BitFsPort0Rxucmd0AckIntEnR {
        BitFsPort0Rxucmd0AckIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt source from receiving trigger frame with more TF field = 1 not addressed to us"]
    #[inline(always)]
    pub fn bit_fs_rx_more_tf1_int_en(&mut self) -> BitFsRxMoreTf1IntEnW<'_, RegFe6imrSpec> {
        BitFsRxMoreTf1IntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt source from trigger frame with more TF field = 0 not addressed to us"]
    #[inline(always)]
    pub fn bit_fs_rx_more_tf0_int_en(&mut self) -> BitFsRxMoreTf0IntEnW<'_, RegFe6imrSpec> {
        BitFsRxMoreTf0IntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - port 1 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd1_ack_int_en(
        &mut self,
    ) -> BitFsPort1Rxbcmd1AckIntEnW<'_, RegFe6imrSpec> {
        BitFsPort1Rxbcmd1AckIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - port 1 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd0_ack_int_en(
        &mut self,
    ) -> BitFsPort1Rxbcmd0AckIntEnW<'_, RegFe6imrSpec> {
        BitFsPort1Rxbcmd0AckIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - port 1 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd1_ack_int_en(
        &mut self,
    ) -> BitFsPort1Rxucmd1AckIntEnW<'_, RegFe6imrSpec> {
        BitFsPort1Rxucmd1AckIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - port 1 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd0_ack_int_en(
        &mut self,
    ) -> BitFsPort1Rxucmd0AckIntEnW<'_, RegFe6imrSpec> {
        BitFsPort1Rxucmd0AckIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - port 0 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxbcmd1_ack_int_en(
        &mut self,
    ) -> BitFsPort0Rxbcmd1AckIntEnW<'_, RegFe6imrSpec> {
        BitFsPort0Rxbcmd1AckIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - port 0 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxbcmd0_ack_int_en(
        &mut self,
    ) -> BitFsPort0Rxbcmd0AckIntEnW<'_, RegFe6imrSpec> {
        BitFsPort0Rxbcmd0AckIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - port 0 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxucmd1_ack_int_en(
        &mut self,
    ) -> BitFsPort0Rxucmd1AckIntEnW<'_, RegFe6imrSpec> {
        BitFsPort0Rxucmd1AckIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - port 0 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxucmd0_ack_int_en(
        &mut self,
    ) -> BitFsPort0Rxucmd0AckIntEnW<'_, RegFe6imrSpec> {
        BitFsPort0Rxucmd0AckIntEnW::new(self, 9)
    }
    #[doc = "Bits 10:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFe6imrSpec> {
        RsvdW::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe6imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fe6imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe6imrSpec;
impl crate::RegisterSpec for RegFe6imrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe6imr::R`](R) reader structure"]
impl crate::Readable for RegFe6imrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fe6imr::W`](W) writer structure"]
impl crate::Writable for RegFe6imrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FE6IMR to value 0"]
impl crate::Resettable for RegFe6imrSpec {}
