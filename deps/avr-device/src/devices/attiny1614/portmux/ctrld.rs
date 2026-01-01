#[doc = "Register `CTRLD` reader"]
pub struct R(crate::R<CTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLD` writer"]
pub struct W(crate::W<CTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLD_SPEC>;
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
impl From<crate::W<CTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCB0` reader - Port Multiplexer TCB0"]
pub type TCB0_R = crate::BitReader<TCB0_A>;
#[doc = "Port Multiplexer TCB0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB0_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCB0_A> for bool {
    #[inline(always)]
    fn from(variant: TCB0_A) -> Self {
        variant as u8 != 0
    }
}
impl TCB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCB0_A {
        match self.bits {
            false => TCB0_A::DEFAULT,
            true => TCB0_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCB0_A::ALTERNATE
    }
}
#[doc = "Field `TCB0` writer - Port Multiplexer TCB0"]
pub type TCB0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, TCB0_A, O>;
impl<'a, const O: u8> TCB0_W<'a, O> {
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB0_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TCB0_A::ALTERNATE)
    }
}
#[doc = "Field `TCB1` reader - Port Multiplexer TCB1"]
pub type TCB1_R = crate::BitReader<TCB1_A>;
#[doc = "Port Multiplexer TCB1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB1_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCB1_A> for bool {
    #[inline(always)]
    fn from(variant: TCB1_A) -> Self {
        variant as u8 != 0
    }
}
impl TCB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCB1_A {
        match self.bits {
            false => TCB1_A::DEFAULT,
            true => TCB1_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB1_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCB1_A::ALTERNATE
    }
}
#[doc = "Field `TCB1` writer - Port Multiplexer TCB1"]
pub type TCB1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, TCB1_A, O>;
impl<'a, const O: u8> TCB1_W<'a, O> {
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB1_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TCB1_A::ALTERNATE)
    }
}
impl R {
    #[doc = "Bit 0 - Port Multiplexer TCB0"]
    #[inline(always)]
    pub fn tcb0(&self) -> TCB0_R {
        TCB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCB1"]
    #[inline(always)]
    pub fn tcb1(&self) -> TCB1_R {
        TCB1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer TCB0"]
    #[inline(always)]
    #[must_use]
    pub fn tcb0(&mut self) -> TCB0_W<0> {
        TCB0_W::new(self)
    }
    #[doc = "Bit 1 - Port Multiplexer TCB1"]
    #[inline(always)]
    #[must_use]
    pub fn tcb1(&mut self) -> TCB1_W<1> {
        TCB1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexer Control D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrld](index.html) module"]
pub struct CTRLD_SPEC;
impl crate::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrld::R](R) reader structure"]
impl crate::Readable for CTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrld::W](W) writer structure"]
impl crate::Writable for CTRLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
