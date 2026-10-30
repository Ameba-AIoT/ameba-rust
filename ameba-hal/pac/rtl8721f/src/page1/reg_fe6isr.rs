#[doc = "Register `REG_FE6ISR` reader"]
pub type R = crate::R<RegFe6isrSpec>;
#[doc = "Register `REG_FE6ISR` writer"]
pub type W = crate::W<RegFe6isrSpec>;
#[doc = "Field `BIT_FS_RX_MORE_TF1_INT` reader - Interrupt source from receiving trigger frame with more TF field = 1 not addressed to us"]
pub type BitFsRxMoreTf1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_MORE_TF0_INT` reader - interrupt source from trigger frame with more TF field = 0 not addressed to us"]
pub type BitFsRxMoreTf0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXBCMD1_ACK_INT` reader - port 1 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort1Rxbcmd1AckIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXBCMD0_ACK_INT` reader - port 1 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort1Rxbcmd0AckIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXUCMD1_ACK_INT` reader - port 1 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort1Rxucmd1AckIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RXUCMD0_ACK_INT` reader - port 1 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort1Rxucmd0AckIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_RXBCMD1_ACK_INT` reader - port 0 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort0Rxbcmd1AckIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_RXBCMD0_ACK_INT` reader - port 0 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort0Rxbcmd0AckIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_RXUCMD1_ACK_INT` reader - port 0 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
pub type BitFsPort0Rxucmd1AckIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT0_RXUCMD0_ACK_INT` reader - port 0 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
pub type BitFsPort0Rxucmd0AckIntR = crate::BitReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - Interrupt source from receiving trigger frame with more TF field = 1 not addressed to us"]
    #[inline(always)]
    pub fn bit_fs_rx_more_tf1_int(&self) -> BitFsRxMoreTf1IntR {
        BitFsRxMoreTf1IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt source from trigger frame with more TF field = 0 not addressed to us"]
    #[inline(always)]
    pub fn bit_fs_rx_more_tf0_int(&self) -> BitFsRxMoreTf0IntR {
        BitFsRxMoreTf0IntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - port 1 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd1_ack_int(&self) -> BitFsPort1Rxbcmd1AckIntR {
        BitFsPort1Rxbcmd1AckIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - port 1 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxbcmd0_ack_int(&self) -> BitFsPort1Rxbcmd0AckIntR {
        BitFsPort1Rxbcmd0AckIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - port 1 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd1_ack_int(&self) -> BitFsPort1Rxucmd1AckIntR {
        BitFsPort1Rxucmd1AckIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - port 1 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rxucmd0_ack_int(&self) -> BitFsPort1Rxucmd0AckIntR {
        BitFsPort1Rxucmd0AckIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - port 0 rx broadcast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxbcmd1_ack_int(&self) -> BitFsPort0Rxbcmd1AckIntR {
        BitFsPort0Rxbcmd1AckIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - port 0 rx broadcast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxbcmd0_ack_int(&self) -> BitFsPort0Rxbcmd0AckIntR {
        BitFsPort0Rxbcmd0AckIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - port 0 rx unicast ACK/BA/MBA with more data = 1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxucmd1_ack_int(&self) -> BitFsPort0Rxucmd1AckIntR {
        BitFsPort0Rxucmd1AckIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - port 0 rx unicast ACK/BA/MBA with more data = 0 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port0_rxucmd0_ack_int(&self) -> BitFsPort0Rxucmd0AckIntR {
        BitFsPort0Rxucmd0AckIntR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFe6isrSpec> {
        RsvdW::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe6isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fe6isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe6isrSpec;
impl crate::RegisterSpec for RegFe6isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe6isr::R`](R) reader structure"]
impl crate::Readable for RegFe6isrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fe6isr::W`](W) writer structure"]
impl crate::Writable for RegFe6isrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FE6ISR to value 0"]
impl crate::Resettable for RegFe6isrSpec {}
