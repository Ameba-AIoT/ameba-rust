#[doc = "Register `TBAR` reader"]
pub type R = crate::R<TbarSpec>;
#[doc = "Register `TBAR` writer"]
pub type W = crate::W<TbarSpec>;
#[doc = "Field `tag_base_0` reader - Tag base address for reference to the flash SEC engine. Not equal to actual tag address, need to be calculated. tag_base = tag_addr - ( (gcm_region_addr / number_of_byte_per_line) * number_of_byte_per_tag ) Please refer to the example on the GCM Tag address page"]
pub type TagBase0R = crate::FieldReader<u32>;
#[doc = "Field `tag_base_0` writer - Tag base address for reference to the flash SEC engine. Not equal to actual tag address, need to be calculated. tag_base = tag_addr - ( (gcm_region_addr / number_of_byte_per_line) * number_of_byte_per_tag ) Please refer to the example on the GCM Tag address page"]
pub type TagBase0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tag base address for reference to the flash SEC engine. Not equal to actual tag address, need to be calculated. tag_base = tag_addr - ( (gcm_region_addr / number_of_byte_per_line) * number_of_byte_per_tag ) Please refer to the example on the GCM Tag address page"]
    #[inline(always)]
    pub fn tag_base_0(&self) -> TagBase0R {
        TagBase0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tag base address for reference to the flash SEC engine. Not equal to actual tag address, need to be calculated. tag_base = tag_addr - ( (gcm_region_addr / number_of_byte_per_line) * number_of_byte_per_tag ) Please refer to the example on the GCM Tag address page"]
    #[inline(always)]
    pub fn tag_base_0(&mut self) -> TagBase0W<'_, TbarSpec> {
        TagBase0W::new(self, 0)
    }
}
#[doc = "GCM Tag Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbarSpec;
impl crate::RegisterSpec for TbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbar::R`](R) reader structure"]
impl crate::Readable for TbarSpec {}
#[doc = "`write(|w| ..)` method takes [`tbar::W`](W) writer structure"]
impl crate::Writable for TbarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBAR to value 0"]
impl crate::Resettable for TbarSpec {}
