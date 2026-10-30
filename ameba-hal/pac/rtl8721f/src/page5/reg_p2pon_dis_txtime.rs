#[doc = "Register `REG_P2PON_DIS_TXTIME` reader"]
pub type R = crate::R<RegP2ponDisTxtimeSpec>;
#[doc = "Register `REG_P2PON_DIS_TXTIME` writer"]
pub type W = crate::W<RegP2ponDisTxtimeSpec>;
#[doc = "Field `BIT_P2PON_DIS_TXTIME` reader - The time of the disable TX period after the P2P RF on interrupt occur. Unit: 64us."]
pub type BitP2ponDisTxtimeR = crate::FieldReader;
#[doc = "Field `BIT_P2PON_DIS_TXTIME` writer - The time of the disable TX period after the P2P RF on interrupt occur. Unit: 64us."]
pub type BitP2ponDisTxtimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - The time of the disable TX period after the P2P RF on interrupt occur. Unit: 64us."]
    #[inline(always)]
    pub fn bit_p2pon_dis_txtime(&self) -> BitP2ponDisTxtimeR {
        BitP2ponDisTxtimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - The time of the disable TX period after the P2P RF on interrupt occur. Unit: 64us."]
    #[inline(always)]
    pub fn bit_p2pon_dis_txtime(&mut self) -> BitP2ponDisTxtimeW<'_, RegP2ponDisTxtimeSpec> {
        BitP2ponDisTxtimeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p2pon_dis_txtime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p2pon_dis_txtime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP2ponDisTxtimeSpec;
impl crate::RegisterSpec for RegP2ponDisTxtimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p2pon_dis_txtime::R`](R) reader structure"]
impl crate::Readable for RegP2ponDisTxtimeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p2pon_dis_txtime::W`](W) writer structure"]
impl crate::Writable for RegP2ponDisTxtimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P2PON_DIS_TXTIME to value 0"]
impl crate::Resettable for RegP2ponDisTxtimeSpec {}
