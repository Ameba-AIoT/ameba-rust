#[doc = "Register `REG_MQ3_TXBD_DESA` reader"]
pub type R = crate::R<RegMq3TxbdDesaSpec>;
#[doc = "Register `REG_MQ3_TXBD_DESA` writer"]
pub type W = crate::W<RegMq3TxbdDesaSpec>;
#[doc = "Field `BIT_MQ3DESA` reader - Buffer Descriptor Base Address for M3 Queue"]
pub type BitMq3desaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MQ3DESA` writer - Buffer Descriptor Base Address for M3 Queue"]
pub type BitMq3desaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M3 Queue"]
    #[inline(always)]
    pub fn bit_mq3desa(&self) -> BitMq3desaR {
        BitMq3desaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M3 Queue"]
    #[inline(always)]
    pub fn bit_mq3desa(&mut self) -> BitMq3desaW<'_, RegMq3TxbdDesaSpec> {
        BitMq3desaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq3_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq3_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq3TxbdDesaSpec;
impl crate::RegisterSpec for RegMq3TxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq3_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMq3TxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq3_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMq3TxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ3_TXBD_DESA to value 0"]
impl crate::Resettable for RegMq3TxbdDesaSpec {}
