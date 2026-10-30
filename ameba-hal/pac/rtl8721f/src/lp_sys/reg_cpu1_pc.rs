#[doc = "Register `REG_CPU1_PC` reader"]
pub type R = crate::R<RegCpu1PcSpec>;
#[doc = "Register `REG_CPU1_PC` writer"]
pub type W = crate::W<RegCpu1PcSpec>;
#[doc = "Field `km4ns_dbg_pc_en` reader - debug enable"]
pub type Km4nsDbgPcEnR = crate::BitReader;
#[doc = "Field `km4ns_dbg_pc_en` writer - debug enable"]
pub type Km4nsDbgPcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4ns_dbg_pc` reader - cpu1 pc value for debug which is pc\\[31:2\\]"]
pub type Km4nsDbgPcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - debug enable"]
    #[inline(always)]
    pub fn km4ns_dbg_pc_en(&self) -> Km4nsDbgPcEnR {
        Km4nsDbgPcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - cpu1 pc value for debug which is pc\\[31:2\\]"]
    #[inline(always)]
    pub fn km4ns_dbg_pc(&self) -> Km4nsDbgPcR {
        Km4nsDbgPcR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - debug enable"]
    #[inline(always)]
    pub fn km4ns_dbg_pc_en(&mut self) -> Km4nsDbgPcEnW<'_, RegCpu1PcSpec> {
        Km4nsDbgPcEnW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cpu1_pc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cpu1_pc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCpu1PcSpec;
impl crate::RegisterSpec for RegCpu1PcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cpu1_pc::R`](R) reader structure"]
impl crate::Readable for RegCpu1PcSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cpu1_pc::W`](W) writer structure"]
impl crate::Writable for RegCpu1PcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CPU1_PC to value 0"]
impl crate::Resettable for RegCpu1PcSpec {}
