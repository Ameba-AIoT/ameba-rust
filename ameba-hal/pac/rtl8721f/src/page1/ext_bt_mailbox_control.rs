#[doc = "Register `EXT_BT_Mailbox_control` reader"]
pub type R = crate::R<ExtBtMailboxControlSpec>;
#[doc = "Register `EXT_BT_Mailbox_control` writer"]
pub type W = crate::W<ExtBtMailboxControlSpec>;
#[doc = "Field `ext_mailbox_out_vld` reader - Mailbox ACK Wifi2BT"]
pub type ExtMailboxOutVldR = crate::BitReader;
#[doc = "Field `ext_mailbox_outempty_inten` reader - Mailbox OUT empty interrupt enable"]
pub type ExtMailboxOutemptyIntenR = crate::BitReader;
#[doc = "Field `ext_mailbox_outempty_inten` writer - Mailbox OUT empty interrupt enable"]
pub type ExtMailboxOutemptyIntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ext_mailbox_in_rdy` reader - Mailbox ready BT2Wifi"]
pub type ExtMailboxInRdyR = crate::BitReader;
#[doc = "Field `ext_mailbox_in_rdy_inten` reader - Mailbox IN ready interrupt enable"]
pub type ExtMailboxInRdyIntenR = crate::BitReader;
#[doc = "Field `ext_mailbox_in_rdy_inten` writer - Mailbox IN ready interrupt enable"]
pub type ExtMailboxInRdyIntenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mailbox ACK Wifi2BT"]
    #[inline(always)]
    pub fn ext_mailbox_out_vld(&self) -> ExtMailboxOutVldR {
        ExtMailboxOutVldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox OUT empty interrupt enable"]
    #[inline(always)]
    pub fn ext_mailbox_outempty_inten(&self) -> ExtMailboxOutemptyIntenR {
        ExtMailboxOutemptyIntenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Mailbox ready BT2Wifi"]
    #[inline(always)]
    pub fn ext_mailbox_in_rdy(&self) -> ExtMailboxInRdyR {
        ExtMailboxInRdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mailbox IN ready interrupt enable"]
    #[inline(always)]
    pub fn ext_mailbox_in_rdy_inten(&self) -> ExtMailboxInRdyIntenR {
        ExtMailboxInRdyIntenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mailbox OUT empty interrupt enable"]
    #[inline(always)]
    pub fn ext_mailbox_outempty_inten(
        &mut self,
    ) -> ExtMailboxOutemptyIntenW<'_, ExtBtMailboxControlSpec> {
        ExtMailboxOutemptyIntenW::new(self, 1)
    }
    #[doc = "Bit 17 - Mailbox IN ready interrupt enable"]
    #[inline(always)]
    pub fn ext_mailbox_in_rdy_inten(
        &mut self,
    ) -> ExtMailboxInRdyIntenW<'_, ExtBtMailboxControlSpec> {
        ExtMailboxInRdyIntenW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_bt_mailbox_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_bt_mailbox_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtBtMailboxControlSpec;
impl crate::RegisterSpec for ExtBtMailboxControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_bt_mailbox_control::R`](R) reader structure"]
impl crate::Readable for ExtBtMailboxControlSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_bt_mailbox_control::W`](W) writer structure"]
impl crate::Writable for ExtBtMailboxControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_BT_Mailbox_control to value 0"]
impl crate::Resettable for ExtBtMailboxControlSpec {}
