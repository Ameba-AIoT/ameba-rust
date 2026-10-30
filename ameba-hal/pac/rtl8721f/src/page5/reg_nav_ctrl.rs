#[doc = "Register `REG_NAV_CTRL` reader"]
pub type R = crate::R<RegNavCtrlSpec>;
#[doc = "Register `REG_NAV_CTRL` writer"]
pub type W = crate::W<RegNavCtrlSpec>;
#[doc = "Field `BIT_RD_NAV_PROT_NXT` reader - In RDG mode ,NAV setting which protects next packets"]
pub type BitRdNavProtNxtR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RD_NAV_PROT_NXT` writer - In RDG mode ,NAV setting which protects next packets"]
pub type BitRdNavProtNxtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_NAV_PROT_LEN` reader - In TXOP mode ,NAV setting which protects next packets"]
pub type BitNavProtLenR = crate::FieldReader<u16>;
#[doc = "Field `BIT_NAV_PROT_LEN` writer - In TXOP mode ,NAV setting which protects next packets"]
pub type BitNavProtLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - In RDG mode ,NAV setting which protects next packets"]
    #[inline(always)]
    pub fn bit_rd_nav_prot_nxt(&self) -> BitRdNavProtNxtR {
        BitRdNavProtNxtR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - In TXOP mode ,NAV setting which protects next packets"]
    #[inline(always)]
    pub fn bit_nav_prot_len(&self) -> BitNavProtLenR {
        BitNavProtLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - In RDG mode ,NAV setting which protects next packets"]
    #[inline(always)]
    pub fn bit_rd_nav_prot_nxt(&mut self) -> BitRdNavProtNxtW<'_, RegNavCtrlSpec> {
        BitRdNavProtNxtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - In TXOP mode ,NAV setting which protects next packets"]
    #[inline(always)]
    pub fn bit_nav_prot_len(&mut self) -> BitNavProtLenW<'_, RegNavCtrlSpec> {
        BitNavProtLenW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nav_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nav_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNavCtrlSpec;
impl crate::RegisterSpec for RegNavCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nav_ctrl::R`](R) reader structure"]
impl crate::Readable for RegNavCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_nav_ctrl::W`](W) writer structure"]
impl crate::Writable for RegNavCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAV_CTRL to value 0x0040_0180"]
impl crate::Resettable for RegNavCtrlSpec {
    const RESET_VALUE: u32 = 0x0040_0180;
}
