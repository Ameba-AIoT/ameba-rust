#[doc = "Register `REG_RX_SUPPER_ADDRESS3_L` reader"]
pub type R = crate::R<RegRxSupperAddress3LSpec>;
#[doc = "Register `REG_RX_SUPPER_ADDRESS3_L` writer"]
pub type W = crate::W<RegRxSupperAddress3LSpec>;
#[doc = "Field `r_sup_addr3_l` reader - Address3 for data packet check. When address3 is match, RX will receive the packet to RXFF without considering other filter condition except crc32"]
pub type RSupAddr3LR = crate::FieldReader<u32>;
#[doc = "Field `r_sup_addr3_l` writer - Address3 for data packet check. When address3 is match, RX will receive the packet to RXFF without considering other filter condition except crc32"]
pub type RSupAddr3LW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address3 for data packet check. When address3 is match, RX will receive the packet to RXFF without considering other filter condition except crc32"]
    #[inline(always)]
    pub fn r_sup_addr3_l(&self) -> RSupAddr3LR {
        RSupAddr3LR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address3 for data packet check. When address3 is match, RX will receive the packet to RXFF without considering other filter condition except crc32"]
    #[inline(always)]
    pub fn r_sup_addr3_l(&mut self) -> RSupAddr3LW<'_, RegRxSupperAddress3LSpec> {
        RSupAddr3LW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_supper_address3_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_supper_address3_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxSupperAddress3LSpec;
impl crate::RegisterSpec for RegRxSupperAddress3LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_supper_address3_l::R`](R) reader structure"]
impl crate::Readable for RegRxSupperAddress3LSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_supper_address3_l::W`](W) writer structure"]
impl crate::Writable for RegRxSupperAddress3LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_SUPPER_ADDRESS3_L to value 0"]
impl crate::Resettable for RegRxSupperAddress3LSpec {}
