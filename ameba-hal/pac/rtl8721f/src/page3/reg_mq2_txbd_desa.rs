#[doc = "Register `REG_MQ2_TXBD_DESA` reader"]
pub type R = crate::R<RegMq2TxbdDesaSpec>;
#[doc = "Register `REG_MQ2_TXBD_DESA` writer"]
pub type W = crate::W<RegMq2TxbdDesaSpec>;
#[doc = "Field `BIT_MQ2DESA` reader - Buffer Descriptor Base Address for M2 Queue"]
pub type BitMq2desaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MQ2DESA` writer - Buffer Descriptor Base Address for M2 Queue"]
pub type BitMq2desaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M2 Queue"]
    #[inline(always)]
    pub fn bit_mq2desa(&self) -> BitMq2desaR {
        BitMq2desaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M2 Queue"]
    #[inline(always)]
    pub fn bit_mq2desa(&mut self) -> BitMq2desaW<'_, RegMq2TxbdDesaSpec> {
        BitMq2desaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq2_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq2_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq2TxbdDesaSpec;
impl crate::RegisterSpec for RegMq2TxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq2_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMq2TxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq2_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMq2TxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ2_TXBD_DESA to value 0"]
impl crate::Resettable for RegMq2TxbdDesaSpec {}
