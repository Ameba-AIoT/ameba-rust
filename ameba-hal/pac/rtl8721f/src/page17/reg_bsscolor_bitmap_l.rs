#[doc = "Register `REG_BSSCOLOR_BITMAP_L` reader"]
pub type R = crate::R<RegBsscolorBitmapLSpec>;
#[doc = "Register `REG_BSSCOLOR_BITMAP_L` writer"]
pub type W = crate::W<RegBsscolorBitmapLSpec>;
#[doc = "Field `r_bsscolor_bitmap_l` reader - low 32bits bss color bitmap"]
pub type RBsscolorBitmapLR = crate::FieldReader<u32>;
#[doc = "Field `r_bsscolor_bitmap_l` writer - low 32bits bss color bitmap"]
pub type RBsscolorBitmapLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bits bss color bitmap"]
    #[inline(always)]
    pub fn r_bsscolor_bitmap_l(&self) -> RBsscolorBitmapLR {
        RBsscolorBitmapLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32bits bss color bitmap"]
    #[inline(always)]
    pub fn r_bsscolor_bitmap_l(&mut self) -> RBsscolorBitmapLW<'_, RegBsscolorBitmapLSpec> {
        RBsscolorBitmapLW::new(self, 0)
    }
}
#[doc = "bss color bitmap register_l\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bsscolor_bitmap_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bsscolor_bitmap_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBsscolorBitmapLSpec;
impl crate::RegisterSpec for RegBsscolorBitmapLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bsscolor_bitmap_l::R`](R) reader structure"]
impl crate::Readable for RegBsscolorBitmapLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bsscolor_bitmap_l::W`](W) writer structure"]
impl crate::Writable for RegBsscolorBitmapLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSCOLOR_BITMAP_L to value 0"]
impl crate::Resettable for RegBsscolorBitmapLSpec {}
