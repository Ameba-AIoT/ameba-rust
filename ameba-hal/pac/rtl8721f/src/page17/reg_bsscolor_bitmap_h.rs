#[doc = "Register `REG_BSSCOLOR_BITMAP_H` reader"]
pub type R = crate::R<RegBsscolorBitmapHSpec>;
#[doc = "Register `REG_BSSCOLOR_BITMAP_H` writer"]
pub type W = crate::W<RegBsscolorBitmapHSpec>;
#[doc = "Field `r_bsscolor_bitmap_h` reader - high 32bits bss color bitmap"]
pub type RBsscolorBitmapHR = crate::FieldReader<u32>;
#[doc = "Field `r_bsscolor_bitmap_h` writer - high 32bits bss color bitmap"]
pub type RBsscolorBitmapHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high 32bits bss color bitmap"]
    #[inline(always)]
    pub fn r_bsscolor_bitmap_h(&self) -> RBsscolorBitmapHR {
        RBsscolorBitmapHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high 32bits bss color bitmap"]
    #[inline(always)]
    pub fn r_bsscolor_bitmap_h(&mut self) -> RBsscolorBitmapHW<'_, RegBsscolorBitmapHSpec> {
        RBsscolorBitmapHW::new(self, 0)
    }
}
#[doc = "bss color bitmap register_h\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bsscolor_bitmap_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bsscolor_bitmap_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBsscolorBitmapHSpec;
impl crate::RegisterSpec for RegBsscolorBitmapHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bsscolor_bitmap_h::R`](R) reader structure"]
impl crate::Readable for RegBsscolorBitmapHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bsscolor_bitmap_h::W`](W) writer structure"]
impl crate::Writable for RegBsscolorBitmapHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BSSCOLOR_BITMAP_H to value 0"]
impl crate::Resettable for RegBsscolorBitmapHSpec {}
