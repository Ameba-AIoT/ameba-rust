#[doc = "Register `EVENT_CMD_CNT` reader"]
pub type R = crate::R<EventCmdCntSpec>;
#[doc = "Field `CMD_NUM` reader - The number of master command"]
pub type CmdNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - The number of master command"]
    #[inline(always)]
    pub fn cmd_num(&self) -> CmdNumR {
        CmdNumR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "The number of master conmmand\n\nYou can [`read`](crate::Reg::read) this register and get [`event_cmd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventCmdCntSpec;
impl crate::RegisterSpec for EventCmdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_cmd_cnt::R`](R) reader structure"]
impl crate::Readable for EventCmdCntSpec {}
#[doc = "`reset()` method sets EVENT_CMD_CNT to value 0"]
impl crate::Resettable for EventCmdCntSpec {}
