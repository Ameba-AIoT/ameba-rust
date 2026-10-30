#[doc = "Register `GLB_STATUS` reader"]
pub type R = crate::R<GlbStatusSpec>;
#[doc = "Register `GLB_STATUS` writer"]
pub type W = crate::W<GlbStatusSpec>;
#[doc = "Field `RUN_STATUS` reader - Run status 0x0: Disabled 0x1: Runing 0x2: Inactive (w/o outstanding) 0x3: Suspend (w/ outstanding) Others: reserved"]
pub type RunStatusR = crate::FieldReader;
#[doc = "Field `RUN_STATUS` writer - Run status 0x0: Disabled 0x1: Runing 0x2: Inactive (w/o outstanding) 0x3: Suspend (w/ outstanding) Others: reserved"]
pub type RunStatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CHN_INACT` reader - Indicates whether the internal DMA channels are inactive (protocol over). Bit 15: 0x0: There are currently outstanding transactions for AWID=0 0x1: There are currently no outstanding transactions for AWID=0 Bit i (16 <= i <= 31): 0x0: There are currently outstanding transactions for ARID=i-16 0x1: There are currently no outstanding transactions for ARID=i-16"]
pub type ChnInactR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Run status 0x0: Disabled 0x1: Runing 0x2: Inactive (w/o outstanding) 0x3: Suspend (w/ outstanding) Others: reserved"]
    #[inline(always)]
    pub fn run_status(&self) -> RunStatusR {
        RunStatusR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 15:31 - Indicates whether the internal DMA channels are inactive (protocol over). Bit 15: 0x0: There are currently outstanding transactions for AWID=0 0x1: There are currently no outstanding transactions for AWID=0 Bit i (16 <= i <= 31): 0x0: There are currently outstanding transactions for ARID=i-16 0x1: There are currently no outstanding transactions for ARID=i-16"]
    #[inline(always)]
    pub fn chn_inact(&self) -> ChnInactR {
        ChnInactR::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - Run status 0x0: Disabled 0x1: Runing 0x2: Inactive (w/o outstanding) 0x3: Suspend (w/ outstanding) Others: reserved"]
    #[inline(always)]
    pub fn run_status(&mut self) -> RunStatusW<'_, GlbStatusSpec> {
        RunStatusW::new(self, 0)
    }
}
#[doc = "Global status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glb_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbStatusSpec;
impl crate::RegisterSpec for GlbStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_status::R`](R) reader structure"]
impl crate::Readable for GlbStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`glb_status::W`](W) writer structure"]
impl crate::Writable for GlbStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GLB_STATUS to value 0x07"]
impl crate::Resettable for GlbStatusSpec {
    const RESET_VALUE: u32 = 0x07;
}
