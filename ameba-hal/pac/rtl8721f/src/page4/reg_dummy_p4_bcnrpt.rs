#[doc = "Register `REG_DUMMY_P4_BCNRPT` reader"]
pub type R = crate::R<RegDummyP4BcnrptSpec>;
#[doc = "Register `REG_DUMMY_P4_BCNRPT` writer"]
pub type W = crate::W<RegDummyP4BcnrptSpec>;
#[doc = "Field `BIT_DUMMY_REG_0` reader - dummy reg"]
pub type BitDummyReg0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_DUMMY_REG_0` writer - dummy reg"]
pub type BitDummyReg0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `BIT_EN_BCN_TRXRPT` reader - If this bit is set, PTCL will write BCN TRXRPT when BCNOK/BCNERR INT. FW have clear this bit before enter LPS 32K mode."]
pub type BitEnBcnTrxrptR = crate::BitReader;
#[doc = "Field `BIT_EN_BCN_TRXRPT` writer - If this bit is set, PTCL will write BCN TRXRPT when BCNOK/BCNERR INT. FW have clear this bit before enter LPS 32K mode."]
pub type BitEnBcnTrxrptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DUMMY_REG_1` reader - dummy reg"]
pub type BitDummyReg1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_DUMMY_REG_1` writer - dummy reg"]
pub type BitDummyReg1W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:16 - dummy reg"]
    #[inline(always)]
    pub fn bit_dummy_reg_0(&self) -> BitDummyReg0R {
        BitDummyReg0R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - If this bit is set, PTCL will write BCN TRXRPT when BCNOK/BCNERR INT. FW have clear this bit before enter LPS 32K mode."]
    #[inline(always)]
    pub fn bit_en_bcn_trxrpt(&self) -> BitEnBcnTrxrptR {
        BitEnBcnTrxrptR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - dummy reg"]
    #[inline(always)]
    pub fn bit_dummy_reg_1(&self) -> BitDummyReg1R {
        BitDummyReg1R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - dummy reg"]
    #[inline(always)]
    pub fn bit_dummy_reg_0(&mut self) -> BitDummyReg0W<'_, RegDummyP4BcnrptSpec> {
        BitDummyReg0W::new(self, 0)
    }
    #[doc = "Bit 17 - If this bit is set, PTCL will write BCN TRXRPT when BCNOK/BCNERR INT. FW have clear this bit before enter LPS 32K mode."]
    #[inline(always)]
    pub fn bit_en_bcn_trxrpt(&mut self) -> BitEnBcnTrxrptW<'_, RegDummyP4BcnrptSpec> {
        BitEnBcnTrxrptW::new(self, 17)
    }
    #[doc = "Bits 18:31 - dummy reg"]
    #[inline(always)]
    pub fn bit_dummy_reg_1(&mut self) -> BitDummyReg1W<'_, RegDummyP4BcnrptSpec> {
        BitDummyReg1W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy_p4_bcnrpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy_p4_bcnrpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummyP4BcnrptSpec;
impl crate::RegisterSpec for RegDummyP4BcnrptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy_p4_bcnrpt::R`](R) reader structure"]
impl crate::Readable for RegDummyP4BcnrptSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy_p4_bcnrpt::W`](W) writer structure"]
impl crate::Writable for RegDummyP4BcnrptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY_P4_BCNRPT to value 0xffff_0000"]
impl crate::Resettable for RegDummyP4BcnrptSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
