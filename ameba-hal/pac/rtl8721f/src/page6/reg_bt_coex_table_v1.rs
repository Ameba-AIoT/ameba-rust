#[doc = "Register `REG_BT_COEX_TABLE_V1` reader"]
pub type R = crate::R<RegBtCoexTableV1Spec>;
#[doc = "Register `REG_BT_COEX_TABLE_V1` writer"]
pub type W = crate::W<RegBtCoexTableV1Spec>;
#[doc = "Field `BIT_COEX_TABLE_1` reader - coexistence table, used at dual antenna & out of band"]
pub type BitCoexTable1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_COEX_TABLE_1` writer - coexistence table, used at dual antenna & out of band"]
pub type BitCoexTable1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - coexistence table, used at dual antenna & out of band"]
    #[inline(always)]
    pub fn bit_coex_table_1(&self) -> BitCoexTable1R {
        BitCoexTable1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - coexistence table, used at dual antenna & out of band"]
    #[inline(always)]
    pub fn bit_coex_table_1(&mut self) -> BitCoexTable1W<'_, RegBtCoexTableV1Spec> {
        BitCoexTable1W::new(self, 0)
    }
}
#[doc = "BT Coexistence Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_coex_table_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_coex_table_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtCoexTableV1Spec;
impl crate::RegisterSpec for RegBtCoexTableV1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_coex_table_v1::R`](R) reader structure"]
impl crate::Readable for RegBtCoexTableV1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_coex_table_v1::W`](W) writer structure"]
impl crate::Writable for RegBtCoexTableV1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_COEX_TABLE_V1 to value 0"]
impl crate::Resettable for RegBtCoexTableV1Spec {}
