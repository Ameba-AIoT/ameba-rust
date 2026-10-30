#[doc = "Register `REG_BT_Isolation_Table_Register_Register_1` reader"]
pub type R = crate::R<RegBtIsolationTableRegisterRegister1Spec>;
#[doc = "Register `REG_BT_Isolation_Table_Register_Register_1` writer"]
pub type W = crate::W<RegBtIsolationTableRegisterRegister1Spec>;
#[doc = "Field `BIT_ISOLATION_CHK_1` reader - 0: leave PTA 1: enter PTA"]
pub type BitIsolationChk1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_ISOLATION_CHK_1` writer - 0: leave PTA 1: enter PTA"]
pub type BitIsolationChk1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 0: leave PTA 1: enter PTA"]
    #[inline(always)]
    pub fn bit_isolation_chk_1(&self) -> BitIsolationChk1R {
        BitIsolationChk1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0: leave PTA 1: enter PTA"]
    #[inline(always)]
    pub fn bit_isolation_chk_1(
        &mut self,
    ) -> BitIsolationChk1W<'_, RegBtIsolationTableRegisterRegister1Spec> {
        BitIsolationChk1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_isolation_table_register_register_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_isolation_table_register_register_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtIsolationTableRegisterRegister1Spec;
impl crate::RegisterSpec for RegBtIsolationTableRegisterRegister1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_isolation_table_register_register_1::R`](R) reader structure"]
impl crate::Readable for RegBtIsolationTableRegisterRegister1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_isolation_table_register_register_1::W`](W) writer structure"]
impl crate::Writable for RegBtIsolationTableRegisterRegister1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_Isolation_Table_Register_Register_1 to value 0"]
impl crate::Resettable for RegBtIsolationTableRegisterRegister1Spec {}
