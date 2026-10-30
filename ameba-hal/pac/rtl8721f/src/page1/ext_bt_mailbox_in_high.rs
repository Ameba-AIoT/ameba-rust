#[doc = "Register `EXT_BT_MAILBOX_IN_HIGH` reader"]
pub type R = crate::R<ExtBtMailboxInHighSpec>;
#[doc = "Field `ext_mailbox_data_81_to51_h` reader - "]
pub type ExtMailboxData81To51HR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ext_mailbox_data_81_to51_h(&self) -> ExtMailboxData81To51HR {
        ExtMailboxData81To51HR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_bt_mailbox_in_high::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtBtMailboxInHighSpec;
impl crate::RegisterSpec for ExtBtMailboxInHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_bt_mailbox_in_high::R`](R) reader structure"]
impl crate::Readable for ExtBtMailboxInHighSpec {}
#[doc = "`reset()` method sets EXT_BT_MAILBOX_IN_HIGH to value 0"]
impl crate::Resettable for ExtBtMailboxInHighSpec {}
