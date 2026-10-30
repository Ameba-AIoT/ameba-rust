#[doc = "Register `LCDC_IMG_B_BASE_ADDR` reader"]
pub type R = crate::R<LcdcImgBBaseAddrSpec>;
#[doc = "Register `LCDC_IMG_B_BASE_ADDR` writer"]
pub type W = crate::W<LcdcImgBBaseAddrSpec>;
#[doc = "Field `img_b_base_addr` reader - Image layer DMA source address. After a frame refresh done, hardware loads the newer base address from this register automatically."]
pub type ImgBBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `img_b_base_addr` writer - Image layer DMA source address. After a frame refresh done, hardware loads the newer base address from this register automatically."]
pub type ImgBBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Image layer DMA source address. After a frame refresh done, hardware loads the newer base address from this register automatically."]
    #[inline(always)]
    pub fn img_b_base_addr(&self) -> ImgBBaseAddrR {
        ImgBBaseAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Image layer DMA source address. After a frame refresh done, hardware loads the newer base address from this register automatically."]
    #[inline(always)]
    pub fn img_b_base_addr(&mut self) -> ImgBBaseAddrW<'_, LcdcImgBBaseAddrSpec> {
        ImgBBaseAddrW::new(self, 0)
    }
}
#[doc = "LCDC image B base address\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_img_b_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_img_b_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcImgBBaseAddrSpec;
impl crate::RegisterSpec for LcdcImgBBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_img_b_base_addr::R`](R) reader structure"]
impl crate::Readable for LcdcImgBBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_img_b_base_addr::W`](W) writer structure"]
impl crate::Writable for LcdcImgBBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_IMG_B_BASE_ADDR to value 0"]
impl crate::Resettable for LcdcImgBBaseAddrSpec {}
