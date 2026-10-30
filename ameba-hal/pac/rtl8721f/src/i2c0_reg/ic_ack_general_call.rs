#[doc = "Register `IC_ACK_GENERAL_CALL` reader"]
pub type R = crate::R<IcAckGeneralCallSpec>;
#[doc = "Register `IC_ACK_GENERAL_CALL` writer"]
pub type W = crate::W<IcAckGeneralCallSpec>;
#[doc = "Field `ACK_GEN_CALL` reader - * 1: I2C responds with a ACK when it receives a General Call. * 0: I2C does not generate General Call interrupts."]
pub type AckGenCallR = crate::BitReader;
#[doc = "Field `ACK_GEN_CALL` writer - * 1: I2C responds with a ACK when it receives a General Call. * 0: I2C does not generate General Call interrupts."]
pub type AckGenCallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - * 1: I2C responds with a ACK when it receives a General Call. * 0: I2C does not generate General Call interrupts."]
    #[inline(always)]
    pub fn ack_gen_call(&self) -> AckGenCallR {
        AckGenCallR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - * 1: I2C responds with a ACK when it receives a General Call. * 0: I2C does not generate General Call interrupts."]
    #[inline(always)]
    pub fn ack_gen_call(&mut self) -> AckGenCallW<'_, IcAckGeneralCallSpec> {
        AckGenCallW::new(self, 0)
    }
}
#[doc = "I2C ACK General Call Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_ack_general_call::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ack_general_call::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcAckGeneralCallSpec;
impl crate::RegisterSpec for IcAckGeneralCallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_ack_general_call::R`](R) reader structure"]
impl crate::Readable for IcAckGeneralCallSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_ack_general_call::W`](W) writer structure"]
impl crate::Writable for IcAckGeneralCallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_ACK_GENERAL_CALL to value 0x01"]
impl crate::Resettable for IcAckGeneralCallSpec {
    const RESET_VALUE: u32 = 0x01;
}
