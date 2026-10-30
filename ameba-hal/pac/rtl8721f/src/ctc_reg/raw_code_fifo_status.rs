#[doc = "Register `RAW_CODE_FIFO_STATUS` reader"]
pub type R = crate::R<RawCodeFifoStatusSpec>;
#[doc = "Register `RAW_CODE_FIFO_STATUS` writer"]
pub type W = crate::W<RawCodeFifoStatusSpec>;
#[doc = "Field `afifo_full` reader - 0: Raw code FIFO not oveflow (not full) 1: Raw code FIFO overflow(full)"]
pub type AfifoFullR = crate::BitReader;
#[doc = "Field `afifo_empty` reader - 0: Raw code FIFO not empty 1: Raw code FIFO empty"]
pub type AfifoEmptyR = crate::BitReader;
#[doc = "Field `afifo_over_lvl` reader - 0: Raw code FIFO not over level 1: Raw code FIFO over level"]
pub type AfifoOverLvlR = crate::BitReader;
#[doc = "Field `afifo_valid_cnt` reader - Raw code FIFO valid cnt(push unit number which can be read)"]
pub type AfifoValidCntR = crate::FieldReader;
#[doc = "Field `afifo_full_lvl` reader - user define the full level for afifo, the valid value is from 0 to 63"]
pub type AfifoFullLvlR = crate::FieldReader;
#[doc = "Field `afifo_full_lvl` writer - user define the full level for afifo, the valid value is from 0 to 63"]
pub type AfifoFullLvlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `afifo_clear` writer - clear the FIFO data"]
pub type AfifoClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Raw code FIFO not oveflow (not full) 1: Raw code FIFO overflow(full)"]
    #[inline(always)]
    pub fn afifo_full(&self) -> AfifoFullR {
        AfifoFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Raw code FIFO not empty 1: Raw code FIFO empty"]
    #[inline(always)]
    pub fn afifo_empty(&self) -> AfifoEmptyR {
        AfifoEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Raw code FIFO not over level 1: Raw code FIFO over level"]
    #[inline(always)]
    pub fn afifo_over_lvl(&self) -> AfifoOverLvlR {
        AfifoOverLvlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - Raw code FIFO valid cnt(push unit number which can be read)"]
    #[inline(always)]
    pub fn afifo_valid_cnt(&self) -> AfifoValidCntR {
        AfifoValidCntR::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 10:15 - user define the full level for afifo, the valid value is from 0 to 63"]
    #[inline(always)]
    pub fn afifo_full_lvl(&self) -> AfifoFullLvlR {
        AfifoFullLvlR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15 - user define the full level for afifo, the valid value is from 0 to 63"]
    #[inline(always)]
    pub fn afifo_full_lvl(&mut self) -> AfifoFullLvlW<'_, RawCodeFifoStatusSpec> {
        AfifoFullLvlW::new(self, 10)
    }
    #[doc = "Bit 16 - clear the FIFO data"]
    #[inline(always)]
    pub fn afifo_clear(&mut self) -> AfifoClearW<'_, RawCodeFifoStatusSpec> {
        AfifoClearW::new(self, 16)
    }
}
#[doc = "FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_code_fifo_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_code_fifo_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawCodeFifoStatusSpec;
impl crate::RegisterSpec for RawCodeFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_code_fifo_status::R`](R) reader structure"]
impl crate::Readable for RawCodeFifoStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`raw_code_fifo_status::W`](W) writer structure"]
impl crate::Writable for RawCodeFifoStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAW_CODE_FIFO_STATUS to value 0x02"]
impl crate::Resettable for RawCodeFifoStatusSpec {
    const RESET_VALUE: u32 = 0x02;
}
