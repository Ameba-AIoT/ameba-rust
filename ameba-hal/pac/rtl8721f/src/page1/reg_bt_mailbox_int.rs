#[doc = "Register `REG_BT_MAILBOX_INT` reader"]
pub type R = crate::R<RegBtMailboxIntSpec>;
#[doc = "Register `REG_BT_MAILBOX_INT` writer"]
pub type W = crate::W<RegBtMailboxIntSpec>;
#[doc = "Field `BIT_BT_MAILBOX_INT` reader - bt2wl and wl2bt mailbox interrupt"]
pub type BitBtMailboxIntR = crate::BitReader;
#[doc = "Field `BIT_BT_MAILBOX_INT_EN` reader - bt2wl and wl2bt mailbox interrupt mask"]
pub type BitBtMailboxIntEnR = crate::BitReader;
#[doc = "Field `BIT_BT_MAILBOX_INT_EN` writer - bt2wl and wl2bt mailbox interrupt mask"]
pub type BitBtMailboxIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - bt2wl and wl2bt mailbox interrupt"]
    #[inline(always)]
    pub fn bit_bt_mailbox_int(&self) -> BitBtMailboxIntR {
        BitBtMailboxIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - bt2wl and wl2bt mailbox interrupt mask"]
    #[inline(always)]
    pub fn bit_bt_mailbox_int_en(&self) -> BitBtMailboxIntEnR {
        BitBtMailboxIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - bt2wl and wl2bt mailbox interrupt mask"]
    #[inline(always)]
    pub fn bit_bt_mailbox_int_en(&mut self) -> BitBtMailboxIntEnW<'_, RegBtMailboxIntSpec> {
        BitBtMailboxIntEnW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_mailbox_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_mailbox_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtMailboxIntSpec;
impl crate::RegisterSpec for RegBtMailboxIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_mailbox_int::R`](R) reader structure"]
impl crate::Readable for RegBtMailboxIntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_mailbox_int::W`](W) writer structure"]
impl crate::Writable for RegBtMailboxIntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_MAILBOX_INT to value 0"]
impl crate::Resettable for RegBtMailboxIntSpec {}
