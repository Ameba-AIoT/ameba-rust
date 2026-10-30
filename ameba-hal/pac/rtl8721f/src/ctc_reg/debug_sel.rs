#[doc = "Register `DEBUG_SEL` reader"]
pub type R = crate::R<DebugSelSpec>;
#[doc = "Register `DEBUG_SEL` writer"]
pub type W = crate::W<DebugSelSpec>;
#[doc = "Field `dbg_sel` reader - Debug information selection 0: dbg_clkrst 1: dbg_apbslv 2: dbg_ctrl 3: dbg_decision 4: dbg_adc_arb Others: dbg_clkrst"]
pub type DbgSelR = crate::FieldReader;
#[doc = "Field `dbg_sel` writer - Debug information selection 0: dbg_clkrst 1: dbg_apbslv 2: dbg_ctrl 3: dbg_decision 4: dbg_adc_arb Others: dbg_clkrst"]
pub type DbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Debug information selection 0: dbg_clkrst 1: dbg_apbslv 2: dbg_ctrl 3: dbg_decision 4: dbg_adc_arb Others: dbg_clkrst"]
    #[inline(always)]
    pub fn dbg_sel(&self) -> DbgSelR {
        DbgSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Debug information selection 0: dbg_clkrst 1: dbg_apbslv 2: dbg_ctrl 3: dbg_decision 4: dbg_adc_arb Others: dbg_clkrst"]
    #[inline(always)]
    pub fn dbg_sel(&mut self) -> DbgSelW<'_, DebugSelSpec> {
        DbgSelW::new(self, 0)
    }
}
#[doc = "Debug Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSelSpec;
impl crate::RegisterSpec for DebugSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_sel::R`](R) reader structure"]
impl crate::Readable for DebugSelSpec {}
#[doc = "`write(|w| ..)` method takes [`debug_sel::W`](W) writer structure"]
impl crate::Writable for DebugSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_SEL to value 0"]
impl crate::Resettable for DebugSelSpec {}
