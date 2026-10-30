#[doc = "Register `REG_MQ0_TXBD_DESA` reader"]
pub type R = crate::R<RegMq0TxbdDesaSpec>;
#[doc = "Register `REG_MQ0_TXBD_DESA` writer"]
pub type W = crate::W<RegMq0TxbdDesaSpec>;
#[doc = "Field `BIT_MQ0DESA` reader - Buffer Descriptor Base Address for M0 Queue"]
pub type BitMq0desaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MQ0DESA` writer - Buffer Descriptor Base Address for M0 Queue"]
pub type BitMq0desaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M0 Queue"]
    #[inline(always)]
    pub fn bit_mq0desa(&self) -> BitMq0desaR {
        BitMq0desaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M0 Queue"]
    #[inline(always)]
    pub fn bit_mq0desa(&mut self) -> BitMq0desaW<'_, RegMq0TxbdDesaSpec> {
        BitMq0desaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq0_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq0_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq0TxbdDesaSpec;
impl crate::RegisterSpec for RegMq0TxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq0_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMq0TxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq0_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMq0TxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ0_TXBD_DESA to value 0"]
impl crate::Resettable for RegMq0TxbdDesaSpec {}
