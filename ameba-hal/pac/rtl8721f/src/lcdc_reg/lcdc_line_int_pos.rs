#[doc = "Register `LCDC_LINE_INT_POS` reader"]
pub type R = crate::R<LcdcLineIntPosSpec>;
#[doc = "Register `LCDC_LINE_INT_POS` writer"]
pub type W = crate::W<LcdcLineIntPosSpec>;
#[doc = "Field `line_int_pos` reader - Line Interrupt Position These bits configure the line interrupt position."]
pub type LineIntPosR = crate::FieldReader<u16>;
#[doc = "Field `line_int_pos` writer - Line Interrupt Position These bits configure the line interrupt position."]
pub type LineIntPosW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Line Interrupt Position These bits configure the line interrupt position."]
    #[inline(always)]
    pub fn line_int_pos(&self) -> LineIntPosR {
        LineIntPosR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Line Interrupt Position These bits configure the line interrupt position."]
    #[inline(always)]
    pub fn line_int_pos(&mut self) -> LineIntPosW<'_, LcdcLineIntPosSpec> {
        LineIntPosW::new(self, 0)
    }
}
#[doc = "LCDC line interrupt position register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_line_int_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_line_int_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcLineIntPosSpec;
impl crate::RegisterSpec for LcdcLineIntPosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_line_int_pos::R`](R) reader structure"]
impl crate::Readable for LcdcLineIntPosSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_line_int_pos::W`](W) writer structure"]
impl crate::Writable for LcdcLineIntPosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_LINE_INT_POS to value 0"]
impl crate::Resettable for LcdcLineIntPosSpec {}
