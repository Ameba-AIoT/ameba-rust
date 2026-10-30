#[doc = "Register `CAN_ERR_CNT_CTL` reader"]
pub type R = crate::R<CanErrCntCtlSpec>;
#[doc = "Register `CAN_ERR_CNT_CTL` writer"]
pub type W = crate::W<CanErrCntCtlSpec>;
#[doc = "Field `tx_err_cnt_clr` writer - write 1 to clear can_tec"]
pub type TxErrCntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_err_cnt_clr` writer - write 1 to clear can_rec"]
pub type RxErrCntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_error_warn_th` reader - error counter warning threshold, programable, default 96. NOTES: 0~127 is availaible to both TEC and REC, 128~255 is only available to TEC"]
pub type CanErrorWarnThR = crate::FieldReader<u16>;
#[doc = "Field `can_error_warn_th` writer - error counter warning threshold, programable, default 96. NOTES: 0~127 is availaible to both TEC and REC, 128~255 is only available to TEC"]
pub type CanErrorWarnThW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 8:16 - error counter warning threshold, programable, default 96. NOTES: 0~127 is availaible to both TEC and REC, 128~255 is only available to TEC"]
    #[inline(always)]
    pub fn can_error_warn_th(&self) -> CanErrorWarnThR {
        CanErrorWarnThR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to clear can_tec"]
    #[inline(always)]
    pub fn tx_err_cnt_clr(&mut self) -> TxErrCntClrW<'_, CanErrCntCtlSpec> {
        TxErrCntClrW::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to clear can_rec"]
    #[inline(always)]
    pub fn rx_err_cnt_clr(&mut self) -> RxErrCntClrW<'_, CanErrCntCtlSpec> {
        RxErrCntClrW::new(self, 1)
    }
    #[doc = "Bits 8:16 - error counter warning threshold, programable, default 96. NOTES: 0~127 is availaible to both TEC and REC, 128~255 is only available to TEC"]
    #[inline(always)]
    pub fn can_error_warn_th(&mut self) -> CanErrorWarnThW<'_, CanErrCntCtlSpec> {
        CanErrorWarnThW::new(self, 8)
    }
}
#[doc = "CAN bus error counter control\n\nYou can [`read`](crate::Reg::read) this register and get [`can_err_cnt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_err_cnt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanErrCntCtlSpec;
impl crate::RegisterSpec for CanErrCntCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_err_cnt_ctl::R`](R) reader structure"]
impl crate::Readable for CanErrCntCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_err_cnt_ctl::W`](W) writer structure"]
impl crate::Writable for CanErrCntCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_ERR_CNT_CTL to value 0x6000"]
impl crate::Resettable for CanErrCntCtlSpec {
    const RESET_VALUE: u32 = 0x6000;
}
