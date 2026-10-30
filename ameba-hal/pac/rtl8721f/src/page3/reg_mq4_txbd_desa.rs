#[doc = "Register `REG_MQ4_TXBD_DESA` reader"]
pub type R = crate::R<RegMq4TxbdDesaSpec>;
#[doc = "Register `REG_MQ4_TXBD_DESA` writer"]
pub type W = crate::W<RegMq4TxbdDesaSpec>;
#[doc = "Field `BIT_MQ4DESA` reader - Buffer Descriptor Base Address for M4 Queue"]
pub type BitMq4desaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MQ4DESA` writer - Buffer Descriptor Base Address for M4 Queue"]
pub type BitMq4desaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M4 Queue"]
    #[inline(always)]
    pub fn bit_mq4desa(&self) -> BitMq4desaR {
        BitMq4desaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M4 Queue"]
    #[inline(always)]
    pub fn bit_mq4desa(&mut self) -> BitMq4desaW<'_, RegMq4TxbdDesaSpec> {
        BitMq4desaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq4_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq4_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq4TxbdDesaSpec;
impl crate::RegisterSpec for RegMq4TxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq4_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMq4TxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq4_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMq4TxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ4_TXBD_DESA to value 0"]
impl crate::Resettable for RegMq4TxbdDesaSpec {}
