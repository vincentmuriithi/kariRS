#[doc = "Register `TWIROUTEA` reader"]
pub struct R(crate::R<TWIROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWIROUTEA` writer"]
pub struct W(crate::W<TWIROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWIROUTEA_SPEC>;
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
impl From<crate::W<TWIROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWIROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWI0` reader - TWI0 Signals"]
pub type TWI0_R = crate::FieldReader<u8, TWI0_A>;
#[doc = "TWI0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWI0_A {
    #[doc = "0: SDA: PA2, SCL: PA3"]
    DEFAULT = 0,
    #[doc = "3: SDA: PA0, SCL: PA1"]
    ALT3 = 3,
}
impl From<TWI0_A> for u8 {
    #[inline(always)]
    fn from(variant: TWI0_A) -> Self {
        variant as _
    }
}
impl TWI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TWI0_A> {
        match self.bits {
            0 => Some(TWI0_A::DEFAULT),
            3 => Some(TWI0_A::ALT3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TWI0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == TWI0_A::ALT3
    }
}
#[doc = "Field `TWI0` writer - TWI0 Signals"]
pub type TWI0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TWIROUTEA_SPEC, u8, TWI0_A, 2, O>;
impl<'a, const O: u8> TWI0_W<'a, O> {
    #[doc = "SDA: PA2, SCL: PA3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TWI0_A::DEFAULT)
    }
    #[doc = "SDA: PA0, SCL: PA1"]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(TWI0_A::ALT3)
    }
}
impl R {
    #[doc = "Bits 0:1 - TWI0 Signals"]
    #[inline(always)]
    pub fn twi0(&self) -> TWI0_R {
        TWI0_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - TWI0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn twi0(&mut self) -> TWI0_W<0> {
        TWI0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI route A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twiroutea](index.html) module"]
pub struct TWIROUTEA_SPEC;
impl crate::RegisterSpec for TWIROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twiroutea::R](R) reader structure"]
impl crate::Readable for TWIROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twiroutea::W](W) writer structure"]
impl crate::Writable for TWIROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWIROUTEA to value 0"]
impl crate::Resettable for TWIROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
