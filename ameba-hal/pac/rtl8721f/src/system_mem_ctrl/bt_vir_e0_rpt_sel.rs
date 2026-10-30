#[doc = "Register `BT_VIR_E0_RPT_SEL` reader"]
pub type R = crate::R<BtVirE0RptSelSpec>;
#[doc = "Register `BT_VIR_E0_RPT_SEL` writer"]
pub type W = crate::W<BtVirE0RptSelSpec>;
#[doc = "Field `bt_vir_e0_rpt_sel` reader - Select which rom's misr_dataout or bist_status of each memory can be read from bt_vir_e0_bist_rpt"]
pub type BtVirE0RptSelR = crate::FieldReader;
#[doc = "Field `bt_vir_e0_rpt_sel` writer - Select which rom's misr_dataout or bist_status of each memory can be read from bt_vir_e0_bist_rpt"]
pub type BtVirE0RptSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select which rom's misr_dataout or bist_status of each memory can be read from bt_vir_e0_bist_rpt"]
    #[inline(always)]
    pub fn bt_vir_e0_rpt_sel(&self) -> BtVirE0RptSelR {
        BtVirE0RptSelR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select which rom's misr_dataout or bist_status of each memory can be read from bt_vir_e0_bist_rpt"]
    #[inline(always)]
    pub fn bt_vir_e0_rpt_sel(&mut self) -> BtVirE0RptSelW<'_, BtVirE0RptSelSpec> {
        BtVirE0RptSelW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_vir_e0_rpt_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_vir_e0_rpt_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtVirE0RptSelSpec;
impl crate::RegisterSpec for BtVirE0RptSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_vir_e0_rpt_sel::R`](R) reader structure"]
impl crate::Readable for BtVirE0RptSelSpec {}
#[doc = "`write(|w| ..)` method takes [`bt_vir_e0_rpt_sel::W`](W) writer structure"]
impl crate::Writable for BtVirE0RptSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BT_VIR_E0_RPT_SEL to value 0"]
impl crate::Resettable for BtVirE0RptSelSpec {}
