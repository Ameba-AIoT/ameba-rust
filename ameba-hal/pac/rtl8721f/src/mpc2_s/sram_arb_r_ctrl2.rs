#[doc = "Register `SRAM_ARB_R_CTRL2` reader"]
pub type R = crate::R<SramArbRCtrl2Spec>;
#[doc = "Register `SRAM_ARB_R_CTRL2` writer"]
pub type W = crate::W<SramArbRCtrl2Spec>;
#[doc = "Field `64B_HPERI_TO_SRAM_ARB` reader - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: AES_SHA 0x1: MJPEG And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type _64bHperiToSramArbR = crate::FieldReader;
#[doc = "Field `64B_HPERI_TO_SRAM_ARB` writer - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: AES_SHA 0x1: MJPEG And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type _64bHperiToSramArbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ALL_HPERI_TO_SRAM_ARB` reader - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: 32B_HPERI_TO_SRAM_ARB group 0x1: 64B_HPERI_TO_SRAM_ARB group And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type AllHperiToSramArbR = crate::FieldReader;
#[doc = "Field `ALL_HPERI_TO_SRAM_ARB` writer - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: 32B_HPERI_TO_SRAM_ARB group 0x1: 64B_HPERI_TO_SRAM_ARB group And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
pub type AllHperiToSramArbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRAM_FINAL_ARB` reader - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU_TO_SRAM_ARB group 0x1: ALL_HPERI_TO_SRAM_ARB group 0x2: SHPERI_TO_SRAM_ARB group And the following shows bit field of each slots: \\[1:0\\]: slot 0 of arbiter \\[3:2\\]: slot 1 of arbiter \\[5:4\\]: slot 2 of arbiter \\[7:6\\]: slot 3 of arbiter \\[9:8\\]: slot 4 of arbiter \\[11:10\\]: slot 5 of arbiter \\[13:12\\]: slot 6 of arbiter \\[15:14\\]: slot 7 of arbiter"]
pub type SramFinalArbR = crate::FieldReader<u16>;
#[doc = "Field `SRAM_FINAL_ARB` writer - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU_TO_SRAM_ARB group 0x1: ALL_HPERI_TO_SRAM_ARB group 0x2: SHPERI_TO_SRAM_ARB group And the following shows bit field of each slots: \\[1:0\\]: slot 0 of arbiter \\[3:2\\]: slot 1 of arbiter \\[5:4\\]: slot 2 of arbiter \\[7:6\\]: slot 3 of arbiter \\[9:8\\]: slot 4 of arbiter \\[11:10\\]: slot 5 of arbiter \\[13:12\\]: slot 6 of arbiter \\[15:14\\]: slot 7 of arbiter"]
pub type SramFinalArbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: AES_SHA 0x1: MJPEG And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn _64b_hperi_to_sram_arb(&self) -> _64bHperiToSramArbR {
        _64bHperiToSramArbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: 32B_HPERI_TO_SRAM_ARB group 0x1: 64B_HPERI_TO_SRAM_ARB group And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn all_hperi_to_sram_arb(&self) -> AllHperiToSramArbR {
        AllHperiToSramArbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU_TO_SRAM_ARB group 0x1: ALL_HPERI_TO_SRAM_ARB group 0x2: SHPERI_TO_SRAM_ARB group And the following shows bit field of each slots: \\[1:0\\]: slot 0 of arbiter \\[3:2\\]: slot 1 of arbiter \\[5:4\\]: slot 2 of arbiter \\[7:6\\]: slot 3 of arbiter \\[9:8\\]: slot 4 of arbiter \\[11:10\\]: slot 5 of arbiter \\[13:12\\]: slot 6 of arbiter \\[15:14\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn sram_final_arb(&self) -> SramFinalArbR {
        SramFinalArbR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: AES_SHA 0x1: MJPEG And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn _64b_hperi_to_sram_arb(&mut self) -> _64bHperiToSramArbW<'_, SramArbRCtrl2Spec> {
        _64bHperiToSramArbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: 32B_HPERI_TO_SRAM_ARB group 0x1: 64B_HPERI_TO_SRAM_ARB group And the following shows bit field of each slots: \\[0\\]: slot 0 of arbiter \\[1\\]: slot 1 of arbiter \\[2\\]: slot 2 of arbiter \\[3\\]: slot 3 of arbiter \\[4\\]: slot 4 of arbiter \\[5\\]: slot 5 of arbiter \\[6\\]: slot 6 of arbiter \\[7\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn all_hperi_to_sram_arb(&mut self) -> AllHperiToSramArbW<'_, SramArbRCtrl2Spec> {
        AllHperiToSramArbW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Read arbitration control register. Each slot should be set by a port number. The port number represents a master group.The master group is shown as following: 0x0: CPU_TO_SRAM_ARB group 0x1: ALL_HPERI_TO_SRAM_ARB group 0x2: SHPERI_TO_SRAM_ARB group And the following shows bit field of each slots: \\[1:0\\]: slot 0 of arbiter \\[3:2\\]: slot 1 of arbiter \\[5:4\\]: slot 2 of arbiter \\[7:6\\]: slot 3 of arbiter \\[9:8\\]: slot 4 of arbiter \\[11:10\\]: slot 5 of arbiter \\[13:12\\]: slot 6 of arbiter \\[15:14\\]: slot 7 of arbiter"]
    #[inline(always)]
    pub fn sram_final_arb(&mut self) -> SramFinalArbW<'_, SramArbRCtrl2Spec> {
        SramFinalArbW::new(self, 16)
    }
}
#[doc = "The register for CPU/LCDC/HPERI master group to SRAM read arbitration\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_arb_r_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_arb_r_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramArbRCtrl2Spec;
impl crate::RegisterSpec for SramArbRCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_arb_r_ctrl2::R`](R) reader structure"]
impl crate::Readable for SramArbRCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_arb_r_ctrl2::W`](W) writer structure"]
impl crate::Writable for SramArbRCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_ARB_R_CTRL2 to value 0x0a50_f0f0"]
impl crate::Resettable for SramArbRCtrl2Spec {
    const RESET_VALUE: u32 = 0x0a50_f0f0;
}
