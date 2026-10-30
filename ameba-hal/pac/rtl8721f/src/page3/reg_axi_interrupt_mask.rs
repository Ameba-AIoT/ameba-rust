#[doc = "Register `REG_AXI_INTERRUPT_MASK` reader"]
pub type R = crate::R<RegAxiInterruptMaskSpec>;
#[doc = "Register `REG_AXI_INTERRUPT_MASK` writer"]
pub type W = crate::W<RegAxiInterruptMaskSpec>;
#[doc = "Field `BIT_SETRXOK_EN` reader - setrxok, rx dma ok"]
pub type BitSetrxokEnR = crate::BitReader;
#[doc = "Field `BIT_SETRXOK_EN` writer - setrxok, rx dma ok"]
pub type BitSetrxokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SET_RDY_EN` reader - set_rdu, rx descriptor unavailable"]
pub type BitSetRdyEnR = crate::BitReader;
#[doc = "Field `BIT_SET_RDY_EN` writer - set_rdu, rx descriptor unavailable"]
pub type BitSetRdyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETVODOK_EN` reader - setvodok, vo queue ok"]
pub type BitSetvodokEnR = crate::BitReader;
#[doc = "Field `BIT_SETVODOK_EN` writer - setvodok, vo queue ok"]
pub type BitSetvodokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETVIDOK_EN` reader - setvidok, vi queue ok"]
pub type BitSetvidokEnR = crate::BitReader;
#[doc = "Field `BIT_SETVIDOK_EN` writer - setvidok, vi queue ok"]
pub type BitSetvidokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBEDOK_EN` reader - setbedok, be queue ok"]
pub type BitSetbedokEnR = crate::BitReader;
#[doc = "Field `BIT_SETBEDOK_EN` writer - setbedok, be queue ok"]
pub type BitSetbedokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBKDOK_EN` reader - setbkdok, bk queue ok"]
pub type BitSetbkdokEnR = crate::BitReader;
#[doc = "Field `BIT_SETBKDOK_EN` writer - setbkdok, bk queue ok"]
pub type BitSetbkdokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETMGDOK_EN` reader - setmgdok, manage queue ok"]
pub type BitSetmgdokEnR = crate::BitReader;
#[doc = "Field `BIT_SETMGDOK_EN` writer - setmgdok, manage queue ok"]
pub type BitSetmgdokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETH2CDOK_EN` reader - seth2cdok, h2c queue ok"]
pub type BitSeth2cdokEnR = crate::BitReader;
#[doc = "Field `BIT_SETH2CDOK_EN` writer - seth2cdok, h2c queue ok"]
pub type BitSeth2cdokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETHIGHDOK_EN` reader - sethighdok, high queue ok"]
pub type BitSethighdokEnR = crate::BitReader;
#[doc = "Field `BIT_SETHIGHDOK_EN` writer - sethighdok, high queue ok"]
pub type BitSethighdokEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNOK0_EN` reader - setbcndok0, beacon queue ok, mbssid=0"]
pub type BitSetbcnok0EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK0_EN` writer - setbcndok0, beacon queue ok, mbssid=0"]
pub type BitSetbcnok0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNOK1_EN` reader - setbcndok1, beacon queue ok, mbssid=1"]
pub type BitSetbcnok1EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK1_EN` writer - setbcndok1, beacon queue ok, mbssid=1"]
pub type BitSetbcnok1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNOK2_EN` reader - setbcndok2, beacon queue ok, mbssid=2"]
pub type BitSetbcnok2EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK2_EN` writer - setbcndok2, beacon queue ok, mbssid=2"]
pub type BitSetbcnok2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNOK3_EN` reader - setbcndok3, beacon queue ok, mbssid=3"]
pub type BitSetbcnok3EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK3_EN` writer - setbcndok3, beacon queue ok, mbssid=3"]
pub type BitSetbcnok3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNOK4_EN` reader - setbcndok4, beacon queue ok, mbssid=4"]
pub type BitSetbcnok4EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK4_EN` writer - setbcndok4, beacon queue ok, mbssid=4"]
pub type BitSetbcnok4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNOK5_EN` reader - setbcndok5, beacon queue ok, mbssid=5"]
pub type BitSetbcnok5EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK5_EN` writer - setbcndok5, beacon queue ok, mbssid=5"]
pub type BitSetbcnok5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNOK6_EN` reader - setbcndok6, beacon queue ok, mbssid=6"]
pub type BitSetbcnok6EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK6_EN` writer - setbcndok6, beacon queue ok, mbssid=6"]
pub type BitSetbcnok6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNOK7_EN` reader - setbcndok7, beacon queue ok, mbssid=7"]
pub type BitSetbcnok7EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK7_EN` writer - setbcndok7, beacon queue ok, mbssid=7"]
pub type BitSetbcnok7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNERR0_EN` reader - setbcnderr0, beacon queue error, mbssid=0"]
pub type BitSetbcnerr0EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR0_EN` writer - setbcnderr0, beacon queue error, mbssid=0"]
pub type BitSetbcnerr0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNERR1_EN` reader - setbcnderr1, beacon queue error, mbssid=1"]
pub type BitSetbcnerr1EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR1_EN` writer - setbcnderr1, beacon queue error, mbssid=1"]
pub type BitSetbcnerr1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNERR2_EN` reader - setbcnderr2, beacon queue error, mbssid=2"]
pub type BitSetbcnerr2EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR2_EN` writer - setbcnderr2, beacon queue error, mbssid=2"]
pub type BitSetbcnerr2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNERR3_EN` reader - setbcnderr3, beacon queue error, mbssid=3"]
pub type BitSetbcnerr3EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR3_EN` writer - setbcnderr3, beacon queue error, mbssid=3"]
pub type BitSetbcnerr3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNERR4_EN` reader - setbcnderr4, beacon queue error, mbssid=4"]
pub type BitSetbcnerr4EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR4_EN` writer - setbcnderr4, beacon queue error, mbssid=4"]
pub type BitSetbcnerr4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNERR5_EN` reader - setbcnderr5, beacon queue error, mbssid=5"]
pub type BitSetbcnerr5EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR5_EN` writer - setbcnderr5, beacon queue error, mbssid=5"]
pub type BitSetbcnerr5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNERR6_EN` reader - setbcnderr6, beacon queue error, mbssid=6"]
pub type BitSetbcnerr6EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR6_EN` writer - setbcnderr6, beacon queue error, mbssid=6"]
pub type BitSetbcnerr6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SETBCNERR7_EN` reader - setbcnderr7, beacon queue error, mbssid=7"]
pub type BitSetbcnerr7EnR = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR7_EN` writer - setbcnderr7, beacon queue error, mbssid=7"]
pub type BitSetbcnerr7EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - setrxok, rx dma ok"]
    #[inline(always)]
    pub fn bit_setrxok_en(&self) -> BitSetrxokEnR {
        BitSetrxokEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set_rdu, rx descriptor unavailable"]
    #[inline(always)]
    pub fn bit_set_rdy_en(&self) -> BitSetRdyEnR {
        BitSetRdyEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - setvodok, vo queue ok"]
    #[inline(always)]
    pub fn bit_setvodok_en(&self) -> BitSetvodokEnR {
        BitSetvodokEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - setvidok, vi queue ok"]
    #[inline(always)]
    pub fn bit_setvidok_en(&self) -> BitSetvidokEnR {
        BitSetvidokEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - setbedok, be queue ok"]
    #[inline(always)]
    pub fn bit_setbedok_en(&self) -> BitSetbedokEnR {
        BitSetbedokEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - setbkdok, bk queue ok"]
    #[inline(always)]
    pub fn bit_setbkdok_en(&self) -> BitSetbkdokEnR {
        BitSetbkdokEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - setmgdok, manage queue ok"]
    #[inline(always)]
    pub fn bit_setmgdok_en(&self) -> BitSetmgdokEnR {
        BitSetmgdokEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - seth2cdok, h2c queue ok"]
    #[inline(always)]
    pub fn bit_seth2cdok_en(&self) -> BitSeth2cdokEnR {
        BitSeth2cdokEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - sethighdok, high queue ok"]
    #[inline(always)]
    pub fn bit_sethighdok_en(&self) -> BitSethighdokEnR {
        BitSethighdokEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - setbcndok0, beacon queue ok, mbssid=0"]
    #[inline(always)]
    pub fn bit_setbcnok0_en(&self) -> BitSetbcnok0EnR {
        BitSetbcnok0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - setbcndok1, beacon queue ok, mbssid=1"]
    #[inline(always)]
    pub fn bit_setbcnok1_en(&self) -> BitSetbcnok1EnR {
        BitSetbcnok1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - setbcndok2, beacon queue ok, mbssid=2"]
    #[inline(always)]
    pub fn bit_setbcnok2_en(&self) -> BitSetbcnok2EnR {
        BitSetbcnok2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - setbcndok3, beacon queue ok, mbssid=3"]
    #[inline(always)]
    pub fn bit_setbcnok3_en(&self) -> BitSetbcnok3EnR {
        BitSetbcnok3EnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - setbcndok4, beacon queue ok, mbssid=4"]
    #[inline(always)]
    pub fn bit_setbcnok4_en(&self) -> BitSetbcnok4EnR {
        BitSetbcnok4EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - setbcndok5, beacon queue ok, mbssid=5"]
    #[inline(always)]
    pub fn bit_setbcnok5_en(&self) -> BitSetbcnok5EnR {
        BitSetbcnok5EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - setbcndok6, beacon queue ok, mbssid=6"]
    #[inline(always)]
    pub fn bit_setbcnok6_en(&self) -> BitSetbcnok6EnR {
        BitSetbcnok6EnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - setbcndok7, beacon queue ok, mbssid=7"]
    #[inline(always)]
    pub fn bit_setbcnok7_en(&self) -> BitSetbcnok7EnR {
        BitSetbcnok7EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - setbcnderr0, beacon queue error, mbssid=0"]
    #[inline(always)]
    pub fn bit_setbcnerr0_en(&self) -> BitSetbcnerr0EnR {
        BitSetbcnerr0EnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - setbcnderr1, beacon queue error, mbssid=1"]
    #[inline(always)]
    pub fn bit_setbcnerr1_en(&self) -> BitSetbcnerr1EnR {
        BitSetbcnerr1EnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - setbcnderr2, beacon queue error, mbssid=2"]
    #[inline(always)]
    pub fn bit_setbcnerr2_en(&self) -> BitSetbcnerr2EnR {
        BitSetbcnerr2EnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - setbcnderr3, beacon queue error, mbssid=3"]
    #[inline(always)]
    pub fn bit_setbcnerr3_en(&self) -> BitSetbcnerr3EnR {
        BitSetbcnerr3EnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - setbcnderr4, beacon queue error, mbssid=4"]
    #[inline(always)]
    pub fn bit_setbcnerr4_en(&self) -> BitSetbcnerr4EnR {
        BitSetbcnerr4EnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - setbcnderr5, beacon queue error, mbssid=5"]
    #[inline(always)]
    pub fn bit_setbcnerr5_en(&self) -> BitSetbcnerr5EnR {
        BitSetbcnerr5EnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - setbcnderr6, beacon queue error, mbssid=6"]
    #[inline(always)]
    pub fn bit_setbcnerr6_en(&self) -> BitSetbcnerr6EnR {
        BitSetbcnerr6EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - setbcnderr7, beacon queue error, mbssid=7"]
    #[inline(always)]
    pub fn bit_setbcnerr7_en(&self) -> BitSetbcnerr7EnR {
        BitSetbcnerr7EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - setrxok, rx dma ok"]
    #[inline(always)]
    pub fn bit_setrxok_en(&mut self) -> BitSetrxokEnW<'_, RegAxiInterruptMaskSpec> {
        BitSetrxokEnW::new(self, 0)
    }
    #[doc = "Bit 1 - set_rdu, rx descriptor unavailable"]
    #[inline(always)]
    pub fn bit_set_rdy_en(&mut self) -> BitSetRdyEnW<'_, RegAxiInterruptMaskSpec> {
        BitSetRdyEnW::new(self, 1)
    }
    #[doc = "Bit 2 - setvodok, vo queue ok"]
    #[inline(always)]
    pub fn bit_setvodok_en(&mut self) -> BitSetvodokEnW<'_, RegAxiInterruptMaskSpec> {
        BitSetvodokEnW::new(self, 2)
    }
    #[doc = "Bit 3 - setvidok, vi queue ok"]
    #[inline(always)]
    pub fn bit_setvidok_en(&mut self) -> BitSetvidokEnW<'_, RegAxiInterruptMaskSpec> {
        BitSetvidokEnW::new(self, 3)
    }
    #[doc = "Bit 4 - setbedok, be queue ok"]
    #[inline(always)]
    pub fn bit_setbedok_en(&mut self) -> BitSetbedokEnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbedokEnW::new(self, 4)
    }
    #[doc = "Bit 5 - setbkdok, bk queue ok"]
    #[inline(always)]
    pub fn bit_setbkdok_en(&mut self) -> BitSetbkdokEnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbkdokEnW::new(self, 5)
    }
    #[doc = "Bit 6 - setmgdok, manage queue ok"]
    #[inline(always)]
    pub fn bit_setmgdok_en(&mut self) -> BitSetmgdokEnW<'_, RegAxiInterruptMaskSpec> {
        BitSetmgdokEnW::new(self, 6)
    }
    #[doc = "Bit 7 - seth2cdok, h2c queue ok"]
    #[inline(always)]
    pub fn bit_seth2cdok_en(&mut self) -> BitSeth2cdokEnW<'_, RegAxiInterruptMaskSpec> {
        BitSeth2cdokEnW::new(self, 7)
    }
    #[doc = "Bit 8 - sethighdok, high queue ok"]
    #[inline(always)]
    pub fn bit_sethighdok_en(&mut self) -> BitSethighdokEnW<'_, RegAxiInterruptMaskSpec> {
        BitSethighdokEnW::new(self, 8)
    }
    #[doc = "Bit 16 - setbcndok0, beacon queue ok, mbssid=0"]
    #[inline(always)]
    pub fn bit_setbcnok0_en(&mut self) -> BitSetbcnok0EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnok0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - setbcndok1, beacon queue ok, mbssid=1"]
    #[inline(always)]
    pub fn bit_setbcnok1_en(&mut self) -> BitSetbcnok1EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnok1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - setbcndok2, beacon queue ok, mbssid=2"]
    #[inline(always)]
    pub fn bit_setbcnok2_en(&mut self) -> BitSetbcnok2EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnok2EnW::new(self, 18)
    }
    #[doc = "Bit 19 - setbcndok3, beacon queue ok, mbssid=3"]
    #[inline(always)]
    pub fn bit_setbcnok3_en(&mut self) -> BitSetbcnok3EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnok3EnW::new(self, 19)
    }
    #[doc = "Bit 20 - setbcndok4, beacon queue ok, mbssid=4"]
    #[inline(always)]
    pub fn bit_setbcnok4_en(&mut self) -> BitSetbcnok4EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnok4EnW::new(self, 20)
    }
    #[doc = "Bit 21 - setbcndok5, beacon queue ok, mbssid=5"]
    #[inline(always)]
    pub fn bit_setbcnok5_en(&mut self) -> BitSetbcnok5EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnok5EnW::new(self, 21)
    }
    #[doc = "Bit 22 - setbcndok6, beacon queue ok, mbssid=6"]
    #[inline(always)]
    pub fn bit_setbcnok6_en(&mut self) -> BitSetbcnok6EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnok6EnW::new(self, 22)
    }
    #[doc = "Bit 23 - setbcndok7, beacon queue ok, mbssid=7"]
    #[inline(always)]
    pub fn bit_setbcnok7_en(&mut self) -> BitSetbcnok7EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnok7EnW::new(self, 23)
    }
    #[doc = "Bit 24 - setbcnderr0, beacon queue error, mbssid=0"]
    #[inline(always)]
    pub fn bit_setbcnerr0_en(&mut self) -> BitSetbcnerr0EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnerr0EnW::new(self, 24)
    }
    #[doc = "Bit 25 - setbcnderr1, beacon queue error, mbssid=1"]
    #[inline(always)]
    pub fn bit_setbcnerr1_en(&mut self) -> BitSetbcnerr1EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnerr1EnW::new(self, 25)
    }
    #[doc = "Bit 26 - setbcnderr2, beacon queue error, mbssid=2"]
    #[inline(always)]
    pub fn bit_setbcnerr2_en(&mut self) -> BitSetbcnerr2EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnerr2EnW::new(self, 26)
    }
    #[doc = "Bit 27 - setbcnderr3, beacon queue error, mbssid=3"]
    #[inline(always)]
    pub fn bit_setbcnerr3_en(&mut self) -> BitSetbcnerr3EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnerr3EnW::new(self, 27)
    }
    #[doc = "Bit 28 - setbcnderr4, beacon queue error, mbssid=4"]
    #[inline(always)]
    pub fn bit_setbcnerr4_en(&mut self) -> BitSetbcnerr4EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnerr4EnW::new(self, 28)
    }
    #[doc = "Bit 29 - setbcnderr5, beacon queue error, mbssid=5"]
    #[inline(always)]
    pub fn bit_setbcnerr5_en(&mut self) -> BitSetbcnerr5EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnerr5EnW::new(self, 29)
    }
    #[doc = "Bit 30 - setbcnderr6, beacon queue error, mbssid=6"]
    #[inline(always)]
    pub fn bit_setbcnerr6_en(&mut self) -> BitSetbcnerr6EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnerr6EnW::new(self, 30)
    }
    #[doc = "Bit 31 - setbcnderr7, beacon queue error, mbssid=7"]
    #[inline(always)]
    pub fn bit_setbcnerr7_en(&mut self) -> BitSetbcnerr7EnW<'_, RegAxiInterruptMaskSpec> {
        BitSetbcnerr7EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_axi_interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_axi_interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxiInterruptMaskSpec;
impl crate::RegisterSpec for RegAxiInterruptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_axi_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for RegAxiInterruptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_axi_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for RegAxiInterruptMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AXI_INTERRUPT_MASK to value 0"]
impl crate::Resettable for RegAxiInterruptMaskSpec {}
