#[doc = "Register `DSR5_L` reader"]
pub type R = crate::R<Dsr5LSpec>;
#[doc = "Register `DSR5_L` writer"]
pub type W = crate::W<Dsr5LSpec>;
#[doc = "Field `DSR5_L_DSI` reader - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr5LDsiR = crate::FieldReader<u32>;
#[doc = "Field `DSR5_L_DSI` writer - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr5LDsiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DSR5_L_DSC` reader - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr5LDscR = crate::FieldReader<u16>;
#[doc = "Field `DSR5_L_DSC` writer - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr5LDscW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr5_l_dsi(&self) -> Dsr5LDsiR {
        Dsr5LDsiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr5_l_dsc(&self) -> Dsr5LDscR {
        Dsr5LDscR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr5_l_dsi(&mut self) -> Dsr5LDsiW<'_, Dsr5LSpec> {
        Dsr5LDsiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr5_l_dsc(&mut self) -> Dsr5LDscW<'_, Dsr5LSpec> {
        Dsr5LDscW::new(self, 20)
    }
}
#[doc = "Channel 5 low-32 bits Destination Scatter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr5_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr5_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr5LSpec;
impl crate::RegisterSpec for Dsr5LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr5_l::R`](R) reader structure"]
impl crate::Readable for Dsr5LSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr5_l::W`](W) writer structure"]
impl crate::Writable for Dsr5LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSR5_L to value 0"]
impl crate::Resettable for Dsr5LSpec {}
