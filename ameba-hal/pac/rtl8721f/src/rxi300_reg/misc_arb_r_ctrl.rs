#[doc = "Register `MISC_ARB_R_CTRL` reader"]
pub type R = crate::R<MiscArbRCtrlSpec>;
#[doc = "Register `MISC_ARB_R_CTRL` writer"]
pub type W = crate::W<MiscArbRCtrlSpec>;
#[doc = "Field `CPU_TO_SRAM_ARB` reader - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0 0x1: CPU1 And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type CpuToSramArbR = crate::FieldReader;
#[doc = "Field `CPU_TO_SRAM_ARB` writer - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0 0x1: CPU1 And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type CpuToSramArbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_TO_PSRAM_ARB` reader - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0 0x1: CPU1 And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type CpuToPsramArbR = crate::FieldReader;
#[doc = "Field `CPU_TO_PSRAM_ARB` writer - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0 0x1: CPU1 And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type CpuToPsramArbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHPERI_TO_SRAM_ARB` reader - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: LCDC 0x1: PPE And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type ShperiToSramArbR = crate::FieldReader;
#[doc = "Field `SHPERI_TO_SRAM_ARB` writer - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: LCDC 0x1: PPE And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type ShperiToSramArbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0 0x1: CPU1 And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn cpu_to_sram_arb(&self) -> CpuToSramArbR {
        CpuToSramArbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0 0x1: CPU1 And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn cpu_to_psram_arb(&self) -> CpuToPsramArbR {
        CpuToPsramArbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: LCDC 0x1: PPE And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn shperi_to_sram_arb(&self) -> ShperiToSramArbR {
        ShperiToSramArbR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0 0x1: CPU1 And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn cpu_to_sram_arb(&mut self) -> CpuToSramArbW<'_, MiscArbRCtrlSpec> {
        CpuToSramArbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU0 0x1: CPU1 And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn cpu_to_psram_arb(&mut self) -> CpuToPsramArbW<'_, MiscArbRCtrlSpec> {
        CpuToPsramArbW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: LCDC 0x1: PPE And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn shperi_to_sram_arb(&mut self) -> ShperiToSramArbW<'_, MiscArbRCtrlSpec> {
        ShperiToSramArbW::new(self, 16)
    }
}
#[doc = "The register for CPU/HPERI to SRAM/PSRAM read arbitration\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_arb_r_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_arb_r_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscArbRCtrlSpec;
impl crate::RegisterSpec for MiscArbRCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_arb_r_ctrl::R`](R) reader structure"]
impl crate::Readable for MiscArbRCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_arb_r_ctrl::W`](W) writer structure"]
impl crate::Writable for MiscArbRCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC_ARB_R_CTRL to value 0x00f0_f0f0"]
impl crate::Resettable for MiscArbRCtrlSpec {
    const RESET_VALUE: u32 = 0x00f0_f0f0;
}
