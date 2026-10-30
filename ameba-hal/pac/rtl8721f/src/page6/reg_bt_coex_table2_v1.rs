#[doc = "Register `REG_BT_COEX_TABLE2_V1` reader"]
pub type R = crate::R<RegBtCoexTable2V1Spec>;
#[doc = "Register `REG_BT_COEX_TABLE2_V1` writer"]
pub type W = crate::W<RegBtCoexTable2V1Spec>;
#[doc = "Field `BIT_COEX_TABLE_2` reader - coexistence table, used at dual antenna & in band or single antenna"]
pub type BitCoexTable2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_COEX_TABLE_2` writer - coexistence table, used at dual antenna & in band or single antenna"]
pub type BitCoexTable2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - coexistence table, used at dual antenna & in band or single antenna"]
    #[inline(always)]
    pub fn bit_coex_table_2(&self) -> BitCoexTable2R {
        BitCoexTable2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - coexistence table, used at dual antenna & in band or single antenna"]
    #[inline(always)]
    pub fn bit_coex_table_2(&mut self) -> BitCoexTable2W<'_, RegBtCoexTable2V1Spec> {
        BitCoexTable2W::new(self, 0)
    }
}
#[doc = "BTCoexistence Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_coex_table2_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_coex_table2_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtCoexTable2V1Spec;
impl crate::RegisterSpec for RegBtCoexTable2V1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_coex_table2_v1::R`](R) reader structure"]
impl crate::Readable for RegBtCoexTable2V1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_coex_table2_v1::W`](W) writer structure"]
impl crate::Writable for RegBtCoexTable2V1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_COEX_TABLE2_V1 to value 0"]
impl crate::Resettable for RegBtCoexTable2V1Spec {}
