#[doc = "Register `EXT_BT_MAILBOX_OUT_HIGH` reader"]
pub type R = crate::R<ExtBtMailboxOutHighSpec>;
#[doc = "Register `EXT_BT_MAILBOX_OUT_HIGH` writer"]
pub type W = crate::W<ExtBtMailboxOutHighSpec>;
#[doc = "Field `ext_mailbox_data_51to81_h` reader - "]
pub type ExtMailboxData51to81HR = crate::FieldReader<u32>;
#[doc = "Field `ext_mailbox_data_51to81_h` writer - "]
pub type ExtMailboxData51to81HW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ext_mailbox_data_51to81_h(&self) -> ExtMailboxData51to81HR {
        ExtMailboxData51to81HR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ext_mailbox_data_51to81_h(
        &mut self,
    ) -> ExtMailboxData51to81HW<'_, ExtBtMailboxOutHighSpec> {
        ExtMailboxData51to81HW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_bt_mailbox_out_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_bt_mailbox_out_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtBtMailboxOutHighSpec;
impl crate::RegisterSpec for ExtBtMailboxOutHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_bt_mailbox_out_high::R`](R) reader structure"]
impl crate::Readable for ExtBtMailboxOutHighSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_bt_mailbox_out_high::W`](W) writer structure"]
impl crate::Writable for ExtBtMailboxOutHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_BT_MAILBOX_OUT_HIGH to value 0"]
impl crate::Resettable for ExtBtMailboxOutHighSpec {}
