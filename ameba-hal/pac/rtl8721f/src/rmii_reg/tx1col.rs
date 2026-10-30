#[doc = "Register `Tx1Col` reader"]
pub type R = crate::R<Tx1colSpec>;
#[doc = "Field `Tx1Col` reader - 16-bit counter of TxCol packets. Rolls over automatically. Write to clear. This only record which have entered just one collision before Tx OK"]
pub type Tx1colR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of TxCol packets. Rolls over automatically. Write to clear. This only record which have entered just one collision before Tx OK"]
    #[inline(always)]
    pub fn tx1col(&self) -> Tx1colR {
        Tx1colR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx1col::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx1colSpec;
impl crate::RegisterSpec for Tx1colSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tx1col::R`](R) reader structure"]
impl crate::Readable for Tx1colSpec {}
#[doc = "`reset()` method sets Tx1Col to value 0"]
impl crate::Resettable for Tx1colSpec {}
