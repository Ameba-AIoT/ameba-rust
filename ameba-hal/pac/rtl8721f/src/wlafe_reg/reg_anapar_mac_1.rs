#[doc = "Register `REG_ANAPAR_MAC_1` reader"]
pub type R = crate::R<RegAnaparMac1Spec>;
#[doc = "Register `REG_ANAPAR_MAC_1` writer"]
pub type W = crate::W<RegAnaparMac1Spec>;
#[doc = "Field `reg_fref_edge` reader - PLL referemce clk edge select 0: positive 1: negtive"]
pub type RegFrefEdgeR = crate::BitReader;
#[doc = "Field `reg_fref_edge` writer - PLL referemce clk edge select 0: positive 1: negtive"]
pub type RegFrefEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK320M_EN` reader - 1: Enable 0: Output \"1\""]
pub type Ck320mEnR = crate::BitReader;
#[doc = "Field `CK320M_EN` writer - 1: Enable 0: Output \"1\""]
pub type Ck320mEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_200M_TEST` reader - "]
pub type Ck200mTestR = crate::BitReader;
#[doc = "Field `CK_200M_TEST` writer - "]
pub type Ck200mTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK320M_AFE_EN` reader - 1: Enable 0: Output \"1\""]
pub type Ck320mAfeEnR = crate::BitReader;
#[doc = "Field `CK320M_AFE_EN` writer - 1: Enable 0: Output \"1\""]
pub type Ck320mAfeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BB_RFE_SEL_AFE_SDM` reader - pll referemce clk select AFE\\[0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15\\] = \\[40M 25M 54M 19.2M 20M 26M 38.4M 17.664M 16M 32M 60M 52M 48M 27M 24M 80M\\] (efuse)"]
pub type BbRfeSelAfeSdmR = crate::FieldReader;
#[doc = "Field `BB_RFE_SEL_AFE_SDM` writer - pll referemce clk select AFE\\[0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15\\] = \\[40M 25M 54M 19.2M 20M 26M 38.4M 17.664M 16M 32M 60M 52M 48M 27M 24M 80M\\] (efuse)"]
pub type BbRfeSelAfeSdmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RFE_SEL_SDM` reader - pll referemce clk select 1:table 0:regesters"]
pub type RfeSelSdmR = crate::BitReader;
#[doc = "Field `RFE_SEL_SDM` writer - pll referemce clk select 1:table 0:regesters"]
pub type RfeSelSdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BB_DBG_SEL_AFE_SDM` reader - SDM debug selection"]
pub type BbDbgSelAfeSdmR = crate::FieldReader;
#[doc = "Field `BB_DBG_SEL_AFE_SDM` writer - SDM debug selection"]
pub type BbDbgSelAfeSdmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVN` reader - SDM divider sel"]
pub type DivnR = crate::FieldReader;
#[doc = "Field `DIVN` writer - SDM divider sel"]
pub type DivnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F0F` reader - SDM divider sel"]
pub type F0fR = crate::FieldReader<u16>;
#[doc = "Field `F0F` writer - SDM divider sel"]
pub type F0fW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - PLL referemce clk edge select 0: positive 1: negtive"]
    #[inline(always)]
    pub fn reg_fref_edge(&self) -> RegFrefEdgeR {
        RegFrefEdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable 0: Output \"1\""]
    #[inline(always)]
    pub fn ck320m_en(&self) -> Ck320mEnR {
        Ck320mEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ck_200m_test(&self) -> Ck200mTestR {
        Ck200mTestR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable 0: Output \"1\""]
    #[inline(always)]
    pub fn ck320m_afe_en(&self) -> Ck320mAfeEnR {
        Ck320mAfeEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - pll referemce clk select AFE\\[0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15\\] = \\[40M 25M 54M 19.2M 20M 26M 38.4M 17.664M 16M 32M 60M 52M 48M 27M 24M 80M\\] (efuse)"]
    #[inline(always)]
    pub fn bb_rfe_sel_afe_sdm(&self) -> BbRfeSelAfeSdmR {
        BbRfeSelAfeSdmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - pll referemce clk select 1:table 0:regesters"]
    #[inline(always)]
    pub fn rfe_sel_sdm(&self) -> RfeSelSdmR {
        RfeSelSdmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - SDM debug selection"]
    #[inline(always)]
    pub fn bb_dbg_sel_afe_sdm(&self) -> BbDbgSelAfeSdmR {
        BbDbgSelAfeSdmR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:18 - SDM divider sel"]
    #[inline(always)]
    pub fn divn(&self) -> DivnR {
        DivnR::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:31 - SDM divider sel"]
    #[inline(always)]
    pub fn f0f(&self) -> F0fR {
        F0fR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PLL referemce clk edge select 0: positive 1: negtive"]
    #[inline(always)]
    pub fn reg_fref_edge(&mut self) -> RegFrefEdgeW<'_, RegAnaparMac1Spec> {
        RegFrefEdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable 0: Output \"1\""]
    #[inline(always)]
    pub fn ck320m_en(&mut self) -> Ck320mEnW<'_, RegAnaparMac1Spec> {
        Ck320mEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ck_200m_test(&mut self) -> Ck200mTestW<'_, RegAnaparMac1Spec> {
        Ck200mTestW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Enable 0: Output \"1\""]
    #[inline(always)]
    pub fn ck320m_afe_en(&mut self) -> Ck320mAfeEnW<'_, RegAnaparMac1Spec> {
        Ck320mAfeEnW::new(self, 3)
    }
    #[doc = "Bits 4:7 - pll referemce clk select AFE\\[0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15\\] = \\[40M 25M 54M 19.2M 20M 26M 38.4M 17.664M 16M 32M 60M 52M 48M 27M 24M 80M\\] (efuse)"]
    #[inline(always)]
    pub fn bb_rfe_sel_afe_sdm(&mut self) -> BbRfeSelAfeSdmW<'_, RegAnaparMac1Spec> {
        BbRfeSelAfeSdmW::new(self, 4)
    }
    #[doc = "Bit 8 - pll referemce clk select 1:table 0:regesters"]
    #[inline(always)]
    pub fn rfe_sel_sdm(&mut self) -> RfeSelSdmW<'_, RegAnaparMac1Spec> {
        RfeSelSdmW::new(self, 8)
    }
    #[doc = "Bits 9:12 - SDM debug selection"]
    #[inline(always)]
    pub fn bb_dbg_sel_afe_sdm(&mut self) -> BbDbgSelAfeSdmW<'_, RegAnaparMac1Spec> {
        BbDbgSelAfeSdmW::new(self, 9)
    }
    #[doc = "Bits 13:18 - SDM divider sel"]
    #[inline(always)]
    pub fn divn(&mut self) -> DivnW<'_, RegAnaparMac1Spec> {
        DivnW::new(self, 13)
    }
    #[doc = "Bits 19:31 - SDM divider sel"]
    #[inline(always)]
    pub fn f0f(&mut self) -> F0fW<'_, RegAnaparMac1Spec> {
        F0fW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_mac_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_mac_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparMac1Spec;
impl crate::RegisterSpec for RegAnaparMac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_mac_1::R`](R) reader structure"]
impl crate::Readable for RegAnaparMac1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_mac_1::W`](W) writer structure"]
impl crate::Writable for RegAnaparMac1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_MAC_1 to value 0x0001_0100"]
impl crate::Resettable for RegAnaparMac1Spec {
    const RESET_VALUE: u32 = 0x0001_0100;
}
