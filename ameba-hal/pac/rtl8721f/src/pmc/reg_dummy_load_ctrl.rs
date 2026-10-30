#[doc = "Register `REG_DUMMY_LOAD_CTRL` reader"]
pub type R = crate::R<RegDummyLoadCtrlSpec>;
#[doc = "Register `REG_DUMMY_LOAD_CTRL` writer"]
pub type W = crate::W<RegDummyLoadCtrlSpec>;
#[doc = "Field `SYS_DMYL_MNL` reader - system use this bit to switch core ldo dummy load control path manually 0: dummy load control is decided by LS/BT/WL PMC arbitration 1: dummy load control is decided by KM4 sleep state"]
pub type SysDmylMnlR = crate::BitReader;
#[doc = "Field `SYS_DMYL_MNL` writer - system use this bit to switch core ldo dummy load control path manually 0: dummy load control is decided by LS/BT/WL PMC arbitration 1: dummy load control is decided by KM4 sleep state"]
pub type SysDmylMnlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_DMYL_ACT` reader - this bit means the dummy load state when KM4 is active 0: disable 1: enable"]
pub type SysDmylActR = crate::BitReader;
#[doc = "Field `SYS_DMYL_ACT` writer - this bit means the dummy load state when KM4 is active 0: disable 1: enable"]
pub type SysDmylActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_DMYL_SLP` reader - this bit means the dummy load state when KM4 is sleep 0: disable 1: enable"]
pub type SysDmylSlpR = crate::BitReader;
#[doc = "Field `SYS_DMYL_SLP` writer - this bit means the dummy load state when KM4 is sleep 0: disable 1: enable"]
pub type SysDmylSlpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - system use this bit to switch core ldo dummy load control path manually 0: dummy load control is decided by LS/BT/WL PMC arbitration 1: dummy load control is decided by KM4 sleep state"]
    #[inline(always)]
    pub fn sys_dmyl_mnl(&self) -> SysDmylMnlR {
        SysDmylMnlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit means the dummy load state when KM4 is active 0: disable 1: enable"]
    #[inline(always)]
    pub fn sys_dmyl_act(&self) -> SysDmylActR {
        SysDmylActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit means the dummy load state when KM4 is sleep 0: disable 1: enable"]
    #[inline(always)]
    pub fn sys_dmyl_slp(&self) -> SysDmylSlpR {
        SysDmylSlpR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - system use this bit to switch core ldo dummy load control path manually 0: dummy load control is decided by LS/BT/WL PMC arbitration 1: dummy load control is decided by KM4 sleep state"]
    #[inline(always)]
    pub fn sys_dmyl_mnl(&mut self) -> SysDmylMnlW<'_, RegDummyLoadCtrlSpec> {
        SysDmylMnlW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit means the dummy load state when KM4 is active 0: disable 1: enable"]
    #[inline(always)]
    pub fn sys_dmyl_act(&mut self) -> SysDmylActW<'_, RegDummyLoadCtrlSpec> {
        SysDmylActW::new(self, 1)
    }
    #[doc = "Bit 2 - this bit means the dummy load state when KM4 is sleep 0: disable 1: enable"]
    #[inline(always)]
    pub fn sys_dmyl_slp(&mut self) -> SysDmylSlpW<'_, RegDummyLoadCtrlSpec> {
        SysDmylSlpW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy_load_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy_load_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummyLoadCtrlSpec;
impl crate::RegisterSpec for RegDummyLoadCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy_load_ctrl::R`](R) reader structure"]
impl crate::Readable for RegDummyLoadCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy_load_ctrl::W`](W) writer structure"]
impl crate::Writable for RegDummyLoadCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY_LOAD_CTRL to value 0x04"]
impl crate::Resettable for RegDummyLoadCtrlSpec {
    const RESET_VALUE: u32 = 0x04;
}
