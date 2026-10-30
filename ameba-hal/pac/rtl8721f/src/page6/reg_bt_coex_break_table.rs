#[doc = "Register `REG_BT_COEX_BREAK_TABLE` reader"]
pub type R = crate::R<RegBtCoexBreakTableSpec>;
#[doc = "Register `REG_BT_COEX_BREAK_TABLE` writer"]
pub type W = crate::W<RegBtCoexBreakTableSpec>;
#[doc = "Field `BIT_BREAK_TABLE_1` reader - Table used to break WLAN activity, when new BT request is coming and WLAN is on transmitting or receiving"]
pub type BitBreakTable1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BREAK_TABLE_1` writer - Table used to break WLAN activity, when new BT request is coming and WLAN is on transmitting or receiving"]
pub type BitBreakTable1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_BREAK_TABLE_2` reader - Table used to break BT activity, when new WLAN request is coming and BT is on transmitting or receiving"]
pub type BitBreakTable2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BREAK_TABLE_2` writer - Table used to break BT activity, when new WLAN request is coming and BT is on transmitting or receiving"]
pub type BitBreakTable2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Table used to break WLAN activity, when new BT request is coming and WLAN is on transmitting or receiving"]
    #[inline(always)]
    pub fn bit_break_table_1(&self) -> BitBreakTable1R {
        BitBreakTable1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Table used to break BT activity, when new WLAN request is coming and BT is on transmitting or receiving"]
    #[inline(always)]
    pub fn bit_break_table_2(&self) -> BitBreakTable2R {
        BitBreakTable2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Table used to break WLAN activity, when new BT request is coming and WLAN is on transmitting or receiving"]
    #[inline(always)]
    pub fn bit_break_table_1(&mut self) -> BitBreakTable1W<'_, RegBtCoexBreakTableSpec> {
        BitBreakTable1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Table used to break BT activity, when new WLAN request is coming and BT is on transmitting or receiving"]
    #[inline(always)]
    pub fn bit_break_table_2(&mut self) -> BitBreakTable2W<'_, RegBtCoexBreakTableSpec> {
        BitBreakTable2W::new(self, 16)
    }
}
#[doc = "BT Coexistence Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_coex_break_table::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_coex_break_table::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtCoexBreakTableSpec;
impl crate::RegisterSpec for RegBtCoexBreakTableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_coex_break_table::R`](R) reader structure"]
impl crate::Readable for RegBtCoexBreakTableSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_coex_break_table::W`](W) writer structure"]
impl crate::Writable for RegBtCoexBreakTableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_COEX_BREAK_TABLE to value 0"]
impl crate::Resettable for RegBtCoexBreakTableSpec {}
