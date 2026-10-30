#[doc = "Register `REG_BOD` reader"]
pub type R = crate::R<RegBodSpec>;
#[doc = "Register `REG_BOD` writer"]
pub type W = crate::W<RegBodSpec>;
#[doc = "Field `REG_BOD_F` reader - 00000:3.3333 00001:3.2877 00010:3.2432 00011:3.2000 00100:3.1579 00101:3.1035 00110:3.0508 00111:3.0000 01000:2.9508 01001:2.9032 01010:2.8571 01011:2.8125 01100:2.7692 01101:2.7169 01110:2.6666 01111:2.6182 10000:2.5623 10001:2.5087 10010:2.4574 10011:2.4080 10100:2.3606 10101:2.3151 10110:2.2713 10111:2.2154 11000:2.1621 11001:2.1114 11010:2.0630 11011:2.0168 11100:1.9672 11101:1.9098 11110:1.8556 11111:1.8045"]
pub type RegBodFR = crate::FieldReader;
#[doc = "Field `REG_BOD_F` writer - 00000:3.3333 00001:3.2877 00010:3.2432 00011:3.2000 00100:3.1579 00101:3.1035 00110:3.0508 00111:3.0000 01000:2.9508 01001:2.9032 01010:2.8571 01011:2.8125 01100:2.7692 01101:2.7169 01110:2.6666 01111:2.6182 10000:2.5623 10001:2.5087 10010:2.4574 10011:2.4080 10100:2.3606 10101:2.3151 10110:2.2713 10111:2.2154 11000:2.1621 11001:2.1114 11010:2.0630 11011:2.0168 11100:1.9672 11101:1.9098 11110:1.8556 11111:1.8045"]
pub type RegBodFW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REG_BOD_F_d` reader - // dummy"]
pub type RegBodFDR = crate::BitReader;
#[doc = "Field `REG_BOD_F_d` writer - // dummy"]
pub type RegBodFDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_BOD_R` reader - 00000:3.3333 00001:3.2877 00010:3.2432 00011:3.2000 00100:3.1579 00101:3.1035 00110:3.0508 00111:3.0000 01000:2.9508 01001:2.9032 01010:2.8571 01011:2.8125 01100:2.7692 01101:2.7169 01110:2.6666 01111:2.6182 10000:2.5623 10001:2.5087 10010:2.4574 10011:2.4080 10100:2.3606 10101:2.3151 10110:2.2713 10111:2.2154 11000:2.1621 11001:2.1114 11010:2.0630 11011:2.0168 11100:1.9672 11101:1.9098 11110:1.8556 11111:1.8045"]
pub type RegBodRR = crate::FieldReader;
#[doc = "Field `REG_BOD_R` writer - 00000:3.3333 00001:3.2877 00010:3.2432 00011:3.2000 00100:3.1579 00101:3.1035 00110:3.0508 00111:3.0000 01000:2.9508 01001:2.9032 01010:2.8571 01011:2.8125 01100:2.7692 01101:2.7169 01110:2.6666 01111:2.6182 10000:2.5623 10001:2.5087 10010:2.4574 10011:2.4080 10100:2.3606 10101:2.3151 10110:2.2713 10111:2.2154 11000:2.1621 11001:2.1114 11010:2.0630 11011:2.0168 11100:1.9672 11101:1.9098 11110:1.8556 11111:1.8045"]
pub type RegBodRW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REG_BOD_R_en` reader - // 0: disable BOD // 1: enable BOD"]
pub type RegBodREnR = crate::BitReader;
#[doc = "Field `REG_BOD_R_en` writer - // 0: disable BOD // 1: enable BOD"]
pub type RegBodREnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmy_6` reader - // dummy)"]
pub type Dmy6R = crate::FieldReader;
#[doc = "Field `dmy_6` writer - // dummy)"]
pub type Dmy6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - 00000:3.3333 00001:3.2877 00010:3.2432 00011:3.2000 00100:3.1579 00101:3.1035 00110:3.0508 00111:3.0000 01000:2.9508 01001:2.9032 01010:2.8571 01011:2.8125 01100:2.7692 01101:2.7169 01110:2.6666 01111:2.6182 10000:2.5623 10001:2.5087 10010:2.4574 10011:2.4080 10100:2.3606 10101:2.3151 10110:2.2713 10111:2.2154 11000:2.1621 11001:2.1114 11010:2.0630 11011:2.0168 11100:1.9672 11101:1.9098 11110:1.8556 11111:1.8045"]
    #[inline(always)]
    pub fn reg_bod_f(&self) -> RegBodFR {
        RegBodFR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - // dummy"]
    #[inline(always)]
    pub fn reg_bod_f_d(&self) -> RegBodFDR {
        RegBodFDR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - 00000:3.3333 00001:3.2877 00010:3.2432 00011:3.2000 00100:3.1579 00101:3.1035 00110:3.0508 00111:3.0000 01000:2.9508 01001:2.9032 01010:2.8571 01011:2.8125 01100:2.7692 01101:2.7169 01110:2.6666 01111:2.6182 10000:2.5623 10001:2.5087 10010:2.4574 10011:2.4080 10100:2.3606 10101:2.3151 10110:2.2713 10111:2.2154 11000:2.1621 11001:2.1114 11010:2.0630 11011:2.0168 11100:1.9672 11101:1.9098 11110:1.8556 11111:1.8045"]
    #[inline(always)]
    pub fn reg_bod_r(&self) -> RegBodRR {
        RegBodRR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - // 0: disable BOD // 1: enable BOD"]
    #[inline(always)]
    pub fn reg_bod_r_en(&self) -> RegBodREnR {
        RegBodREnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17 - // dummy)"]
    #[inline(always)]
    pub fn dmy_6(&self) -> Dmy6R {
        Dmy6R::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 00000:3.3333 00001:3.2877 00010:3.2432 00011:3.2000 00100:3.1579 00101:3.1035 00110:3.0508 00111:3.0000 01000:2.9508 01001:2.9032 01010:2.8571 01011:2.8125 01100:2.7692 01101:2.7169 01110:2.6666 01111:2.6182 10000:2.5623 10001:2.5087 10010:2.4574 10011:2.4080 10100:2.3606 10101:2.3151 10110:2.2713 10111:2.2154 11000:2.1621 11001:2.1114 11010:2.0630 11011:2.0168 11100:1.9672 11101:1.9098 11110:1.8556 11111:1.8045"]
    #[inline(always)]
    pub fn reg_bod_f(&mut self) -> RegBodFW<'_, RegBodSpec> {
        RegBodFW::new(self, 0)
    }
    #[doc = "Bit 5 - // dummy"]
    #[inline(always)]
    pub fn reg_bod_f_d(&mut self) -> RegBodFDW<'_, RegBodSpec> {
        RegBodFDW::new(self, 5)
    }
    #[doc = "Bits 6:10 - 00000:3.3333 00001:3.2877 00010:3.2432 00011:3.2000 00100:3.1579 00101:3.1035 00110:3.0508 00111:3.0000 01000:2.9508 01001:2.9032 01010:2.8571 01011:2.8125 01100:2.7692 01101:2.7169 01110:2.6666 01111:2.6182 10000:2.5623 10001:2.5087 10010:2.4574 10011:2.4080 10100:2.3606 10101:2.3151 10110:2.2713 10111:2.2154 11000:2.1621 11001:2.1114 11010:2.0630 11011:2.0168 11100:1.9672 11101:1.9098 11110:1.8556 11111:1.8045"]
    #[inline(always)]
    pub fn reg_bod_r(&mut self) -> RegBodRW<'_, RegBodSpec> {
        RegBodRW::new(self, 6)
    }
    #[doc = "Bit 11 - // 0: disable BOD // 1: enable BOD"]
    #[inline(always)]
    pub fn reg_bod_r_en(&mut self) -> RegBodREnW<'_, RegBodSpec> {
        RegBodREnW::new(self, 11)
    }
    #[doc = "Bits 12:17 - // dummy)"]
    #[inline(always)]
    pub fn dmy_6(&mut self) -> Dmy6W<'_, RegBodSpec> {
        Dmy6W::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBodSpec;
impl crate::RegisterSpec for RegBodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bod::R`](R) reader structure"]
impl crate::Readable for RegBodSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bod::W`](W) writer structure"]
impl crate::Writable for RegBodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BOD to value 0x0a53"]
impl crate::Resettable for RegBodSpec {
    const RESET_VALUE: u32 = 0x0a53;
}
