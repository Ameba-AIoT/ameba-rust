#[doc = "Register `REG_CKE_GRP2` reader"]
pub type R = crate::R<RegCkeGrp2Spec>;
#[doc = "Register `REG_CKE_GRP2` writer"]
pub type W = crate::W<RegCkeGrp2Spec>;
#[doc = "Field `cke_osc4m_soc` reader - 1.Enable 4M clk path for soc clk 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
pub type CkeOsc4mSocR = crate::BitReader;
#[doc = "Field `cke_osc4m_soc` writer - 1.Enable 4M clk path for soc clk 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
pub type CkeOsc4mSocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_ret` reader - 1.Enable Some IP's clock who has retention function when pmc state change 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
pub type CkeRetR = crate::BitReader;
#[doc = "Field `cke_ret` writer - 1.Enable Some IP's clock who has retention function when pmc state change 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
pub type CkeRetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_sys_32k` reader - 1.Enable system/wifi 32k enable 0: Disable"]
pub type CkeSys32kR = crate::BitReader;
#[doc = "Field `cke_sys_32k` writer - 1.Enable system/wifi 32k enable 0: Disable"]
pub type CkeSys32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_bt_32k` reader - 1.Enable bt 32k enable 0: Disable"]
pub type CkeBt32kR = crate::BitReader;
#[doc = "Field `cke_bt_32k` writer - 1.Enable bt 32k enable 0: Disable"]
pub type CkeBt32kW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1.Enable 4M clk path for soc clk 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
    #[inline(always)]
    pub fn cke_osc4m_soc(&self) -> CkeOsc4mSocR {
        CkeOsc4mSocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1.Enable Some IP's clock who has retention function when pmc state change 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
    #[inline(always)]
    pub fn cke_ret(&self) -> CkeRetR {
        CkeRetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - 1.Enable system/wifi 32k enable 0: Disable"]
    #[inline(always)]
    pub fn cke_sys_32k(&self) -> CkeSys32kR {
        CkeSys32kR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1.Enable bt 32k enable 0: Disable"]
    #[inline(always)]
    pub fn cke_bt_32k(&self) -> CkeBt32kR {
        CkeBt32kR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1.Enable 4M clk path for soc clk 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
    #[inline(always)]
    pub fn cke_osc4m_soc(&mut self) -> CkeOsc4mSocW<'_, RegCkeGrp2Spec> {
        CkeOsc4mSocW::new(self, 0)
    }
    #[doc = "Bit 1 - 1.Enable Some IP's clock who has retention function when pmc state change 0: Disable Note: this bit is controlled HW PMC, SW shall not modify"]
    #[inline(always)]
    pub fn cke_ret(&mut self) -> CkeRetW<'_, RegCkeGrp2Spec> {
        CkeRetW::new(self, 1)
    }
    #[doc = "Bit 12 - 1.Enable system/wifi 32k enable 0: Disable"]
    #[inline(always)]
    pub fn cke_sys_32k(&mut self) -> CkeSys32kW<'_, RegCkeGrp2Spec> {
        CkeSys32kW::new(self, 12)
    }
    #[doc = "Bit 13 - 1.Enable bt 32k enable 0: Disable"]
    #[inline(always)]
    pub fn cke_bt_32k(&mut self) -> CkeBt32kW<'_, RegCkeGrp2Spec> {
        CkeBt32kW::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cke_grp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cke_grp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkeGrp2Spec;
impl crate::RegisterSpec for RegCkeGrp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cke_grp2::R`](R) reader structure"]
impl crate::Readable for RegCkeGrp2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_cke_grp2::W`](W) writer structure"]
impl crate::Writable for RegCkeGrp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKE_GRP2 to value 0x3000"]
impl crate::Resettable for RegCkeGrp2Spec {
    const RESET_VALUE: u32 = 0x3000;
}
