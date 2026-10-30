#[doc = "Register `EXT_BT_MAILBOX_IN_LOW` reader"]
pub type R = crate::R<ExtBtMailboxInLowSpec>;
#[doc = "Field `ext_mailbox_data_81to51_l` reader - "]
pub type ExtMailboxData81to51LR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ext_mailbox_data_81to51_l(&self) -> ExtMailboxData81to51LR {
        ExtMailboxData81to51LR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_bt_mailbox_in_low::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtBtMailboxInLowSpec;
impl crate::RegisterSpec for ExtBtMailboxInLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_bt_mailbox_in_low::R`](R) reader structure"]
impl crate::Readable for ExtBtMailboxInLowSpec {}
#[doc = "`reset()` method sets EXT_BT_MAILBOX_IN_LOW to value 0"]
impl crate::Resettable for ExtBtMailboxInLowSpec {}
