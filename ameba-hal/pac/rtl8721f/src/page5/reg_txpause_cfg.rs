#[doc = "Register `REG_TXPAUSE_CFG` reader"]
pub type R = crate::R<RegTxpauseCfgSpec>;
#[doc = "Register `REG_TXPAUSE_CFG` writer"]
pub type W = crate::W<RegTxpauseCfgSpec>;
#[doc = "Field `BIT_DRVERLYITV_P2` reader - Time to send interrupt to notify driver to change beacon content. Time unit is TU. It is used to wake up early at DriverEarlyTime. For port 2"]
pub type BitDrverlyitvP2R = crate::FieldReader;
#[doc = "Field `BIT_DRVERLYITV_P2` writer - Time to send interrupt to notify driver to change beacon content. Time unit is TU. It is used to wake up early at DriverEarlyTime. For port 2"]
pub type BitDrverlyitvP2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_TXPAUSE1` reader - same as 0x522\\[7:0\\], allow FW/DR to control pause jointly"]
pub type BitTxpause1R = crate::FieldReader;
#[doc = "Field `BIT_TXPAUSE1` writer - same as 0x522\\[7:0\\], allow FW/DR to control pause jointly"]
pub type BitTxpause1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_TSF_DIFF_P1P2` reader - the difference of beacon count down value between port0 and port1, unit is TU"]
pub type BitTsfDiffP1p2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Time to send interrupt to notify driver to change beacon content. Time unit is TU. It is used to wake up early at DriverEarlyTime. For port 2"]
    #[inline(always)]
    pub fn bit_drverlyitv_p2(&self) -> BitDrverlyitvP2R {
        BitDrverlyitvP2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - same as 0x522\\[7:0\\], allow FW/DR to control pause jointly"]
    #[inline(always)]
    pub fn bit_txpause1(&self) -> BitTxpause1R {
        BitTxpause1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - the difference of beacon count down value between port0 and port1, unit is TU"]
    #[inline(always)]
    pub fn bit_tsf_diff_p1p2(&self) -> BitTsfDiffP1p2R {
        BitTsfDiffP1p2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time to send interrupt to notify driver to change beacon content. Time unit is TU. It is used to wake up early at DriverEarlyTime. For port 2"]
    #[inline(always)]
    pub fn bit_drverlyitv_p2(&mut self) -> BitDrverlyitvP2W<'_, RegTxpauseCfgSpec> {
        BitDrverlyitvP2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - same as 0x522\\[7:0\\], allow FW/DR to control pause jointly"]
    #[inline(always)]
    pub fn bit_txpause1(&mut self) -> BitTxpause1W<'_, RegTxpauseCfgSpec> {
        BitTxpause1W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txpause_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txpause_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxpauseCfgSpec;
impl crate::RegisterSpec for RegTxpauseCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txpause_cfg::R`](R) reader structure"]
impl crate::Readable for RegTxpauseCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txpause_cfg::W`](W) writer structure"]
impl crate::Writable for RegTxpauseCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXPAUSE_CFG to value 0x05"]
impl crate::Resettable for RegTxpauseCfgSpec {
    const RESET_VALUE: u32 = 0x05;
}
