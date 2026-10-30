#[doc = "Register `SCAN_CTRL0` reader"]
pub type R = crate::R<ScanCtrl0Spec>;
#[doc = "Register `SCAN_CTRL0` writer"]
pub type W = crate::W<ScanCtrl0Spec>;
#[doc = "Field `SCAN_MODE_SETPS` reader - scan mode enable . It is a level signal , write 1 will trigger into scan mode"]
pub type ScanModeSetpsR = crate::BitReader;
#[doc = "Field `SCAN_MODE_SETPS` writer - scan mode enable . It is a level signal , write 1 will trigger into scan mode"]
pub type ScanModeSetpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_PAR_LATPS` reader - Used as scan latch enable bit . If want to change scan par from func setting , set this bit to 1 first , then write func register will also effect scan shadow register ."]
pub type ScanParLatpsR = crate::BitReader;
#[doc = "Field `SCAN_PAR_LATPS` writer - Used as scan latch enable bit . If want to change scan par from func setting , set this bit to 1 first , then write func register will also effect scan shadow register ."]
pub type ScanParLatpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_COMPRESS` reader - scan compress"]
pub type ScanCompressR = crate::BitReader;
#[doc = "Field `SCAN_COMPRESS` writer - scan compress"]
pub type ScanCompressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_PLL_BYPASS` reader - scan pll bypass"]
pub type ScanPllBypassR = crate::BitReader;
#[doc = "Field `SCAN_PLL_BYPASS` writer - scan pll bypass"]
pub type ScanPllBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_SPC_DISABLE` reader - scan spc disable"]
pub type ScanSpcDisableR = crate::BitReader;
#[doc = "Field `SCAN_SPC_DISABLE` writer - scan spc disable"]
pub type ScanSpcDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CP` reader - scan cp mode"]
pub type ScanCpR = crate::BitReader;
#[doc = "Field `SCAN_CP` writer - scan cp mode"]
pub type ScanCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_FT0` reader - scan ft mode0"]
pub type ScanFt0R = crate::BitReader;
#[doc = "Field `SCAN_FT0` writer - scan ft mode0"]
pub type ScanFt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_FT1` reader - scan ft mode1"]
pub type ScanFt1R = crate::BitReader;
#[doc = "Field `SCAN_FT1` writer - scan ft mode1"]
pub type ScanFt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_RET_EN` reader - scan retention test enable"]
pub type ScanRetEnR = crate::BitReader;
#[doc = "Field `SCAN_RET_EN` writer - scan retention test enable"]
pub type ScanRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_PLL_OUT_SEL` reader - scan pll out sel"]
pub type ScanPllOutSelR = crate::FieldReader;
#[doc = "Field `SCAN_PLL_OUT_SEL` writer - scan pll out sel"]
pub type ScanPllOutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - scan mode enable . It is a level signal , write 1 will trigger into scan mode"]
    #[inline(always)]
    pub fn scan_mode_setps(&self) -> ScanModeSetpsR {
        ScanModeSetpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Used as scan latch enable bit . If want to change scan par from func setting , set this bit to 1 first , then write func register will also effect scan shadow register ."]
    #[inline(always)]
    pub fn scan_par_latps(&self) -> ScanParLatpsR {
        ScanParLatpsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - scan compress"]
    #[inline(always)]
    pub fn scan_compress(&self) -> ScanCompressR {
        ScanCompressR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - scan pll bypass"]
    #[inline(always)]
    pub fn scan_pll_bypass(&self) -> ScanPllBypassR {
        ScanPllBypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - scan spc disable"]
    #[inline(always)]
    pub fn scan_spc_disable(&self) -> ScanSpcDisableR {
        ScanSpcDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - scan cp mode"]
    #[inline(always)]
    pub fn scan_cp(&self) -> ScanCpR {
        ScanCpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - scan ft mode0"]
    #[inline(always)]
    pub fn scan_ft0(&self) -> ScanFt0R {
        ScanFt0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - scan ft mode1"]
    #[inline(always)]
    pub fn scan_ft1(&self) -> ScanFt1R {
        ScanFt1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - scan retention test enable"]
    #[inline(always)]
    pub fn scan_ret_en(&self) -> ScanRetEnR {
        ScanRetEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - scan pll out sel"]
    #[inline(always)]
    pub fn scan_pll_out_sel(&self) -> ScanPllOutSelR {
        ScanPllOutSelR::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - scan mode enable . It is a level signal , write 1 will trigger into scan mode"]
    #[inline(always)]
    pub fn scan_mode_setps(&mut self) -> ScanModeSetpsW<'_, ScanCtrl0Spec> {
        ScanModeSetpsW::new(self, 0)
    }
    #[doc = "Bit 1 - Used as scan latch enable bit . If want to change scan par from func setting , set this bit to 1 first , then write func register will also effect scan shadow register ."]
    #[inline(always)]
    pub fn scan_par_latps(&mut self) -> ScanParLatpsW<'_, ScanCtrl0Spec> {
        ScanParLatpsW::new(self, 1)
    }
    #[doc = "Bit 2 - scan compress"]
    #[inline(always)]
    pub fn scan_compress(&mut self) -> ScanCompressW<'_, ScanCtrl0Spec> {
        ScanCompressW::new(self, 2)
    }
    #[doc = "Bit 3 - scan pll bypass"]
    #[inline(always)]
    pub fn scan_pll_bypass(&mut self) -> ScanPllBypassW<'_, ScanCtrl0Spec> {
        ScanPllBypassW::new(self, 3)
    }
    #[doc = "Bit 4 - scan spc disable"]
    #[inline(always)]
    pub fn scan_spc_disable(&mut self) -> ScanSpcDisableW<'_, ScanCtrl0Spec> {
        ScanSpcDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - scan cp mode"]
    #[inline(always)]
    pub fn scan_cp(&mut self) -> ScanCpW<'_, ScanCtrl0Spec> {
        ScanCpW::new(self, 5)
    }
    #[doc = "Bit 6 - scan ft mode0"]
    #[inline(always)]
    pub fn scan_ft0(&mut self) -> ScanFt0W<'_, ScanCtrl0Spec> {
        ScanFt0W::new(self, 6)
    }
    #[doc = "Bit 7 - scan ft mode1"]
    #[inline(always)]
    pub fn scan_ft1(&mut self) -> ScanFt1W<'_, ScanCtrl0Spec> {
        ScanFt1W::new(self, 7)
    }
    #[doc = "Bit 8 - scan retention test enable"]
    #[inline(always)]
    pub fn scan_ret_en(&mut self) -> ScanRetEnW<'_, ScanCtrl0Spec> {
        ScanRetEnW::new(self, 8)
    }
    #[doc = "Bits 9:14 - scan pll out sel"]
    #[inline(always)]
    pub fn scan_pll_out_sel(&mut self) -> ScanPllOutSelW<'_, ScanCtrl0Spec> {
        ScanPllOutSelW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanCtrl0Spec;
impl crate::RegisterSpec for ScanCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scan_ctrl0::R`](R) reader structure"]
impl crate::Readable for ScanCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`scan_ctrl0::W`](W) writer structure"]
impl crate::Writable for ScanCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAN_CTRL0 to value 0"]
impl crate::Resettable for ScanCtrl0Spec {}
