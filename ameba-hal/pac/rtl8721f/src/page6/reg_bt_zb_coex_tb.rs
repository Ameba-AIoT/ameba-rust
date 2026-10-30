#[doc = "Register `REG_BT_ZB_COEX_TB` reader"]
pub type R = crate::R<RegBtZbCoexTbSpec>;
#[doc = "Register `REG_BT_ZB_COEX_TB` writer"]
pub type W = crate::W<RegBtZbCoexTbSpec>;
#[doc = "Field `r_bt_zb_coex_table` reader - bt and zigbee coexist table"]
pub type RBtZbCoexTableR = crate::FieldReader<u32>;
#[doc = "Field `r_bt_zb_coex_table` writer - bt and zigbee coexist table"]
pub type RBtZbCoexTableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bt and zigbee coexist table"]
    #[inline(always)]
    pub fn r_bt_zb_coex_table(&self) -> RBtZbCoexTableR {
        RBtZbCoexTableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bt and zigbee coexist table"]
    #[inline(always)]
    pub fn r_bt_zb_coex_table(&mut self) -> RBtZbCoexTableW<'_, RegBtZbCoexTbSpec> {
        RBtZbCoexTableW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_zb_coex_tb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_zb_coex_tb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtZbCoexTbSpec;
impl crate::RegisterSpec for RegBtZbCoexTbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_zb_coex_tb::R`](R) reader structure"]
impl crate::Readable for RegBtZbCoexTbSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_zb_coex_tb::W`](W) writer structure"]
impl crate::Writable for RegBtZbCoexTbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_ZB_COEX_TB to value 0"]
impl crate::Resettable for RegBtZbCoexTbSpec {}
