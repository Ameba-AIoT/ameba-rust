#[doc = "Register `REG_ZB_COEX_TABLE_1` reader"]
pub type R = crate::R<RegZbCoexTable1Spec>;
#[doc = "Register `REG_ZB_COEX_TABLE_1` writer"]
pub type W = crate::W<RegZbCoexTable1Spec>;
#[doc = "Field `BIT_ZB_COEX_TABLE_1` reader - coexistence table between WLAN and Zigbee, selected by BIT_COEX_TABLE_SEL = 0"]
pub type BitZbCoexTable1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_ZB_COEX_TABLE_1` writer - coexistence table between WLAN and Zigbee, selected by BIT_COEX_TABLE_SEL = 0"]
pub type BitZbCoexTable1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - coexistence table between WLAN and Zigbee, selected by BIT_COEX_TABLE_SEL = 0"]
    #[inline(always)]
    pub fn bit_zb_coex_table_1(&self) -> BitZbCoexTable1R {
        BitZbCoexTable1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - coexistence table between WLAN and Zigbee, selected by BIT_COEX_TABLE_SEL = 0"]
    #[inline(always)]
    pub fn bit_zb_coex_table_1(&mut self) -> BitZbCoexTable1W<'_, RegZbCoexTable1Spec> {
        BitZbCoexTable1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_zb_coex_table_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_zb_coex_table_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegZbCoexTable1Spec;
impl crate::RegisterSpec for RegZbCoexTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_zb_coex_table_1::R`](R) reader structure"]
impl crate::Readable for RegZbCoexTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_zb_coex_table_1::W`](W) writer structure"]
impl crate::Writable for RegZbCoexTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ZB_COEX_TABLE_1 to value 0"]
impl crate::Resettable for RegZbCoexTable1Spec {}
