#[doc = "Register `REG_MQ1_TXBD_DESA` reader"]
pub type R = crate::R<RegMq1TxbdDesaSpec>;
#[doc = "Register `REG_MQ1_TXBD_DESA` writer"]
pub type W = crate::W<RegMq1TxbdDesaSpec>;
#[doc = "Field `BIT_MQ1DESA` reader - Buffer Descriptor Base Address for M1 Queue"]
pub type BitMq1desaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MQ1DESA` writer - Buffer Descriptor Base Address for M1 Queue"]
pub type BitMq1desaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M1 Queue"]
    #[inline(always)]
    pub fn bit_mq1desa(&self) -> BitMq1desaR {
        BitMq1desaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for M1 Queue"]
    #[inline(always)]
    pub fn bit_mq1desa(&mut self) -> BitMq1desaW<'_, RegMq1TxbdDesaSpec> {
        BitMq1desaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq1_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq1_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq1TxbdDesaSpec;
impl crate::RegisterSpec for RegMq1TxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq1_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMq1TxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq1_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMq1TxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ1_TXBD_DESA to value 0"]
impl crate::Resettable for RegMq1TxbdDesaSpec {}
