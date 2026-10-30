#[doc = "Register `REG_ROM_PATCH` reader"]
pub type R = crate::R<RegRomPatchSpec>;
#[doc = "Field `ROM_PATCH_PG` reader - ROM patch enable.This Bit will programed by FT if ROM patch do not needed"]
pub type RomPatchPgR = crate::FieldReader;
#[doc = "Field `ROM_PATCH_LWE1` reader - ROM patch write protection(protect Low 256bits) : 500-51f"]
pub type RomPatchLwe1R = crate::BitReader;
#[doc = "Field `ROM_PATCH_LWE2` reader - ROM patch write protection(protect Low 256bits) : 520-53f"]
pub type RomPatchLwe2R = crate::BitReader;
#[doc = "Field `ROM_PATCH_LWE3` reader - ROM patch write protection(protect Low 256bits) : 540-55f"]
pub type RomPatchLwe3R = crate::BitReader;
#[doc = "Field `ROM_PATCH_LWE4` reader - ROM patch write protection(protect Low 256bits): 560-57f"]
pub type RomPatchLwe4R = crate::BitReader;
#[doc = "Field `ROM_PATCH_LWE5` reader - ROM patch write protection(protect low 1Kbits) :580-5ff"]
pub type RomPatchLwe5R = crate::BitReader;
#[doc = "Field `ROM_PATCH_HWE` reader - ROM patch write protection (protect High 2Kbits)"]
pub type RomPatchHweR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - ROM patch enable.This Bit will programed by FT if ROM patch do not needed"]
    #[inline(always)]
    pub fn rom_patch_pg(&self) -> RomPatchPgR {
        RomPatchPgR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ROM patch write protection(protect Low 256bits) : 500-51f"]
    #[inline(always)]
    pub fn rom_patch_lwe1(&self) -> RomPatchLwe1R {
        RomPatchLwe1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ROM patch write protection(protect Low 256bits) : 520-53f"]
    #[inline(always)]
    pub fn rom_patch_lwe2(&self) -> RomPatchLwe2R {
        RomPatchLwe2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ROM patch write protection(protect Low 256bits) : 540-55f"]
    #[inline(always)]
    pub fn rom_patch_lwe3(&self) -> RomPatchLwe3R {
        RomPatchLwe3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ROM patch write protection(protect Low 256bits): 560-57f"]
    #[inline(always)]
    pub fn rom_patch_lwe4(&self) -> RomPatchLwe4R {
        RomPatchLwe4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ROM patch write protection(protect low 1Kbits) :580-5ff"]
    #[inline(always)]
    pub fn rom_patch_lwe5(&self) -> RomPatchLwe5R {
        RomPatchLwe5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ROM patch write protection (protect High 2Kbits)"]
    #[inline(always)]
    pub fn rom_patch_hwe(&self) -> RomPatchHweR {
        RomPatchHweR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rom_patch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRomPatchSpec;
impl crate::RegisterSpec for RegRomPatchSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`reg_rom_patch::R`](R) reader structure"]
impl crate::Readable for RegRomPatchSpec {}
#[doc = "`reset()` method sets REG_ROM_PATCH to value 0xff"]
impl crate::Resettable for RegRomPatchSpec {
    const RESET_VALUE: u8 = 0xff;
}
