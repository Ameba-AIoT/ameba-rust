#[doc = "Register `PDEX_CTRL0` reader"]
pub type R = crate::R<PdexCtrl0Spec>;
#[doc = "Register `PDEX_CTRL0` writer"]
pub type W = crate::W<PdexCtrl0Spec>;
#[doc = "Field `PDE_CMD0` reader - Indicate the \\[3-0\\] byte of SPI PDE command value. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeCmd0R = crate::FieldReader<u32>;
#[doc = "Field `PDE_CMD0` writer - Indicate the \\[3-0\\] byte of SPI PDE command value. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeCmd0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicate the \\[3-0\\] byte of SPI PDE command value. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_cmd0(&self) -> PdeCmd0R {
        PdeCmd0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicate the \\[3-0\\] byte of SPI PDE command value. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_cmd0(&mut self) -> PdeCmd0W<'_, PdexCtrl0Spec> {
        PdeCmd0W::new(self, 0)
    }
}
#[doc = "PDEX Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdex_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdex_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdexCtrl0Spec;
impl crate::RegisterSpec for PdexCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdex_ctrl0::R`](R) reader structure"]
impl crate::Readable for PdexCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdex_ctrl0::W`](W) writer structure"]
impl crate::Writable for PdexCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDEX_CTRL0 to value 0"]
impl crate::Resettable for PdexCtrl0Spec {}
