#[doc = "Register `LINE_INDEX` reader"]
pub type R = crate::R<LineIndexSpec>;
#[doc = "Register `LINE_INDEX` writer"]
pub type W = crate::W<LineIndexSpec>;
#[doc = "Field `LINE_INDEX` reader - Line index for line over interrupt LINE_OVER interrupt is raised when the transaction of last pixel of this index marked line has been completed."]
pub type LineIndexR = crate::FieldReader<u16>;
#[doc = "Field `LINE_INDEX` writer - Line index for line over interrupt LINE_OVER interrupt is raised when the transaction of last pixel of this index marked line has been completed."]
pub type LineIndexW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Line index for line over interrupt LINE_OVER interrupt is raised when the transaction of last pixel of this index marked line has been completed."]
    #[inline(always)]
    pub fn line_index(&self) -> LineIndexR {
        LineIndexR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Line index for line over interrupt LINE_OVER interrupt is raised when the transaction of last pixel of this index marked line has been completed."]
    #[inline(always)]
    pub fn line_index(&mut self) -> LineIndexW<'_, LineIndexSpec> {
        LineIndexW::new(self, 0)
    }
}
#[doc = "Line index register\n\nYou can [`read`](crate::Reg::read) this register and get [`line_index::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`line_index::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LineIndexSpec;
impl crate::RegisterSpec for LineIndexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`line_index::R`](R) reader structure"]
impl crate::Readable for LineIndexSpec {}
#[doc = "`write(|w| ..)` method takes [`line_index::W`](W) writer structure"]
impl crate::Writable for LineIndexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINE_INDEX to value 0x07ff"]
impl crate::Resettable for LineIndexSpec {
    const RESET_VALUE: u32 = 0x07ff;
}
