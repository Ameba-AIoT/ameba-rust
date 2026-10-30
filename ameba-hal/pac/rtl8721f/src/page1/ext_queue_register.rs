#[doc = "Register `ext_queue_register` reader"]
pub type R = crate::R<ExtQueueRegisterSpec>;
#[doc = "Register `ext_queue_register` writer"]
pub type W = crate::W<ExtQueueRegisterSpec>;
#[doc = "Field `h2cq_pri` reader - "]
pub type H2cqPriR = crate::FieldReader;
#[doc = "Field `h2cq_pri` writer - "]
pub type H2cqPriW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXBUF_WKCAM_OFFSET` reader - the offset of WOL pattern stored in txpktbuffer, unit is 8byte"]
pub type TxbufWkcamOffsetR = crate::FieldReader<u16>;
#[doc = "Field `TXBUF_WKCAM_OFFSET` writer - the offset of WOL pattern stored in txpktbuffer, unit is 8byte"]
pub type TxbufWkcamOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn h2cq_pri(&self) -> H2cqPriR {
        H2cqPriR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:15 - the offset of WOL pattern stored in txpktbuffer, unit is 8byte"]
    #[inline(always)]
    pub fn txbuf_wkcam_offset(&self) -> TxbufWkcamOffsetR {
        TxbufWkcamOffsetR::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn h2cq_pri(&mut self) -> H2cqPriW<'_, ExtQueueRegisterSpec> {
        H2cqPriW::new(self, 0)
    }
    #[doc = "Bits 3:15 - the offset of WOL pattern stored in txpktbuffer, unit is 8byte"]
    #[inline(always)]
    pub fn txbuf_wkcam_offset(&mut self) -> TxbufWkcamOffsetW<'_, ExtQueueRegisterSpec> {
        TxbufWkcamOffsetW::new(self, 3)
    }
    #[doc = "Bits 16:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, ExtQueueRegisterSpec> {
        DummyW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_queue_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_queue_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtQueueRegisterSpec;
impl crate::RegisterSpec for ExtQueueRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_queue_register::R`](R) reader structure"]
impl crate::Readable for ExtQueueRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_queue_register::W`](W) writer structure"]
impl crate::Writable for ExtQueueRegisterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ext_queue_register to value 0"]
impl crate::Resettable for ExtQueueRegisterSpec {}
