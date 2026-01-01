#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCA00` reader - Port Multiplexer TCA0 Output 0"]
pub type TCA00_R = crate::BitReader<TCA00_A>;
#[doc = "Port Multiplexer TCA0 Output 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA00_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA00_A> for bool {
    #[inline(always)]
    fn from(variant: TCA00_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCA00_A {
        match self.bits {
            false => TCA00_A::DEFAULT,
            true => TCA00_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA00_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA00_A::ALTERNATE
    }
}
#[doc = "Field `TCA00` writer - Port Multiplexer TCA0 Output 0"]
pub type TCA00_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, TCA00_A, O>;
impl<'a, const O: u8> TCA00_W<'a, O> {
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCA00_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TCA00_A::ALTERNATE)
    }
}
#[doc = "Field `TCA01` reader - Port Multiplexer TCA0 Output 1"]
pub type TCA01_R = crate::BitReader<TCA01_A>;
#[doc = "Port Multiplexer TCA0 Output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA01_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA01_A> for bool {
    #[inline(always)]
    fn from(variant: TCA01_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCA01_A {
        match self.bits {
            false => TCA01_A::DEFAULT,
            true => TCA01_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA01_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA01_A::ALTERNATE
    }
}
#[doc = "Field `TCA01` writer - Port Multiplexer TCA0 Output 1"]
pub type TCA01_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, TCA01_A, O>;
impl<'a, const O: u8> TCA01_W<'a, O> {
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCA01_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TCA01_A::ALTERNATE)
    }
}
#[doc = "Field `TCA02` reader - Port Multiplexer TCA0 Output 2"]
pub type TCA02_R = crate::BitReader<TCA02_A>;
#[doc = "Port Multiplexer TCA0 Output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA02_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA02_A> for bool {
    #[inline(always)]
    fn from(variant: TCA02_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCA02_A {
        match self.bits {
            false => TCA02_A::DEFAULT,
            true => TCA02_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA02_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA02_A::ALTERNATE
    }
}
#[doc = "Field `TCA02` writer - Port Multiplexer TCA0 Output 2"]
pub type TCA02_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, TCA02_A, O>;
impl<'a, const O: u8> TCA02_W<'a, O> {
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCA02_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TCA02_A::ALTERNATE)
    }
}
#[doc = "Field `TCA03` reader - Port Multiplexer TCA0 Output 3"]
pub type TCA03_R = crate::BitReader<TCA03_A>;
#[doc = "Port Multiplexer TCA0 Output 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCA03_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCA03_A> for bool {
    #[inline(always)]
    fn from(variant: TCA03_A) -> Self {
        variant as u8 != 0
    }
}
impl TCA03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCA03_A {
        match self.bits {
            false => TCA03_A::DEFAULT,
            true => TCA03_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCA03_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCA03_A::ALTERNATE
    }
}
#[doc = "Field `TCA03` writer - Port Multiplexer TCA0 Output 3"]
pub type TCA03_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, TCA03_A, O>;
impl<'a, const O: u8> TCA03_W<'a, O> {
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCA03_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TCA03_A::ALTERNATE)
    }
}
impl R {
    #[doc = "Bit 0 - Port Multiplexer TCA0 Output 0"]
    #[inline(always)]
    pub fn tca00(&self) -> TCA00_R {
        TCA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCA0 Output 1"]
    #[inline(always)]
    pub fn tca01(&self) -> TCA01_R {
        TCA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Multiplexer TCA0 Output 2"]
    #[inline(always)]
    pub fn tca02(&self) -> TCA02_R {
        TCA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Multiplexer TCA0 Output 3"]
    #[inline(always)]
    pub fn tca03(&self) -> TCA03_R {
        TCA03_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer TCA0 Output 0"]
    #[inline(always)]
    #[must_use]
    pub fn tca00(&mut self) -> TCA00_W<0> {
        TCA00_W::new(self)
    }
    #[doc = "Bit 1 - Port Multiplexer TCA0 Output 1"]
    #[inline(always)]
    #[must_use]
    pub fn tca01(&mut self) -> TCA01_W<1> {
        TCA01_W::new(self)
    }
    #[doc = "Bit 2 - Port Multiplexer TCA0 Output 2"]
    #[inline(always)]
    #[must_use]
    pub fn tca02(&mut self) -> TCA02_W<2> {
        TCA02_W::new(self)
    }
    #[doc = "Bit 3 - Port Multiplexer TCA0 Output 3"]
    #[inline(always)]
    #[must_use]
    pub fn tca03(&mut self) -> TCA03_W<3> {
        TCA03_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexer Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
