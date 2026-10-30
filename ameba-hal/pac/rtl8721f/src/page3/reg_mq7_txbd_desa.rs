#[doc = "Register `REG_MQ7_TXBD_DESA` reader"]
pub type R = crate::R<RegMq7TxbdDesaSpec>;
#[doc = "Register `REG_MQ7_TXBD_DESA` writer"]
pub type W = crate::W<RegMq7TxbdDesaSpec>;
#[doc = "Field `BIT_MQ7DESA` reader - Buffer Descriptor Base Address for M7 Queue"]
pub type BitMq7desaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MQ7DESA` writer - Buffer Descriptor Base Address for M7 Queue"]
pub type BitMq7desaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M7 Queue"]
    #[inline(always)]
    pub fn bit_mq7desa(&self) -> BitMq7desaR {
        BitMq7desaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M7 Queue"]
    #[inline(always)]
    pub fn bit_mq7desa(&mut self) -> BitMq7desaW<'_, RegMq7TxbdDesaSpec> {
        BitMq7desaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq7_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq7_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq7TxbdDesaSpec;
impl crate::RegisterSpec for RegMq7TxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq7_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMq7TxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq7_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMq7TxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ7_TXBD_DESA to value 0"]
impl crate::Resettable for RegMq7TxbdDesaSpec {}
