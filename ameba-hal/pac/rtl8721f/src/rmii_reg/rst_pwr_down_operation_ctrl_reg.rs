#[doc = "Register `Rst_Pwr_Down_Operation_Ctrl_REG` reader"]
pub type R = crate::R<RstPwrDownOperationCtrlRegSpec>;
#[doc = "Register `Rst_Pwr_Down_Operation_Ctrl_REG` writer"]
pub type W = crate::W<RstPwrDownOperationCtrlRegSpec>;
#[doc = "Field `AUTOLSU` reader - Auto Link Status Update"]
pub type AutolsuR = crate::BitReader;
#[doc = "Field `AUTOLSU` writer - Auto Link Status Update"]
pub type AutolsuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOISOU` reader - Auto Isolate Update"]
pub type AutoisouR = crate::BitReader;
#[doc = "Field `AUTOISOU` writer - Auto Isolate Update"]
pub type AutoisouW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTXCLKDIS` reader - 1: Auto Disable Tx Clock when AUTOISOU=1"]
pub type AutotxclkdisR = crate::BitReader;
#[doc = "Field `AUTOTXCLKDIS` writer - 1: Auto Disable Tx Clock when AUTOISOU=1"]
pub type AutotxclkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORXCLKDIS` reader - 1: Auto Disable Rx Clock when AUTOISOU=1"]
pub type AutorxclkdisR = crate::BitReader;
#[doc = "Field `AUTORXCLKDIS` writer - 1: Auto Disable Rx Clock when AUTOISOU=1"]
pub type AutorxclkdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Auto Link Status Update"]
    #[inline(always)]
    pub fn autolsu(&self) -> AutolsuR {
        AutolsuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto Isolate Update"]
    #[inline(always)]
    pub fn autoisou(&self) -> AutoisouR {
        AutoisouR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Auto Disable Tx Clock when AUTOISOU=1"]
    #[inline(always)]
    pub fn autotxclkdis(&self) -> AutotxclkdisR {
        AutotxclkdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Auto Disable Rx Clock when AUTOISOU=1"]
    #[inline(always)]
    pub fn autorxclkdis(&self) -> AutorxclkdisR {
        AutorxclkdisR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Link Status Update"]
    #[inline(always)]
    pub fn autolsu(&mut self) -> AutolsuW<'_, RstPwrDownOperationCtrlRegSpec> {
        AutolsuW::new(self, 0)
    }
    #[doc = "Bit 1 - Auto Isolate Update"]
    #[inline(always)]
    pub fn autoisou(&mut self) -> AutoisouW<'_, RstPwrDownOperationCtrlRegSpec> {
        AutoisouW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Auto Disable Tx Clock when AUTOISOU=1"]
    #[inline(always)]
    pub fn autotxclkdis(&mut self) -> AutotxclkdisW<'_, RstPwrDownOperationCtrlRegSpec> {
        AutotxclkdisW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Auto Disable Rx Clock when AUTOISOU=1"]
    #[inline(always)]
    pub fn autorxclkdis(&mut self) -> AutorxclkdisW<'_, RstPwrDownOperationCtrlRegSpec> {
        AutorxclkdisW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_pwr_down_operation_ctrl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_pwr_down_operation_ctrl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstPwrDownOperationCtrlRegSpec;
impl crate::RegisterSpec for RstPwrDownOperationCtrlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_pwr_down_operation_ctrl_reg::R`](R) reader structure"]
impl crate::Readable for RstPwrDownOperationCtrlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_pwr_down_operation_ctrl_reg::W`](W) writer structure"]
impl crate::Writable for RstPwrDownOperationCtrlRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Rst_Pwr_Down_Operation_Ctrl_REG to value 0"]
impl crate::Resettable for RstPwrDownOperationCtrlRegSpec {}
