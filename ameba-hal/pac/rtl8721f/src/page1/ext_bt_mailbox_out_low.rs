#[doc = "Register `EXT_BT_MAILBOX_OUT_LOW` reader"]
pub type R = crate::R<ExtBtMailboxOutLowSpec>;
#[doc = "Register `EXT_BT_MAILBOX_OUT_LOW` writer"]
pub type W = crate::W<ExtBtMailboxOutLowSpec>;
#[doc = "Field `ext_mailbox_data_51to81_l` reader - "]
pub type ExtMailboxData51to81LR = crate::FieldReader<u32>;
#[doc = "Field `ext_mailbox_data_51to81_l` writer - "]
pub type ExtMailboxData51to81LW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ext_mailbox_data_51to81_l(&self) -> ExtMailboxData51to81LR {
        ExtMailboxData51to81LR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ext_mailbox_data_51to81_l(
        &mut self,
    ) -> ExtMailboxData51to81LW<'_, ExtBtMailboxOutLowSpec> {
        ExtMailboxData51to81LW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_bt_mailbox_out_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_bt_mailbox_out_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtBtMailboxOutLowSpec;
impl crate::RegisterSpec for ExtBtMailboxOutLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_bt_mailbox_out_low::R`](R) reader structure"]
impl crate::Readable for ExtBtMailboxOutLowSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_bt_mailbox_out_low::W`](W) writer structure"]
impl crate::Writable for ExtBtMailboxOutLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_BT_MAILBOX_OUT_LOW to value 0"]
impl crate::Resettable for ExtBtMailboxOutLowSpec {}
