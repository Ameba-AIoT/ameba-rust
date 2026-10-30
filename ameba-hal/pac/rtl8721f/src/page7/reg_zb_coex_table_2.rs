#[doc = "Register `REG_ZB_COEX_TABLE_2` reader"]
pub type R = crate::R<RegZbCoexTable2Spec>;
#[doc = "Register `REG_ZB_COEX_TABLE_2` writer"]
pub type W = crate::W<RegZbCoexTable2Spec>;
#[doc = "Field `BIT_ZB_COEX_TABLE_2` reader - coexistence table between WLAN and Zigbee, selected by BIT_COEX_TABLE_SEL = 1"]
pub type BitZbCoexTable2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_ZB_COEX_TABLE_2` writer - coexistence table between WLAN and Zigbee, selected by BIT_COEX_TABLE_SEL = 1"]
pub type BitZbCoexTable2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - coexistence table between WLAN and Zigbee, selected by BIT_COEX_TABLE_SEL = 1"]
    #[inline(always)]
    pub fn bit_zb_coex_table_2(&self) -> BitZbCoexTable2R {
        BitZbCoexTable2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - coexistence table between WLAN and Zigbee, selected by BIT_COEX_TABLE_SEL = 1"]
    #[inline(always)]
    pub fn bit_zb_coex_table_2(&mut self) -> BitZbCoexTable2W<'_, RegZbCoexTable2Spec> {
        BitZbCoexTable2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_zb_coex_table_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_zb_coex_table_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegZbCoexTable2Spec;
impl crate::RegisterSpec for RegZbCoexTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_zb_coex_table_2::R`](R) reader structure"]
impl crate::Readable for RegZbCoexTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_zb_coex_table_2::W`](W) writer structure"]
impl crate::Writable for RegZbCoexTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ZB_COEX_TABLE_2 to value 0"]
impl crate::Resettable for RegZbCoexTable2Spec {}
