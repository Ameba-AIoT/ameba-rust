#[doc = "Register `REG_CLK_FORCE` reader"]
pub type R = crate::R<RegClkForceSpec>;
#[doc = "Register `REG_CLK_FORCE` writer"]
pub type W = crate::W<RegClkForceSpec>;
#[doc = "Field `osc4m_req_force` reader - force to request osc4m 1:osc4m will never shutdown 0:osc4m will auto shutdown when no IP use it"]
pub type Osc4mReqForceR = crate::BitReader;
#[doc = "Field `osc4m_req_force` writer - force to request osc4m 1:osc4m will never shutdown 0:osc4m will auto shutdown when no IP use it"]
pub type Osc4mReqForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb_pll_div3_force` reader - force to enable usb pll div3 1:usb pll div3 clock always enable 0:usb pll div3 clock always auto gate when no IP use it"]
pub type UsbPllDiv3ForceR = crate::BitReader;
#[doc = "Field `usb_pll_div3_force` writer - force to enable usb pll div3 1:usb pll div3 clock always enable 0:usb pll div3 clock always auto gate when no IP use it"]
pub type UsbPllDiv3ForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sys_pll_div3_force` reader - force to enable sys pll div3 1:sys pll div3 clock always enable 0:sys pll div3 clock always auto gate when no IP use it"]
pub type SysPllDiv3ForceR = crate::BitReader;
#[doc = "Field `sys_pll_div3_force` writer - force to enable sys pll div3 1:sys pll div3 clock always enable 0:sys pll div3 clock always auto gate when no IP use it"]
pub type SysPllDiv3ForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - force to request osc4m 1:osc4m will never shutdown 0:osc4m will auto shutdown when no IP use it"]
    #[inline(always)]
    pub fn osc4m_req_force(&self) -> Osc4mReqForceR {
        Osc4mReqForceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force to enable usb pll div3 1:usb pll div3 clock always enable 0:usb pll div3 clock always auto gate when no IP use it"]
    #[inline(always)]
    pub fn usb_pll_div3_force(&self) -> UsbPllDiv3ForceR {
        UsbPllDiv3ForceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - force to enable sys pll div3 1:sys pll div3 clock always enable 0:sys pll div3 clock always auto gate when no IP use it"]
    #[inline(always)]
    pub fn sys_pll_div3_force(&self) -> SysPllDiv3ForceR {
        SysPllDiv3ForceR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - force to request osc4m 1:osc4m will never shutdown 0:osc4m will auto shutdown when no IP use it"]
    #[inline(always)]
    pub fn osc4m_req_force(&mut self) -> Osc4mReqForceW<'_, RegClkForceSpec> {
        Osc4mReqForceW::new(self, 0)
    }
    #[doc = "Bit 1 - force to enable usb pll div3 1:usb pll div3 clock always enable 0:usb pll div3 clock always auto gate when no IP use it"]
    #[inline(always)]
    pub fn usb_pll_div3_force(&mut self) -> UsbPllDiv3ForceW<'_, RegClkForceSpec> {
        UsbPllDiv3ForceW::new(self, 1)
    }
    #[doc = "Bit 2 - force to enable sys pll div3 1:sys pll div3 clock always enable 0:sys pll div3 clock always auto gate when no IP use it"]
    #[inline(always)]
    pub fn sys_pll_div3_force(&mut self) -> SysPllDiv3ForceW<'_, RegClkForceSpec> {
        SysPllDiv3ForceW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clk_force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clk_force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegClkForceSpec;
impl crate::RegisterSpec for RegClkForceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_clk_force::R`](R) reader structure"]
impl crate::Readable for RegClkForceSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_clk_force::W`](W) writer structure"]
impl crate::Writable for RegClkForceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLK_FORCE to value 0"]
impl crate::Resettable for RegClkForceSpec {}
