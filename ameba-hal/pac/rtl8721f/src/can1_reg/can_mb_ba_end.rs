#[doc = "Register `CAN_MB_BA_END` reader"]
pub type R = crate::R<CanMbBaEndSpec>;
#[doc = "Register `CAN_MB_BA_END` writer"]
pub type W = crate::W<CanMbBaEndSpec>;
#[doc = "Field `can_msg_ba_end` reader - End of message buffer base address in RAM"]
pub type CanMsgBaEndR = crate::FieldReader<u16>;
#[doc = "Field `can_msg_ba_end` writer - End of message buffer base address in RAM"]
pub type CanMsgBaEndW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End of message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg_ba_end(&self) -> CanMsgBaEndR {
        CanMsgBaEndR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End of message buffer base address in RAM"]
    #[inline(always)]
    pub fn can_msg_ba_end(&mut self) -> CanMsgBaEndW<'_, CanMbBaEndSpec> {
        CanMsgBaEndW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb_ba_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb_ba_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMbBaEndSpec;
impl crate::RegisterSpec for CanMbBaEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb_ba_end::R`](R) reader structure"]
impl crate::Readable for CanMbBaEndSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb_ba_end::W`](W) writer structure"]
impl crate::Writable for CanMbBaEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB_BA_END to value 0x60"]
impl crate::Resettable for CanMbBaEndSpec {
    const RESET_VALUE: u32 = 0x60;
}
