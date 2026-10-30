#[doc = "Register `PREFETCH_CTRL` reader"]
pub type R = crate::R<PrefetchCtrlSpec>;
#[doc = "Register `PREFETCH_CTRL` writer"]
pub type W = crate::W<PrefetchCtrlSpec>;
#[doc = "Field `REMAIN_ENTRY` reader - The remaining number of valid entries in the stream buffer required before automatically intiating prefetch.(support up to 16)"]
pub type RemainEntryR = crate::FieldReader;
#[doc = "Field `REMAIN_ENTRY` writer - The remaining number of valid entries in the stream buffer required before automatically intiating prefetch.(support up to 16)"]
pub type RemainEntryW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PREFETCH_DIST` reader - The size of prefech data 2'b00: no prefetch 2'h01: one cache line 2'h1?: two cache line"]
pub type PrefetchDistR = crate::FieldReader;
#[doc = "Field `PREFETCH_DIST` writer - The size of prefech data 2'b00: no prefetch 2'h01: one cache line 2'h1?: two cache line"]
pub type PrefetchDistW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DETECT_NUM_BURST` reader - The number of required sequential cache line accesses before initiating prefetch"]
pub type DetectNumBurstR = crate::FieldReader;
#[doc = "Field `DETECT_NUM_BURST` writer - The number of required sequential cache line accesses before initiating prefetch"]
pub type DetectNumBurstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DETECT_NUM_SINGLE` reader - The number of required sequential single accesses before initiating prefetch"]
pub type DetectNumSingleR = crate::FieldReader;
#[doc = "Field `DETECT_NUM_SINGLE` writer - The number of required sequential single accesses before initiating prefetch"]
pub type DetectNumSingleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - The remaining number of valid entries in the stream buffer required before automatically intiating prefetch.(support up to 16)"]
    #[inline(always)]
    pub fn remain_entry(&self) -> RemainEntryR {
        RemainEntryR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - The size of prefech data 2'b00: no prefetch 2'h01: one cache line 2'h1?: two cache line"]
    #[inline(always)]
    pub fn prefetch_dist(&self) -> PrefetchDistR {
        PrefetchDistR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - The number of required sequential cache line accesses before initiating prefetch"]
    #[inline(always)]
    pub fn detect_num_burst(&self) -> DetectNumBurstR {
        DetectNumBurstR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - The number of required sequential single accesses before initiating prefetch"]
    #[inline(always)]
    pub fn detect_num_single(&self) -> DetectNumSingleR {
        DetectNumSingleR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - The remaining number of valid entries in the stream buffer required before automatically intiating prefetch.(support up to 16)"]
    #[inline(always)]
    pub fn remain_entry(&mut self) -> RemainEntryW<'_, PrefetchCtrlSpec> {
        RemainEntryW::new(self, 0)
    }
    #[doc = "Bits 8:9 - The size of prefech data 2'b00: no prefetch 2'h01: one cache line 2'h1?: two cache line"]
    #[inline(always)]
    pub fn prefetch_dist(&mut self) -> PrefetchDistW<'_, PrefetchCtrlSpec> {
        PrefetchDistW::new(self, 8)
    }
    #[doc = "Bits 16:19 - The number of required sequential cache line accesses before initiating prefetch"]
    #[inline(always)]
    pub fn detect_num_burst(&mut self) -> DetectNumBurstW<'_, PrefetchCtrlSpec> {
        DetectNumBurstW::new(self, 16)
    }
    #[doc = "Bits 20:23 - The number of required sequential single accesses before initiating prefetch"]
    #[inline(always)]
    pub fn detect_num_single(&mut self) -> DetectNumSingleW<'_, PrefetchCtrlSpec> {
        DetectNumSingleW::new(self, 20)
    }
}
#[doc = "Prefetch control setting of stream buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`prefetch_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefetch_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrefetchCtrlSpec;
impl crate::RegisterSpec for PrefetchCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prefetch_ctrl::R`](R) reader structure"]
impl crate::Readable for PrefetchCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`prefetch_ctrl::W`](W) writer structure"]
impl crate::Writable for PrefetchCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PREFETCH_CTRL to value 0x0033_0208"]
impl crate::Resettable for PrefetchCtrlSpec {
    const RESET_VALUE: u32 = 0x0033_0208;
}
