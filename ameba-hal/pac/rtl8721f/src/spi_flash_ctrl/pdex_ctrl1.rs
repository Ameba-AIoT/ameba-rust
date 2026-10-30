#[doc = "Register `PDEX_CTRL1` reader"]
pub type R = crate::R<PdexCtrl1Spec>;
#[doc = "Register `PDEX_CTRL1` writer"]
pub type W = crate::W<PdexCtrl1Spec>;
#[doc = "Field `PDE_CMD1` reader - Indicate the \\[7-4\\] byte of SPI PDE command value. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeCmd1R = crate::FieldReader<u32>;
#[doc = "Field `PDE_CMD1` writer - Indicate the \\[7-4\\] byte of SPI PDE command value. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeCmd1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicate the \\[7-4\\] byte of SPI PDE command value. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_cmd1(&self) -> PdeCmd1R {
        PdeCmd1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicate the \\[7-4\\] byte of SPI PDE command value. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_cmd1(&mut self) -> PdeCmd1W<'_, PdexCtrl1Spec> {
        PdeCmd1W::new(self, 0)
    }
}
#[doc = "PDEX Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdex_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdex_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdexCtrl1Spec;
impl crate::RegisterSpec for PdexCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdex_ctrl1::R`](R) reader structure"]
impl crate::Readable for PdexCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdex_ctrl1::W`](W) writer structure"]
impl crate::Writable for PdexCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDEX_CTRL1 to value 0"]
impl crate::Resettable for PdexCtrl1Spec {}
