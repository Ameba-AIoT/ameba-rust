#[doc = "Register `REG_SW_RST_CTRL` reader"]
pub type R = crate::R<RegSwRstCtrlSpec>;
#[doc = "Register `REG_SW_RST_CTRL` writer"]
pub type W = crate::W<RegSwRstCtrlSpec>;
#[doc = "Field `othercpu_rst_en` reader - 1. warm reset request by one cpu will reset other cpu's core and local cpu 0. warm reset request by one cpu will only reset local cpu"]
pub type OthercpuRstEnR = crate::BitReader;
#[doc = "Field `othercpu_rst_en` writer - 1. warm reset request by one cpu will reset other cpu's core and local cpu 0. warm reset request by one cpu will only reset local cpu"]
pub type OthercpuRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `periall_rst_en` reader - 1. warm reset request by one cpu will reset all peri and local cpu 0. warm reset request by one cpu will only reset local cpu Note : peri rst will use sysrst mst also"]
pub type PeriallRstEnR = crate::BitReader;
#[doc = "Field `periall_rst_en` writer - 1. warm reset request by one cpu will reset all peri and local cpu 0. warm reset request by one cpu will only reset local cpu Note : peri rst will use sysrst mst also"]
pub type PeriallRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdg0_glb_rst_en` reader - 1.wdg0 will reset all syson IP and local CPU 0.wdg0 will reset local CPU and itself Note: wdg0 's local CPU is CPU1"]
pub type Wdg0GlbRstEnR = crate::BitReader;
#[doc = "Field `wdg0_glb_rst_en` writer - 1.wdg0 will reset all syson IP and local CPU 0.wdg0 will reset local CPU and itself Note: wdg0 's local CPU is CPU1"]
pub type Wdg0GlbRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdg1_glb_rst_en` reader - 1.wdg1 will reset all syson IP and local CPU 0.wdg1 will reset local CPU and itself Note: wdg1 's local CPU is CPU0"]
pub type Wdg1GlbRstEnR = crate::BitReader;
#[doc = "Field `wdg1_glb_rst_en` writer - 1.wdg1 will reset all syson IP and local CPU 0.wdg1 will reset local CPU and itself Note: wdg1 's local CPU is CPU0"]
pub type Wdg1GlbRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdg2_glb_rst_en` reader - 1.wdg2 will reset all syson IP and local CPU 0.wdg2 will reset local CPU and itself Note: wdg2 's local CPU is CPU0"]
pub type Wdg2GlbRstEnR = crate::BitReader;
#[doc = "Field `wdg2_glb_rst_en` writer - 1.wdg2 will reset all syson IP and local CPU 0.wdg2 will reset local CPU and itself Note: wdg2 's local CPU is CPU0"]
pub type Wdg2GlbRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdg_glbrst_timeout` reader - If wdg's glb rst en is disable , wdg reset will reset its local cpu only . but if bus is hang , it will trigger global reset also . HW will wait local CPU's status , if it is not in IDLE in a threshold time, it will trigger global reset . Note : This value must set > 1 , it is base on 32k clock cycle ."]
pub type WdgGlbrstTimeoutR = crate::FieldReader;
#[doc = "Field `wdg_glbrst_timeout` writer - If wdg's glb rst en is disable , wdg reset will reset its local cpu only . but if bus is hang , it will trigger global reset also . HW will wait local CPU's status , if it is not in IDLE in a threshold time, it will trigger global reset . Note : This value must set > 1 , it is base on 32k clock cycle ."]
pub type WdgGlbrstTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `sys_glbrst_timeout` reader - If system glb rst en is disable , system reset will reset its local cpu only . but if bus is hang , it will trigger global reset also . HW will wait local CPU's status , if it is not in IDLE in a threshold time, it will trigger global reset . Note : This value must set > 1 , it is base on 4M clock cycle ."]
pub type SysGlbrstTimeoutR = crate::FieldReader;
#[doc = "Field `sys_glbrst_timeout` writer - If system glb rst en is disable , system reset will reset its local cpu only . but if bus is hang , it will trigger global reset also . HW will wait local CPU's status , if it is not in IDLE in a threshold time, it will trigger global reset . Note : This value must set > 1 , it is base on 4M clock cycle ."]
pub type SysGlbrstTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `srst0_glb_rst_en` reader - 1.system reset0 will reset all syson IP and local CPU0(km4tz) 0.system reset0 will reset local CPU0(km4tz)"]
pub type Srst0GlbRstEnR = crate::BitReader;
#[doc = "Field `srst0_glb_rst_en` writer - 1.system reset0 will reset all syson IP and local CPU0(km4tz) 0.system reset0 will reset local CPU0(km4tz)"]
pub type Srst0GlbRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `srst1_glb_rst_en` reader - 1.system reset1 will reset all syson IP and local CPU1(km4ns) 0.system reset0 will reset local CPU1(km4ns)"]
pub type Srst1GlbRstEnR = crate::BitReader;
#[doc = "Field `srst1_glb_rst_en` writer - 1.system reset1 will reset all syson IP and local CPU1(km4ns) 0.system reset0 will reset local CPU1(km4ns)"]
pub type Srst1GlbRstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4ns_sys_rst` reader - Only when REG_SW_RST_TRIG is 0x96969696 , this reg can be write"]
pub type Km4nsSysRstR = crate::BitReader;
#[doc = "Field `km4ns_sys_rst` writer - Only when REG_SW_RST_TRIG is 0x96969696 , this reg can be write"]
pub type Km4nsSysRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4tz_sys_rst` reader - Only when REG_SW_RST_TRIG is 0x96969696 , this reg can be write"]
pub type Km4tzSysRstR = crate::BitReader;
#[doc = "Field `km4tz_sys_rst` writer - Only when REG_SW_RST_TRIG is 0x96969696 , this reg can be write"]
pub type Km4tzSysRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1. warm reset request by one cpu will reset other cpu's core and local cpu 0. warm reset request by one cpu will only reset local cpu"]
    #[inline(always)]
    pub fn othercpu_rst_en(&self) -> OthercpuRstEnR {
        OthercpuRstEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1. warm reset request by one cpu will reset all peri and local cpu 0. warm reset request by one cpu will only reset local cpu Note : peri rst will use sysrst mst also"]
    #[inline(always)]
    pub fn periall_rst_en(&self) -> PeriallRstEnR {
        PeriallRstEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 1.wdg0 will reset all syson IP and local CPU 0.wdg0 will reset local CPU and itself Note: wdg0 's local CPU is CPU1"]
    #[inline(always)]
    pub fn wdg0_glb_rst_en(&self) -> Wdg0GlbRstEnR {
        Wdg0GlbRstEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1.wdg1 will reset all syson IP and local CPU 0.wdg1 will reset local CPU and itself Note: wdg1 's local CPU is CPU0"]
    #[inline(always)]
    pub fn wdg1_glb_rst_en(&self) -> Wdg1GlbRstEnR {
        Wdg1GlbRstEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1.wdg2 will reset all syson IP and local CPU 0.wdg2 will reset local CPU and itself Note: wdg2 's local CPU is CPU0"]
    #[inline(always)]
    pub fn wdg2_glb_rst_en(&self) -> Wdg2GlbRstEnR {
        Wdg2GlbRstEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - If wdg's glb rst en is disable , wdg reset will reset its local cpu only . but if bus is hang , it will trigger global reset also . HW will wait local CPU's status , if it is not in IDLE in a threshold time, it will trigger global reset . Note : This value must set > 1 , it is base on 32k clock cycle ."]
    #[inline(always)]
    pub fn wdg_glbrst_timeout(&self) -> WdgGlbrstTimeoutR {
        WdgGlbrstTimeoutR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - If system glb rst en is disable , system reset will reset its local cpu only . but if bus is hang , it will trigger global reset also . HW will wait local CPU's status , if it is not in IDLE in a threshold time, it will trigger global reset . Note : This value must set > 1 , it is base on 4M clock cycle ."]
    #[inline(always)]
    pub fn sys_glbrst_timeout(&self) -> SysGlbrstTimeoutR {
        SysGlbrstTimeoutR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1.system reset0 will reset all syson IP and local CPU0(km4tz) 0.system reset0 will reset local CPU0(km4tz)"]
    #[inline(always)]
    pub fn srst0_glb_rst_en(&self) -> Srst0GlbRstEnR {
        Srst0GlbRstEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1.system reset1 will reset all syson IP and local CPU1(km4ns) 0.system reset0 will reset local CPU1(km4ns)"]
    #[inline(always)]
    pub fn srst1_glb_rst_en(&self) -> Srst1GlbRstEnR {
        Srst1GlbRstEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Only when REG_SW_RST_TRIG is 0x96969696 , this reg can be write"]
    #[inline(always)]
    pub fn km4ns_sys_rst(&self) -> Km4nsSysRstR {
        Km4nsSysRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Only when REG_SW_RST_TRIG is 0x96969696 , this reg can be write"]
    #[inline(always)]
    pub fn km4tz_sys_rst(&self) -> Km4tzSysRstR {
        Km4tzSysRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1. warm reset request by one cpu will reset other cpu's core and local cpu 0. warm reset request by one cpu will only reset local cpu"]
    #[inline(always)]
    pub fn othercpu_rst_en(&mut self) -> OthercpuRstEnW<'_, RegSwRstCtrlSpec> {
        OthercpuRstEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1. warm reset request by one cpu will reset all peri and local cpu 0. warm reset request by one cpu will only reset local cpu Note : peri rst will use sysrst mst also"]
    #[inline(always)]
    pub fn periall_rst_en(&mut self) -> PeriallRstEnW<'_, RegSwRstCtrlSpec> {
        PeriallRstEnW::new(self, 1)
    }
    #[doc = "Bit 4 - 1.wdg0 will reset all syson IP and local CPU 0.wdg0 will reset local CPU and itself Note: wdg0 's local CPU is CPU1"]
    #[inline(always)]
    pub fn wdg0_glb_rst_en(&mut self) -> Wdg0GlbRstEnW<'_, RegSwRstCtrlSpec> {
        Wdg0GlbRstEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 1.wdg1 will reset all syson IP and local CPU 0.wdg1 will reset local CPU and itself Note: wdg1 's local CPU is CPU0"]
    #[inline(always)]
    pub fn wdg1_glb_rst_en(&mut self) -> Wdg1GlbRstEnW<'_, RegSwRstCtrlSpec> {
        Wdg1GlbRstEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 1.wdg2 will reset all syson IP and local CPU 0.wdg2 will reset local CPU and itself Note: wdg2 's local CPU is CPU0"]
    #[inline(always)]
    pub fn wdg2_glb_rst_en(&mut self) -> Wdg2GlbRstEnW<'_, RegSwRstCtrlSpec> {
        Wdg2GlbRstEnW::new(self, 6)
    }
    #[doc = "Bits 8:15 - If wdg's glb rst en is disable , wdg reset will reset its local cpu only . but if bus is hang , it will trigger global reset also . HW will wait local CPU's status , if it is not in IDLE in a threshold time, it will trigger global reset . Note : This value must set > 1 , it is base on 32k clock cycle ."]
    #[inline(always)]
    pub fn wdg_glbrst_timeout(&mut self) -> WdgGlbrstTimeoutW<'_, RegSwRstCtrlSpec> {
        WdgGlbrstTimeoutW::new(self, 8)
    }
    #[doc = "Bits 16:23 - If system glb rst en is disable , system reset will reset its local cpu only . but if bus is hang , it will trigger global reset also . HW will wait local CPU's status , if it is not in IDLE in a threshold time, it will trigger global reset . Note : This value must set > 1 , it is base on 4M clock cycle ."]
    #[inline(always)]
    pub fn sys_glbrst_timeout(&mut self) -> SysGlbrstTimeoutW<'_, RegSwRstCtrlSpec> {
        SysGlbrstTimeoutW::new(self, 16)
    }
    #[doc = "Bit 24 - 1.system reset0 will reset all syson IP and local CPU0(km4tz) 0.system reset0 will reset local CPU0(km4tz)"]
    #[inline(always)]
    pub fn srst0_glb_rst_en(&mut self) -> Srst0GlbRstEnW<'_, RegSwRstCtrlSpec> {
        Srst0GlbRstEnW::new(self, 24)
    }
    #[doc = "Bit 25 - 1.system reset1 will reset all syson IP and local CPU1(km4ns) 0.system reset0 will reset local CPU1(km4ns)"]
    #[inline(always)]
    pub fn srst1_glb_rst_en(&mut self) -> Srst1GlbRstEnW<'_, RegSwRstCtrlSpec> {
        Srst1GlbRstEnW::new(self, 25)
    }
    #[doc = "Bit 30 - Only when REG_SW_RST_TRIG is 0x96969696 , this reg can be write"]
    #[inline(always)]
    pub fn km4ns_sys_rst(&mut self) -> Km4nsSysRstW<'_, RegSwRstCtrlSpec> {
        Km4nsSysRstW::new(self, 30)
    }
    #[doc = "Bit 31 - Only when REG_SW_RST_TRIG is 0x96969696 , this reg can be write"]
    #[inline(always)]
    pub fn km4tz_sys_rst(&mut self) -> Km4tzSysRstW<'_, RegSwRstCtrlSpec> {
        Km4tzSysRstW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sw_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sw_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwRstCtrlSpec;
impl crate::RegisterSpec for RegSwRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sw_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSwRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sw_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSwRstCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SW_RST_CTRL to value 0x03ff_0f70"]
impl crate::Resettable for RegSwRstCtrlSpec {
    const RESET_VALUE: u32 = 0x03ff_0f70;
}
