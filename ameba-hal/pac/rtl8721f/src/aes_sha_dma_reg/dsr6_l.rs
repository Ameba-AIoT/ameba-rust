#[doc = "Register `DSR6_L` reader"]
pub type R = crate::R<Dsr6LSpec>;
#[doc = "Register `DSR6_L` writer"]
pub type W = crate::W<Dsr6LSpec>;
#[doc = "Field `DSR6_L_DSI` reader - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr6LDsiR = crate::FieldReader<u32>;
#[doc = "Field `DSR6_L_DSI` writer - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr6LDsiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DSR6_L_DSC` reader - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr6LDscR = crate::FieldReader<u16>;
#[doc = "Field `DSR6_L_DSC` writer - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr6LDscW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr6_l_dsi(&self) -> Dsr6LDsiR {
        Dsr6LDsiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr6_l_dsc(&self) -> Dsr6LDscR {
        Dsr6LDscR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr6_l_dsi(&mut self) -> Dsr6LDsiW<'_, Dsr6LSpec> {
        Dsr6LDsiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr6_l_dsc(&mut self) -> Dsr6LDscW<'_, Dsr6LSpec> {
        Dsr6LDscW::new(self, 20)
    }
}
#[doc = "Channel 6 low-32 bits Destination Scatter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr6_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr6_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr6LSpec;
impl crate::RegisterSpec for Dsr6LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr6_l::R`](R) reader structure"]
impl crate::Readable for Dsr6LSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr6_l::W`](W) writer structure"]
impl crate::Writable for Dsr6LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSR6_L to value 0"]
impl crate::Resettable for Dsr6LSpec {}
