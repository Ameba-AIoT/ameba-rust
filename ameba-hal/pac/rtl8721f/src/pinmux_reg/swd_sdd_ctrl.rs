#[doc = "Register `SWD_SDD_CTRL` reader"]
pub type R = crate::R<SwdSddCtrlSpec>;
#[doc = "Register `SWD_SDD_CTRL` writer"]
pub type W = crate::W<SwdSddCtrlSpec>;
#[doc = "Field `SWD_PMUX_EN` reader - 1: Force enable share SWD pinmux function 0: Disable"]
pub type SwdPmuxEnR = crate::BitReader;
#[doc = "Field `SWD_PMUX_EN` writer - 1: Force enable share SWD pinmux function 0: Disable"]
pub type SwdPmuxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4_swd_sel` reader - Toggle this bit will switch shared SWD port, this bit only accessed by KM4. Default is KM4 SWD, if km4 toggle this bit , SWD port will switch to KR4"]
pub type Km4SwdSelR = crate::BitReader;
#[doc = "Field `km4_swd_sel` writer - Toggle this bit will switch shared SWD port, this bit only accessed by KM4. Default is KM4 SWD, if km4 toggle this bit , SWD port will switch to KR4"]
pub type Km4SwdSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `kscan_pupd_ctrl` reader - enable kscan pupd hw control logic, active 1"]
pub type KscanPupdCtrlR = crate::BitReader;
#[doc = "Field `kscan_pupd_ctrl` writer - enable kscan pupd hw control logic, active 1"]
pub type KscanPupdCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Force enable share SWD pinmux function 0: Disable"]
    #[inline(always)]
    pub fn swd_pmux_en(&self) -> SwdPmuxEnR {
        SwdPmuxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Toggle this bit will switch shared SWD port, this bit only accessed by KM4. Default is KM4 SWD, if km4 toggle this bit , SWD port will switch to KR4"]
    #[inline(always)]
    pub fn km4_swd_sel(&self) -> Km4SwdSelR {
        Km4SwdSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - enable kscan pupd hw control logic, active 1"]
    #[inline(always)]
    pub fn kscan_pupd_ctrl(&self) -> KscanPupdCtrlR {
        KscanPupdCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Force enable share SWD pinmux function 0: Disable"]
    #[inline(always)]
    pub fn swd_pmux_en(&mut self) -> SwdPmuxEnW<'_, SwdSddCtrlSpec> {
        SwdPmuxEnW::new(self, 0)
    }
    #[doc = "Bit 3 - Toggle this bit will switch shared SWD port, this bit only accessed by KM4. Default is KM4 SWD, if km4 toggle this bit , SWD port will switch to KR4"]
    #[inline(always)]
    pub fn km4_swd_sel(&mut self) -> Km4SwdSelW<'_, SwdSddCtrlSpec> {
        Km4SwdSelW::new(self, 3)
    }
    #[doc = "Bit 8 - enable kscan pupd hw control logic, active 1"]
    #[inline(always)]
    pub fn kscan_pupd_ctrl(&mut self) -> KscanPupdCtrlW<'_, SwdSddCtrlSpec> {
        KscanPupdCtrlW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`swd_sdd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swd_sdd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwdSddCtrlSpec;
impl crate::RegisterSpec for SwdSddCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swd_sdd_ctrl::R`](R) reader structure"]
impl crate::Readable for SwdSddCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`swd_sdd_ctrl::W`](W) writer structure"]
impl crate::Writable for SwdSddCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWD_SDD_CTRL to value 0x0109"]
impl crate::Resettable for SwdSddCtrlSpec {
    const RESET_VALUE: u32 = 0x0109;
}
