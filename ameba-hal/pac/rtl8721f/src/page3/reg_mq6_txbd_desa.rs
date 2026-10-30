#[doc = "Register `REG_MQ6_TXBD_DESA` reader"]
pub type R = crate::R<RegMq6TxbdDesaSpec>;
#[doc = "Register `REG_MQ6_TXBD_DESA` writer"]
pub type W = crate::W<RegMq6TxbdDesaSpec>;
#[doc = "Field `BIT_MQ6DESA` reader - Buffer Descriptor Base Address for M6 Queue"]
pub type BitMq6desaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MQ6DESA` writer - Buffer Descriptor Base Address for M6 Queue"]
pub type BitMq6desaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M6 Queue"]
    #[inline(always)]
    pub fn bit_mq6desa(&self) -> BitMq6desaR {
        BitMq6desaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M6 Queue"]
    #[inline(always)]
    pub fn bit_mq6desa(&mut self) -> BitMq6desaW<'_, RegMq6TxbdDesaSpec> {
        BitMq6desaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq6_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq6_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq6TxbdDesaSpec;
impl crate::RegisterSpec for RegMq6TxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq6_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMq6TxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq6_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMq6TxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ6_TXBD_DESA to value 0"]
impl crate::Resettable for RegMq6TxbdDesaSpec {}
