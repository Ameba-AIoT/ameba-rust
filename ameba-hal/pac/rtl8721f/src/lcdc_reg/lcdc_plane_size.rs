#[doc = "Register `LCDC_PLANE_SIZE` reader"]
pub type R = crate::R<LcdcPlaneSizeSpec>;
#[doc = "Register `LCDC_PLANE_SIZE` writer"]
pub type W = crate::W<LcdcPlaneSizeSpec>;
#[doc = "Field `imagewidth` reader - The width of panel (X-channel based), the min. value is 0x1. *For LCD, it means pixel number per line."]
pub type ImagewidthR = crate::FieldReader<u16>;
#[doc = "Field `imagewidth` writer - The width of panel (X-channel based), the min. value is 0x1. *For LCD, it means pixel number per line."]
pub type ImagewidthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `imageheight` reader - The height of panel (Y-channel based), the min. value is 0x1."]
pub type ImageheightR = crate::FieldReader<u16>;
#[doc = "Field `imageheight` writer - The height of panel (Y-channel based), the min. value is 0x1."]
pub type ImageheightW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The width of panel (X-channel based), the min. value is 0x1. *For LCD, it means pixel number per line."]
    #[inline(always)]
    pub fn imagewidth(&self) -> ImagewidthR {
        ImagewidthR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - The height of panel (Y-channel based), the min. value is 0x1."]
    #[inline(always)]
    pub fn imageheight(&self) -> ImageheightR {
        ImageheightR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The width of panel (X-channel based), the min. value is 0x1. *For LCD, it means pixel number per line."]
    #[inline(always)]
    pub fn imagewidth(&mut self) -> ImagewidthW<'_, LcdcPlaneSizeSpec> {
        ImagewidthW::new(self, 0)
    }
    #[doc = "Bits 16:27 - The height of panel (Y-channel based), the min. value is 0x1."]
    #[inline(always)]
    pub fn imageheight(&mut self) -> ImageheightW<'_, LcdcPlaneSizeSpec> {
        ImageheightW::new(self, 16)
    }
}
#[doc = "LCDC plane size register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_plane_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_plane_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcPlaneSizeSpec;
impl crate::RegisterSpec for LcdcPlaneSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_plane_size::R`](R) reader structure"]
impl crate::Readable for LcdcPlaneSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_plane_size::W`](W) writer structure"]
impl crate::Writable for LcdcPlaneSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_PLANE_SIZE to value 0x0140_00f0"]
impl crate::Resettable for LcdcPlaneSizeSpec {
    const RESET_VALUE: u32 = 0x0140_00f0;
}
