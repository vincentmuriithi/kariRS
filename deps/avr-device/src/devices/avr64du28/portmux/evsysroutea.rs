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
#[doc = "Field `EVOUTA` reader - Event Output A"]
pub type EVOUTA_R = crate::BitReader<EVOUTA_A>;
#[doc = "Event Output A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVOUTA_A {
    #[doc = "0: EVOUTA: PA2"]
    DEFAULT = 0,
    #[doc = "1: EVOUTA: PA7"]
    ALT1 = 1,
}
impl From<EVOUTA_A> for bool {
    #[inline(always)]
    fn from(variant: EVOUTA_A) -> Self {
        variant as u8 != 0
    }
}
impl EVOUTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVOUTA_A {
        match self.bits {
            false => EVOUTA_A::DEFAULT,
            true => EVOUTA_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == EVOUTA_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == EVOUTA_A::ALT1
    }
}
#[doc = "Field `EVOUTA` writer - Event Output A"]
pub type EVOUTA_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, EVOUTA_A, O>;
impl<'a, const O: u8> EVOUTA_W<'a, O> {
    #[doc = "EVOUTA: PA2"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(EVOUTA_A::DEFAULT)
    }
    #[doc = "EVOUTA: PA7"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(EVOUTA_A::ALT1)
    }
}
#[doc = "Field `EVOUTD` reader - Event Output D"]
pub type EVOUTD_R = crate::BitReader<EVOUTD_A>;
#[doc = "Event Output D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVOUTD_A {
    #[doc = "0: EVOUTD: PD2"]
    DEFAULT = 0,
    #[doc = "1: EVOUTD: PD7"]
    ALT1 = 1,
}
impl From<EVOUTD_A> for bool {
    #[inline(always)]
    fn from(variant: EVOUTD_A) -> Self {
        variant as u8 != 0
    }
}
impl EVOUTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVOUTD_A {
        match self.bits {
            false => EVOUTD_A::DEFAULT,
            true => EVOUTD_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == EVOUTD_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == EVOUTD_A::ALT1
    }
}
#[doc = "Field `EVOUTD` writer - Event Output D"]
pub type EVOUTD_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, EVOUTD_A, O>;
impl<'a, const O: u8> EVOUTD_W<'a, O> {
    #[doc = "EVOUTD: PD2"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(EVOUTD_A::DEFAULT)
    }
    #[doc = "EVOUTD: PD7"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(EVOUTD_A::ALT1)
    }
}
#[doc = "Field `EVOUTF` reader - Event Output F"]
pub type EVOUTF_R = crate::BitReader<EVOUTF_A>;
#[doc = "Event Output F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVOUTF_A {
    #[doc = "0: Not connected to any pins"]
    DEFAULT = 0,
    #[doc = "1: EVOUTF: PF7"]
    ALT1 = 1,
}
impl From<EVOUTF_A> for bool {
    #[inline(always)]
    fn from(variant: EVOUTF_A) -> Self {
        variant as u8 != 0
    }
}
impl EVOUTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVOUTF_A {
        match self.bits {
            false => EVOUTF_A::DEFAULT,
            true => EVOUTF_A::ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == EVOUTF_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == EVOUTF_A::ALT1
    }
}
#[doc = "Field `EVOUTF` writer - Event Output F"]
pub type EVOUTF_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVSYSROUTEA_SPEC, EVOUTF_A, O>;
impl<'a, const O: u8> EVOUTF_W<'a, O> {
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(EVOUTF_A::DEFAULT)
    }
    #[doc = "EVOUTF: PF7"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(EVOUTF_A::ALT1)
    }
}
impl R {
    #[doc = "Bit 0 - Event Output A"]
    #[inline(always)]
    pub fn evouta(&self) -> EVOUTA_R {
        EVOUTA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Event Output D"]
    #[inline(always)]
    pub fn evoutd(&self) -> EVOUTD_R {
        EVOUTD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Output F"]
    #[inline(always)]
    pub fn evoutf(&self) -> EVOUTF_R {
        EVOUTF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Output A"]
    #[inline(always)]
    #[must_use]
    pub fn evouta(&mut self) -> EVOUTA_W<0> {
        EVOUTA_W::new(self)
    }
    #[doc = "Bit 3 - Event Output D"]
    #[inline(always)]
    #[must_use]
    pub fn evoutd(&mut self) -> EVOUTD_W<3> {
        EVOUTD_W::new(self)
    }
    #[doc = "Bit 5 - Event Output F"]
    #[inline(always)]
    #[must_use]
    pub fn evoutf(&mut self) -> EVOUTF_W<5> {
        EVOUTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EVSYS route A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evsysroutea](index.html) module"]
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
