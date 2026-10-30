#[doc = "Register `TX_NDF` reader"]
pub type R = crate::R<TxNdfSpec>;
#[doc = "Register `TX_NDF` writer"]
pub type W = crate::W<TxNdfSpec>;
#[doc = "Field `TX_NDF` reader - Indicates a number of data frames (Unit: Byte). After transmitting command and address, SPIC transmits data continuously until data frames are equal to TX_NDF. TX_NDF can set larger than TX_FIFO size."]
pub type TxNdfR = crate::FieldReader<u32>;
#[doc = "Field `TX_NDF` writer - Indicates a number of data frames (Unit: Byte). After transmitting command and address, SPIC transmits data continuously until data frames are equal to TX_NDF. TX_NDF can set larger than TX_FIFO size."]
pub type TxNdfW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Indicates a number of data frames (Unit: Byte). After transmitting command and address, SPIC transmits data continuously until data frames are equal to TX_NDF. TX_NDF can set larger than TX_FIFO size."]
    #[inline(always)]
    pub fn tx_ndf(&self) -> TxNdfR {
        TxNdfR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Indicates a number of data frames (Unit: Byte). After transmitting command and address, SPIC transmits data continuously until data frames are equal to TX_NDF. TX_NDF can set larger than TX_FIFO size."]
    #[inline(always)]
    pub fn tx_ndf(&mut self) -> TxNdfW<'_, TxNdfSpec> {
        TxNdfW::new(self, 0)
    }
}
#[doc = "TX_NDF\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ndf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ndf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxNdfSpec;
impl crate::RegisterSpec for TxNdfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ndf::R`](R) reader structure"]
impl crate::Readable for TxNdfSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_ndf::W`](W) writer structure"]
impl crate::Writable for TxNdfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_NDF to value 0"]
impl crate::Resettable for TxNdfSpec {}
