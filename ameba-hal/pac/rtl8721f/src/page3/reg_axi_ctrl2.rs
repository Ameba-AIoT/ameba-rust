#[doc = "Register `REG_AXI_CTRL2` reader"]
pub type R = crate::R<RegAxiCtrl2Spec>;
#[doc = "Register `REG_AXI_CTRL2` writer"]
pub type W = crate::W<RegAxiCtrl2Spec>;
#[doc = "Field `BIT_DEBUG_REG3` reader - Debug Selection Register"]
pub type BitDebugReg3R = crate::FieldReader<u32>;
#[doc = "Field `BIT_DEBUG_REG3` writer - Debug Selection Register"]
pub type BitDebugReg3W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_DEBUG_REG0` reader - Write:Debug Selection Register Read:3'b000"]
pub type BitDebugReg0R = crate::FieldReader;
#[doc = "Field `BIT_DEBUG_REG0` writer - Write:Debug Selection Register Read:3'b000"]
pub type BitDebugReg0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_INT_WLMAC` reader - Write:hps_clkr_ahb\\[0\\] Read:Interrupt.Write clear."]
pub type BitIntWlmacR = crate::BitReader;
#[doc = "Field `BIT_INT_WLMAC` writer - Write:hps_clkr_ahb\\[0\\] Read:Interrupt.Write clear."]
pub type BitIntWlmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HPS_CLKR_AHB` reader - The Registers are used to prevent the deadlock whenever HW or FW turn off ANACLK or SYSCLK, which the HW or FW could not resume by themself. Currently they are not used in normal control flow. They might be helpful when Driver would like to handle whole Suspned & Resume flow by itself hps_clkr_ahb\\[2:1\\]"]
pub type BitHpsClkrAhbR = crate::FieldReader;
#[doc = "Field `BIT_HPS_CLKR_AHB` writer - The Registers are used to prevent the deadlock whenever HW or FW turn off ANACLK or SYSCLK, which the HW or FW could not resume by themself. Currently they are not used in normal control flow. They might be helpful when Driver would like to handle whole Suspned & Resume flow by itself hps_clkr_ahb\\[2:1\\]"]
pub type BitHpsClkrAhbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_DEBUG_REG1` reader - Write:hps_clkr_ahb\\[3\\] Read:Debug Selection Register"]
pub type BitDebugReg1R = crate::BitReader;
#[doc = "Field `BIT_DEBUG_REG1` writer - Write:hps_clkr_ahb\\[3\\] Read:Debug Selection Register"]
pub type BitDebugReg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DEBUG_REG2` reader - Debug Selection Register"]
pub type BitDebugReg2R = crate::BitReader;
#[doc = "Field `BIT_DEBUG_REG2` writer - Debug Selection Register"]
pub type BitDebugReg2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Debug Selection Register"]
    #[inline(always)]
    pub fn bit_debug_reg3(&self) -> BitDebugReg3R {
        BitDebugReg3R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:26 - Write:Debug Selection Register Read:3'b000"]
    #[inline(always)]
    pub fn bit_debug_reg0(&self) -> BitDebugReg0R {
        BitDebugReg0R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Write:hps_clkr_ahb\\[0\\] Read:Interrupt.Write clear."]
    #[inline(always)]
    pub fn bit_int_wlmac(&self) -> BitIntWlmacR {
        BitIntWlmacR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - The Registers are used to prevent the deadlock whenever HW or FW turn off ANACLK or SYSCLK, which the HW or FW could not resume by themself. Currently they are not used in normal control flow. They might be helpful when Driver would like to handle whole Suspned & Resume flow by itself hps_clkr_ahb\\[2:1\\]"]
    #[inline(always)]
    pub fn bit_hps_clkr_ahb(&self) -> BitHpsClkrAhbR {
        BitHpsClkrAhbR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Write:hps_clkr_ahb\\[3\\] Read:Debug Selection Register"]
    #[inline(always)]
    pub fn bit_debug_reg1(&self) -> BitDebugReg1R {
        BitDebugReg1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug Selection Register"]
    #[inline(always)]
    pub fn bit_debug_reg2(&self) -> BitDebugReg2R {
        BitDebugReg2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Debug Selection Register"]
    #[inline(always)]
    pub fn bit_debug_reg3(&mut self) -> BitDebugReg3W<'_, RegAxiCtrl2Spec> {
        BitDebugReg3W::new(self, 0)
    }
    #[doc = "Bits 24:26 - Write:Debug Selection Register Read:3'b000"]
    #[inline(always)]
    pub fn bit_debug_reg0(&mut self) -> BitDebugReg0W<'_, RegAxiCtrl2Spec> {
        BitDebugReg0W::new(self, 24)
    }
    #[doc = "Bit 27 - Write:hps_clkr_ahb\\[0\\] Read:Interrupt.Write clear."]
    #[inline(always)]
    pub fn bit_int_wlmac(&mut self) -> BitIntWlmacW<'_, RegAxiCtrl2Spec> {
        BitIntWlmacW::new(self, 27)
    }
    #[doc = "Bits 28:29 - The Registers are used to prevent the deadlock whenever HW or FW turn off ANACLK or SYSCLK, which the HW or FW could not resume by themself. Currently they are not used in normal control flow. They might be helpful when Driver would like to handle whole Suspned & Resume flow by itself hps_clkr_ahb\\[2:1\\]"]
    #[inline(always)]
    pub fn bit_hps_clkr_ahb(&mut self) -> BitHpsClkrAhbW<'_, RegAxiCtrl2Spec> {
        BitHpsClkrAhbW::new(self, 28)
    }
    #[doc = "Bit 30 - Write:hps_clkr_ahb\\[3\\] Read:Debug Selection Register"]
    #[inline(always)]
    pub fn bit_debug_reg1(&mut self) -> BitDebugReg1W<'_, RegAxiCtrl2Spec> {
        BitDebugReg1W::new(self, 30)
    }
    #[doc = "Bit 31 - Debug Selection Register"]
    #[inline(always)]
    pub fn bit_debug_reg2(&mut self) -> BitDebugReg2W<'_, RegAxiCtrl2Spec> {
        BitDebugReg2W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_axi_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_axi_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxiCtrl2Spec;
impl crate::RegisterSpec for RegAxiCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_axi_ctrl2::R`](R) reader structure"]
impl crate::Readable for RegAxiCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_axi_ctrl2::W`](W) writer structure"]
impl crate::Writable for RegAxiCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AXI_CTRL2 to value 0x04"]
impl crate::Resettable for RegAxiCtrl2Spec {
    const RESET_VALUE: u32 = 0x04;
}
