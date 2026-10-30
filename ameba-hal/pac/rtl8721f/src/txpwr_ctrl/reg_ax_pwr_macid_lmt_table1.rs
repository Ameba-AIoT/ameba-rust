#[doc = "Register `REG_AX_PWR_MACID_LMT_TABLE1` reader"]
pub type R = crate::R<RegAxPwrMacidLmtTable1Spec>;
#[doc = "Register `REG_AX_PWR_MACID_LMT_TABLE1` writer"]
pub type W = crate::W<RegAxPwrMacidLmtTable1Spec>;
#[doc = "Field `r_macid_1_txpwr0` reader - s(8.1)"]
pub type RMacid1Txpwr0R = crate::FieldReader;
#[doc = "Field `r_macid_1_txpwr0` writer - s(8.1)"]
pub type RMacid1Txpwr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_macid_1_txpwr1` reader - s(8.1)"]
pub type RMacid1Txpwr1R = crate::FieldReader;
#[doc = "Field `r_macid_1_txpwr1` writer - s(8.1)"]
pub type RMacid1Txpwr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_macid_1_CCA_PWR_TH` reader - s(8.1)"]
pub type RMacid1CcaPwrThR = crate::FieldReader;
#[doc = "Field `r_macid_1_CCA_PWR_TH` writer - s(8.1)"]
pub type RMacid1CcaPwrThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_macid_1_txpwr0_en` reader - u(1.0)"]
pub type RMacid1Txpwr0EnR = crate::BitReader;
#[doc = "Field `r_macid_1_txpwr0_en` writer - u(1.0)"]
pub type RMacid1Txpwr0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_macid_1_txpwr1_en` reader - u(1.0)"]
pub type RMacid1Txpwr1EnR = crate::BitReader;
#[doc = "Field `r_macid_1_txpwr1_en` writer - u(1.0)"]
pub type RMacid1Txpwr1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_macid_1_CCA_PWR_TH_en` reader - u(1.0)"]
pub type RMacid1CcaPwrThEnR = crate::BitReader;
#[doc = "Field `r_macid_1_CCA_PWR_TH_en` writer - u(1.0)"]
pub type RMacid1CcaPwrThEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - s(8.1)"]
    #[inline(always)]
    pub fn r_macid_1_txpwr0(&self) -> RMacid1Txpwr0R {
        RMacid1Txpwr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - s(8.1)"]
    #[inline(always)]
    pub fn r_macid_1_txpwr1(&self) -> RMacid1Txpwr1R {
        RMacid1Txpwr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - s(8.1)"]
    #[inline(always)]
    pub fn r_macid_1_cca_pwr_th(&self) -> RMacid1CcaPwrThR {
        RMacid1CcaPwrThR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - u(1.0)"]
    #[inline(always)]
    pub fn r_macid_1_txpwr0_en(&self) -> RMacid1Txpwr0EnR {
        RMacid1Txpwr0EnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - u(1.0)"]
    #[inline(always)]
    pub fn r_macid_1_txpwr1_en(&self) -> RMacid1Txpwr1EnR {
        RMacid1Txpwr1EnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - u(1.0)"]
    #[inline(always)]
    pub fn r_macid_1_cca_pwr_th_en(&self) -> RMacid1CcaPwrThEnR {
        RMacid1CcaPwrThEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - s(8.1)"]
    #[inline(always)]
    pub fn r_macid_1_txpwr0(&mut self) -> RMacid1Txpwr0W<'_, RegAxPwrMacidLmtTable1Spec> {
        RMacid1Txpwr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - s(8.1)"]
    #[inline(always)]
    pub fn r_macid_1_txpwr1(&mut self) -> RMacid1Txpwr1W<'_, RegAxPwrMacidLmtTable1Spec> {
        RMacid1Txpwr1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - s(8.1)"]
    #[inline(always)]
    pub fn r_macid_1_cca_pwr_th(&mut self) -> RMacid1CcaPwrThW<'_, RegAxPwrMacidLmtTable1Spec> {
        RMacid1CcaPwrThW::new(self, 16)
    }
    #[doc = "Bit 24 - u(1.0)"]
    #[inline(always)]
    pub fn r_macid_1_txpwr0_en(&mut self) -> RMacid1Txpwr0EnW<'_, RegAxPwrMacidLmtTable1Spec> {
        RMacid1Txpwr0EnW::new(self, 24)
    }
    #[doc = "Bit 25 - u(1.0)"]
    #[inline(always)]
    pub fn r_macid_1_txpwr1_en(&mut self) -> RMacid1Txpwr1EnW<'_, RegAxPwrMacidLmtTable1Spec> {
        RMacid1Txpwr1EnW::new(self, 25)
    }
    #[doc = "Bit 26 - u(1.0)"]
    #[inline(always)]
    pub fn r_macid_1_cca_pwr_th_en(
        &mut self,
    ) -> RMacid1CcaPwrThEnW<'_, RegAxPwrMacidLmtTable1Spec> {
        RMacid1CcaPwrThEnW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_macid_lmt_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_macid_lmt_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrMacidLmtTable1Spec;
impl crate::RegisterSpec for RegAxPwrMacidLmtTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_macid_lmt_table1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrMacidLmtTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_macid_lmt_table1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrMacidLmtTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_MACID_LMT_TABLE1 to value 0"]
impl crate::Resettable for RegAxPwrMacidLmtTable1Spec {}
