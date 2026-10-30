#[doc = "Register `REG_ZB_BREAK_TABLE` reader"]
pub type R = crate::R<RegZbBreakTableSpec>;
#[doc = "Register `REG_ZB_BREAK_TABLE` writer"]
pub type W = crate::W<RegZbBreakTableSpec>;
#[doc = "Field `BIT_ZB_BREAK_TABLE_1` reader - Table used to break WLAN activity, when new ZB request is coming and WLAN is on transmitting or receiving"]
pub type BitZbBreakTable1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_ZB_BREAK_TABLE_1` writer - Table used to break WLAN activity, when new ZB request is coming and WLAN is on transmitting or receiving"]
pub type BitZbBreakTable1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_ZB_BREAK_TABLE_2` reader - Table used to break ZB activity, when new WLAN request is coming and ZB is on transmitting or receiving"]
pub type BitZbBreakTable2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_ZB_BREAK_TABLE_2` writer - Table used to break ZB activity, when new WLAN request is coming and ZB is on transmitting or receiving"]
pub type BitZbBreakTable2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Table used to break WLAN activity, when new ZB request is coming and WLAN is on transmitting or receiving"]
    #[inline(always)]
    pub fn bit_zb_break_table_1(&self) -> BitZbBreakTable1R {
        BitZbBreakTable1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Table used to break ZB activity, when new WLAN request is coming and ZB is on transmitting or receiving"]
    #[inline(always)]
    pub fn bit_zb_break_table_2(&self) -> BitZbBreakTable2R {
        BitZbBreakTable2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Table used to break WLAN activity, when new ZB request is coming and WLAN is on transmitting or receiving"]
    #[inline(always)]
    pub fn bit_zb_break_table_1(&mut self) -> BitZbBreakTable1W<'_, RegZbBreakTableSpec> {
        BitZbBreakTable1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Table used to break ZB activity, when new WLAN request is coming and ZB is on transmitting or receiving"]
    #[inline(always)]
    pub fn bit_zb_break_table_2(&mut self) -> BitZbBreakTable2W<'_, RegZbBreakTableSpec> {
        BitZbBreakTable2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_zb_break_table::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_zb_break_table::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegZbBreakTableSpec;
impl crate::RegisterSpec for RegZbBreakTableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_zb_break_table::R`](R) reader structure"]
impl crate::Readable for RegZbBreakTableSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_zb_break_table::W`](W) writer structure"]
impl crate::Writable for RegZbBreakTableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ZB_BREAK_TABLE to value 0"]
impl crate::Resettable for RegZbBreakTableSpec {}
