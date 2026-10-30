#[doc = "Register `REG_MQ5_TXBD_DESA` reader"]
pub type R = crate::R<RegMq5TxbdDesaSpec>;
#[doc = "Register `REG_MQ5_TXBD_DESA` writer"]
pub type W = crate::W<RegMq5TxbdDesaSpec>;
#[doc = "Field `BIT_MQ5DESA` reader - Buffer Descriptor Base Address for M5 Queue"]
pub type BitMq5desaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MQ5DESA` writer - Buffer Descriptor Base Address for M5 Queue"]
pub type BitMq5desaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M5 Queue"]
    #[inline(always)]
    pub fn bit_mq5desa(&self) -> BitMq5desaR {
        BitMq5desaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M5 Queue"]
    #[inline(always)]
    pub fn bit_mq5desa(&mut self) -> BitMq5desaW<'_, RegMq5TxbdDesaSpec> {
        BitMq5desaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq5_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq5_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq5TxbdDesaSpec;
impl crate::RegisterSpec for RegMq5TxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq5_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMq5TxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq5_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMq5TxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ5_TXBD_DESA to value 0"]
impl crate::Resettable for RegMq5TxbdDesaSpec {}
