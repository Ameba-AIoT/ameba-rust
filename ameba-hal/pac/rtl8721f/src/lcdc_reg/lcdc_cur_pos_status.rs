#[doc = "Register `LCDC_CUR_POS_STATUS` reader"]
pub type R = crate::R<LcdcCurPosStatusSpec>;
#[doc = "Field `cur_pos_x` reader - Current X Position These bits return the current X position."]
pub type CurPosXR = crate::FieldReader<u16>;
#[doc = "Field `cur_pos_y` reader - Current Y Position These bits return the current Y position."]
pub type CurPosYR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Current X Position These bits return the current X position."]
    #[inline(always)]
    pub fn cur_pos_x(&self) -> CurPosXR {
        CurPosXR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Current Y Position These bits return the current Y position."]
    #[inline(always)]
    pub fn cur_pos_y(&self) -> CurPosYR {
        CurPosYR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "Display current position\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_cur_pos_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcCurPosStatusSpec;
impl crate::RegisterSpec for LcdcCurPosStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_cur_pos_status::R`](R) reader structure"]
impl crate::Readable for LcdcCurPosStatusSpec {}
#[doc = "`reset()` method sets LCDC_CUR_POS_STATUS to value 0x0001_0001"]
impl crate::Resettable for LcdcCurPosStatusSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
