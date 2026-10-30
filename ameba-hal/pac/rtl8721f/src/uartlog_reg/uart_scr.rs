#[doc = "Register `UART_SCR` reader"]
pub type R = crate::R<UartScrSpec>;
#[doc = "Register `UART_SCR` writer"]
pub type W = crate::W<UartScrSpec>;
#[doc = "Field `scratch_6_0` reader - *Bit\\[6\\]: Rx break signal interrupt enable *Bit\\[5\\]: fl_set_bi_err *Bit\\[4\\]: fl_frame_err *Bit\\[3\\]: pin_lb_test *Bit\\[2:0\\]: reserved"]
pub type Scratch6_0R = crate::FieldReader;
#[doc = "Field `scratch_6_0` writer - *Bit\\[6\\]: Rx break signal interrupt enable *Bit\\[5\\]: fl_set_bi_err *Bit\\[4\\]: fl_frame_err *Bit\\[3\\]: pin_lb_test *Bit\\[2:0\\]: reserved"]
pub type Scratch6_0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `scratch_7` reader - clear Rx break signal interrupt status"]
pub type Scratch7R = crate::BitReader;
#[doc = "Field `scratch_7` writer - clear Rx break signal interrupt status"]
pub type Scratch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbg_sel` reader - Debug port selection"]
pub type DbgSelR = crate::FieldReader;
#[doc = "Field `dbg_sel` writer - Debug port selection"]
pub type DbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `xfactor_adj` reader - One factor of Baud rate calculation for Tx path"]
pub type XfactorAdjR = crate::FieldReader<u16>;
#[doc = "Field `xfactor_adj` writer - One factor of Baud rate calculation for Tx path"]
pub type XfactorAdjW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:6 - *Bit\\[6\\]: Rx break signal interrupt enable *Bit\\[5\\]: fl_set_bi_err *Bit\\[4\\]: fl_frame_err *Bit\\[3\\]: pin_lb_test *Bit\\[2:0\\]: reserved"]
    #[inline(always)]
    pub fn scratch_6_0(&self) -> Scratch6_0R {
        Scratch6_0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clear Rx break signal interrupt status"]
    #[inline(always)]
    pub fn scratch_7(&self) -> Scratch7R {
        Scratch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Debug port selection"]
    #[inline(always)]
    pub fn dbg_sel(&self) -> DbgSelR {
        DbgSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - One factor of Baud rate calculation for Tx path"]
    #[inline(always)]
    pub fn xfactor_adj(&self) -> XfactorAdjR {
        XfactorAdjR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - *Bit\\[6\\]: Rx break signal interrupt enable *Bit\\[5\\]: fl_set_bi_err *Bit\\[4\\]: fl_frame_err *Bit\\[3\\]: pin_lb_test *Bit\\[2:0\\]: reserved"]
    #[inline(always)]
    pub fn scratch_6_0(&mut self) -> Scratch6_0W<'_, UartScrSpec> {
        Scratch6_0W::new(self, 0)
    }
    #[doc = "Bit 7 - clear Rx break signal interrupt status"]
    #[inline(always)]
    pub fn scratch_7(&mut self) -> Scratch7W<'_, UartScrSpec> {
        Scratch7W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Debug port selection"]
    #[inline(always)]
    pub fn dbg_sel(&mut self) -> DbgSelW<'_, UartScrSpec> {
        DbgSelW::new(self, 8)
    }
    #[doc = "Bits 16:26 - One factor of Baud rate calculation for Tx path"]
    #[inline(always)]
    pub fn xfactor_adj(&mut self) -> XfactorAdjW<'_, UartScrSpec> {
        XfactorAdjW::new(self, 16)
    }
}
#[doc = "UART scratch pad register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartScrSpec;
impl crate::RegisterSpec for UartScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_scr::R`](R) reader structure"]
impl crate::Readable for UartScrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_scr::W`](W) writer structure"]
impl crate::Writable for UartScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_SCR to value 0x30"]
impl crate::Resettable for UartScrSpec {
    const RESET_VALUE: u32 = 0x30;
}
