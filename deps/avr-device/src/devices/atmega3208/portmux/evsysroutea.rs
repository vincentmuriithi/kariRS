#[doc = "Register `EVSYSROUTEA` reader"]
pub struct R(crate::R<EVSYSROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSYSROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSYSROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSYSROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSYSROUTEA` writer"]
pub struct W(crate::W<EVSYSROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSYSROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EVSYSROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSYSROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVOUT0` reader - Event Output 0"]
pub type EVOUT0_R = crate::BitReader<bool>;
#[doc = "Field `EVOUT0` writer - Event Output 0"]
pub type EVOUT0_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, bool, O>;
#[doc = "Field `EVOUT1` reader - Event Output 1"]
pub type EVOUT1_R = crate::BitReader<bool>;
#[doc = "Field `EVOUT1` writer - Event Output 1"]
pub type EVOUT1_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, bool, O>;
#[doc = "Field `EVOUT2` reader - Event Output 2"]
pub type EVOUT2_R = crate::BitReader<bool>;
#[doc = "Field `EVOUT2` writer - Event Output 2"]
pub type EVOUT2_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, bool, O>;
#[doc = "Field `EVOUT3` reader - Event Output 3"]
pub type EVOUT3_R = crate::BitReader<bool>;
#[doc = "Field `EVOUT3` writer - Event Output 3"]
pub type EVOUT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, bool, O>;
#[doc = "Field `EVOUT4` reader - Event Output 4"]
pub type EVOUT4_R = crate::BitReader<bool>;
#[doc = "Field `EVOUT4` writer - Event Output 4"]
pub type EVOUT4_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, bool, O>;
#[doc = "Field `EVOUT5` reader - Event Output 5"]
pub type EVOUT5_R = crate::BitReader<bool>;
#[doc = "Field `EVOUT5` writer - Event Output 5"]
pub type EVOUT5_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Event Output 0"]
    #[inline(always)]
    pub fn evout0(&self) -> EVOUT0_R {
        EVOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Output 1"]
    #[inline(always)]
    pub fn evout1(&self) -> EVOUT1_R {
        EVOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Output 2"]
    #[inline(always)]
    pub fn evout2(&self) -> EVOUT2_R {
        EVOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Output 3"]
    #[inline(always)]
    pub fn evout3(&self) -> EVOUT3_R {
        EVOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Output 4"]
    #[inline(always)]
    pub fn evout4(&self) -> EVOUT4_R {
        EVOUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Output 5"]
    #[inline(always)]
    pub fn evout5(&self) -> EVOUT5_R {
        EVOUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Output 0"]
    #[inline(always)]
    #[must_use]
    pub fn evout0(&mut self) -> EVOUT0_W<0> {
        EVOUT0_W::new(self)
    }
    #[doc = "Bit 1 - Event Output 1"]
    #[inline(always)]
    #[must_use]
    pub fn evout1(&mut self) -> EVOUT1_W<1> {
        EVOUT1_W::new(self)
    }
    #[doc = "Bit 2 - Event Output 2"]
    #[inline(always)]
    #[must_use]
    pub fn evout2(&mut self) -> EVOUT2_W<2> {
        EVOUT2_W::new(self)
    }
    #[doc = "Bit 3 - Event Output 3"]
    #[inline(always)]
    #[must_use]
    pub fn evout3(&mut self) -> EVOUT3_W<3> {
        EVOUT3_W::new(self)
    }
    #[doc = "Bit 4 - Event Output 4"]
    #[inline(always)]
    #[must_use]
    pub fn evout4(&mut self) -> EVOUT4_W<4> {
        EVOUT4_W::new(self)
    }
    #[doc = "Bit 5 - Event Output 5"]
    #[inline(always)]
    #[must_use]
    pub fn evout5(&mut self) -> EVOUT5_W<5> {
        EVOUT5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexer EVSYS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evsysroutea](index.html) module"]
pub struct EVSYSROUTEA_SPEC;
impl crate::RegisterSpec for EVSYSROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evsysroutea::R](R) reader structure"]
impl crate::Readable for EVSYSROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evsysroutea::W](W) writer structure"]
impl crate::Writable for EVSYSROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSYSROUTEA to value 0"]
impl crate::Resettable for EVSYSROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
