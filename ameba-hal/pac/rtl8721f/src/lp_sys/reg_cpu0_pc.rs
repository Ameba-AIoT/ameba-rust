#[doc = "Register `REG_CPU0_PC` reader"]
pub type R = crate::R<RegCpu0PcSpec>;
#[doc = "Register `REG_CPU0_PC` writer"]
pub type W = crate::W<RegCpu0PcSpec>;
#[doc = "Field `km4tz_dbg_pc_en` reader - debug enable"]
pub type Km4tzDbgPcEnR = crate::BitReader;
#[doc = "Field `km4tz_dbg_pc_en` writer - debug enable"]
pub type Km4tzDbgPcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4tz_dbg_pc` reader - cpu0 pc value for debug which is pc\\[31:2\\]"]
pub type Km4tzDbgPcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - debug enable"]
    #[inline(always)]
    pub fn km4tz_dbg_pc_en(&self) -> Km4tzDbgPcEnR {
        Km4tzDbgPcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - cpu0 pc value for debug which is pc\\[31:2\\]"]
    #[inline(always)]
    pub fn km4tz_dbg_pc(&self) -> Km4tzDbgPcR {
        Km4tzDbgPcR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - debug enable"]
    #[inline(always)]
    pub fn km4tz_dbg_pc_en(&mut self) -> Km4tzDbgPcEnW<'_, RegCpu0PcSpec> {
        Km4tzDbgPcEnW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cpu0_pc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cpu0_pc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCpu0PcSpec;
impl crate::RegisterSpec for RegCpu0PcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cpu0_pc::R`](R) reader structure"]
impl crate::Readable for RegCpu0PcSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cpu0_pc::W`](W) writer structure"]
impl crate::Writable for RegCpu0PcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CPU0_PC to value 0"]
impl crate::Resettable for RegCpu0PcSpec {}
