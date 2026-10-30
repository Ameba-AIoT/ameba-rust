#[doc = "Register `IC_DEBUG_SEL` reader"]
pub type R = crate::R<IcDebugSelSpec>;
#[doc = "Register `IC_DEBUG_SEL` writer"]
pub type W = crate::W<IcDebugSelSpec>;
#[doc = "Field `IC_DEBUG_SEL` reader - * 0: Debug clock * 1: Debug APB * 2: RSVD * 3: Debug FIFO * 4: Debug Timing * 5: Debug Slave Mode Address Match * 6: Debug Interrupt"]
pub type IcDebugSelR = crate::FieldReader;
#[doc = "Field `IC_DEBUG_SEL` writer - * 0: Debug clock * 1: Debug APB * 2: RSVD * 3: Debug FIFO * 4: Debug Timing * 5: Debug Slave Mode Address Match * 6: Debug Interrupt"]
pub type IcDebugSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - * 0: Debug clock * 1: Debug APB * 2: RSVD * 3: Debug FIFO * 4: Debug Timing * 5: Debug Slave Mode Address Match * 6: Debug Interrupt"]
    #[inline(always)]
    pub fn ic_debug_sel(&self) -> IcDebugSelR {
        IcDebugSelR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - * 0: Debug clock * 1: Debug APB * 2: RSVD * 3: Debug FIFO * 4: Debug Timing * 5: Debug Slave Mode Address Match * 6: Debug Interrupt"]
    #[inline(always)]
    pub fn ic_debug_sel(&mut self) -> IcDebugSelW<'_, IcDebugSelSpec> {
        IcDebugSelW::new(self, 0)
    }
}
#[doc = "I2C Debug SEL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_debug_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_debug_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDebugSelSpec;
impl crate::RegisterSpec for IcDebugSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_debug_sel::R`](R) reader structure"]
impl crate::Readable for IcDebugSelSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_debug_sel::W`](W) writer structure"]
impl crate::Writable for IcDebugSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_DEBUG_SEL to value 0"]
impl crate::Resettable for IcDebugSelSpec {}
