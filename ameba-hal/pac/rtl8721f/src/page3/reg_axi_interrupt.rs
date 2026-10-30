#[doc = "Register `REG_AXI_INTERRUPT` reader"]
pub type R = crate::R<RegAxiInterruptSpec>;
#[doc = "Field `BIT_SETRXOK` reader - setrxok, rx dma ok"]
pub type BitSetrxokR = crate::BitReader;
#[doc = "Field `BIT_SET_RDY` reader - set_rdu, rx descriptor unavailable"]
pub type BitSetRdyR = crate::BitReader;
#[doc = "Field `BIT_SETVODOK` reader - setvodok, vo queue ok"]
pub type BitSetvodokR = crate::BitReader;
#[doc = "Field `BIT_SETVIDOK` reader - setvidok, vi queue ok"]
pub type BitSetvidokR = crate::BitReader;
#[doc = "Field `BIT_SETBEDOK` reader - setbedok, be queue ok"]
pub type BitSetbedokR = crate::BitReader;
#[doc = "Field `BIT_SETBKDOK` reader - setbkdok, bk queue ok"]
pub type BitSetbkdokR = crate::BitReader;
#[doc = "Field `BIT_SETMGDOK` reader - setmgdok, manage queue ok"]
pub type BitSetmgdokR = crate::BitReader;
#[doc = "Field `BIT_SETH2CDOK` reader - seth2cdok, h2c queue ok"]
pub type BitSeth2cdokR = crate::BitReader;
#[doc = "Field `BIT_SETHIGHDOK` reader - sethighdok, high queue ok"]
pub type BitSethighdokR = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK0` reader - setbcndok0, beacon queue ok, mbssid=0"]
pub type BitSetbcnok0R = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK1` reader - setbcndok1, beacon queue ok, mbssid=1"]
pub type BitSetbcnok1R = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK2` reader - setbcndok2, beacon queue ok, mbssid=2"]
pub type BitSetbcnok2R = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK3` reader - setbcndok3, beacon queue ok, mbssid=3"]
pub type BitSetbcnok3R = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK4` reader - setbcndok4, beacon queue ok, mbssid=4"]
pub type BitSetbcnok4R = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK5` reader - setbcndok5, beacon queue ok, mbssid=5"]
pub type BitSetbcnok5R = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK6` reader - setbcndok6, beacon queue ok, mbssid=6"]
pub type BitSetbcnok6R = crate::BitReader;
#[doc = "Field `BIT_SETBCNOK7` reader - setbcndok7, beacon queue ok, mbssid=7"]
pub type BitSetbcnok7R = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR0` reader - setbcnderr0, beacon queue error, mbssid=0"]
pub type BitSetbcnerr0R = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR1` reader - setbcnderr1, beacon queue error, mbssid=1"]
pub type BitSetbcnerr1R = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR2` reader - setbcnderr2, beacon queue error, mbssid=2"]
pub type BitSetbcnerr2R = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR3` reader - setbcnderr3, beacon queue error, mbssid=3"]
pub type BitSetbcnerr3R = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR4` reader - setbcnderr4, beacon queue error, mbssid=4"]
pub type BitSetbcnerr4R = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR5` reader - setbcnderr5, beacon queue error, mbssid=5"]
pub type BitSetbcnerr5R = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR6` reader - setbcnderr6, beacon queue error, mbssid=6"]
pub type BitSetbcnerr6R = crate::BitReader;
#[doc = "Field `BIT_SETBCNERR7` reader - setbcnderr7, beacon queue error, mbssid=7"]
pub type BitSetbcnerr7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - setrxok, rx dma ok"]
    #[inline(always)]
    pub fn bit_setrxok(&self) -> BitSetrxokR {
        BitSetrxokR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set_rdu, rx descriptor unavailable"]
    #[inline(always)]
    pub fn bit_set_rdy(&self) -> BitSetRdyR {
        BitSetRdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - setvodok, vo queue ok"]
    #[inline(always)]
    pub fn bit_setvodok(&self) -> BitSetvodokR {
        BitSetvodokR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - setvidok, vi queue ok"]
    #[inline(always)]
    pub fn bit_setvidok(&self) -> BitSetvidokR {
        BitSetvidokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - setbedok, be queue ok"]
    #[inline(always)]
    pub fn bit_setbedok(&self) -> BitSetbedokR {
        BitSetbedokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - setbkdok, bk queue ok"]
    #[inline(always)]
    pub fn bit_setbkdok(&self) -> BitSetbkdokR {
        BitSetbkdokR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - setmgdok, manage queue ok"]
    #[inline(always)]
    pub fn bit_setmgdok(&self) -> BitSetmgdokR {
        BitSetmgdokR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - seth2cdok, h2c queue ok"]
    #[inline(always)]
    pub fn bit_seth2cdok(&self) -> BitSeth2cdokR {
        BitSeth2cdokR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - sethighdok, high queue ok"]
    #[inline(always)]
    pub fn bit_sethighdok(&self) -> BitSethighdokR {
        BitSethighdokR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - setbcndok0, beacon queue ok, mbssid=0"]
    #[inline(always)]
    pub fn bit_setbcnok0(&self) -> BitSetbcnok0R {
        BitSetbcnok0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - setbcndok1, beacon queue ok, mbssid=1"]
    #[inline(always)]
    pub fn bit_setbcnok1(&self) -> BitSetbcnok1R {
        BitSetbcnok1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - setbcndok2, beacon queue ok, mbssid=2"]
    #[inline(always)]
    pub fn bit_setbcnok2(&self) -> BitSetbcnok2R {
        BitSetbcnok2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - setbcndok3, beacon queue ok, mbssid=3"]
    #[inline(always)]
    pub fn bit_setbcnok3(&self) -> BitSetbcnok3R {
        BitSetbcnok3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - setbcndok4, beacon queue ok, mbssid=4"]
    #[inline(always)]
    pub fn bit_setbcnok4(&self) -> BitSetbcnok4R {
        BitSetbcnok4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - setbcndok5, beacon queue ok, mbssid=5"]
    #[inline(always)]
    pub fn bit_setbcnok5(&self) -> BitSetbcnok5R {
        BitSetbcnok5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - setbcndok6, beacon queue ok, mbssid=6"]
    #[inline(always)]
    pub fn bit_setbcnok6(&self) -> BitSetbcnok6R {
        BitSetbcnok6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - setbcndok7, beacon queue ok, mbssid=7"]
    #[inline(always)]
    pub fn bit_setbcnok7(&self) -> BitSetbcnok7R {
        BitSetbcnok7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - setbcnderr0, beacon queue error, mbssid=0"]
    #[inline(always)]
    pub fn bit_setbcnerr0(&self) -> BitSetbcnerr0R {
        BitSetbcnerr0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - setbcnderr1, beacon queue error, mbssid=1"]
    #[inline(always)]
    pub fn bit_setbcnerr1(&self) -> BitSetbcnerr1R {
        BitSetbcnerr1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - setbcnderr2, beacon queue error, mbssid=2"]
    #[inline(always)]
    pub fn bit_setbcnerr2(&self) -> BitSetbcnerr2R {
        BitSetbcnerr2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - setbcnderr3, beacon queue error, mbssid=3"]
    #[inline(always)]
    pub fn bit_setbcnerr3(&self) -> BitSetbcnerr3R {
        BitSetbcnerr3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - setbcnderr4, beacon queue error, mbssid=4"]
    #[inline(always)]
    pub fn bit_setbcnerr4(&self) -> BitSetbcnerr4R {
        BitSetbcnerr4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - setbcnderr5, beacon queue error, mbssid=5"]
    #[inline(always)]
    pub fn bit_setbcnerr5(&self) -> BitSetbcnerr5R {
        BitSetbcnerr5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - setbcnderr6, beacon queue error, mbssid=6"]
    #[inline(always)]
    pub fn bit_setbcnerr6(&self) -> BitSetbcnerr6R {
        BitSetbcnerr6R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - setbcnderr7, beacon queue error, mbssid=7"]
    #[inline(always)]
    pub fn bit_setbcnerr7(&self) -> BitSetbcnerr7R {
        BitSetbcnerr7R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_axi_interrupt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxiInterruptSpec;
impl crate::RegisterSpec for RegAxiInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_axi_interrupt::R`](R) reader structure"]
impl crate::Readable for RegAxiInterruptSpec {}
#[doc = "`reset()` method sets REG_AXI_INTERRUPT to value 0"]
impl crate::Resettable for RegAxiInterruptSpec {}
