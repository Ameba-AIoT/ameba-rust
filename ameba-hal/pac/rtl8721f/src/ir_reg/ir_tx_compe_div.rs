#[doc = "Register `IR_TX_COMPE_DIV` reader"]
pub type R = crate::R<IrTxCompeDivSpec>;
#[doc = "Register `IR_TX_COMPE_DIV` writer"]
pub type W = crate::W<IrTxCompeDivSpec>;
#[doc = "Field `TX_COMPE_DIV` reader - IR_TX_CLK_Period = SCLK/(TX_COMPE_DIV + 1)"]
pub type TxCompeDivR = crate::FieldReader<u16>;
#[doc = "Field `TX_COMPE_DIV` writer - IR_TX_CLK_Period = SCLK/(TX_COMPE_DIV + 1)"]
pub type TxCompeDivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - IR_TX_CLK_Period = SCLK/(TX_COMPE_DIV + 1)"]
    #[inline(always)]
    pub fn tx_compe_div(&self) -> TxCompeDivR {
        TxCompeDivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - IR_TX_CLK_Period = SCLK/(TX_COMPE_DIV + 1)"]
    #[inline(always)]
    pub fn tx_compe_div(&mut self) -> TxCompeDivW<'_, IrTxCompeDivSpec> {
        TxCompeDivW::new(self, 0)
    }
}
#[doc = "IR Tx compensation division register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_tx_compe_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_tx_compe_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrTxCompeDivSpec;
impl crate::RegisterSpec for IrTxCompeDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_tx_compe_div::R`](R) reader structure"]
impl crate::Readable for IrTxCompeDivSpec {}
#[doc = "`write(|w| ..)` method takes [`ir_tx_compe_div::W`](W) writer structure"]
impl crate::Writable for IrTxCompeDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_TX_COMPE_DIV to value 0"]
impl crate::Resettable for IrTxCompeDivSpec {}
