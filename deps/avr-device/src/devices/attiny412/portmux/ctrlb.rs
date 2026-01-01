#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART0` reader - Port Multiplexer USART0"]
pub type USART0_R = crate::BitReader<USART0_A>;
#[doc = "Port Multiplexer USART0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART0_A {
    #[doc = "0: Default pins"]
    DEFAULT = 0,
    #[doc = "1: Alternate pins"]
    ALTERNATE = 1,
}
impl From<USART0_A> for bool {
    #[inline(always)]
    fn from(variant: USART0_A) -> Self {
        variant as u8 != 0
    }
}
impl USART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART0_A {
        match self.bits {
            false => USART0_A::DEFAULT,
            true => USART0_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == USART0_A::ALTERNATE
    }
}
#[doc = "Field `USART0` writer - Port Multiplexer USART0"]
pub type USART0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, USART0_A, O>;
impl<'a, const O: u8> USART0_W<'a, O> {
    #[doc = "Default pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(USART0_A::DEFAULT)
    }
    #[doc = "Alternate pins"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(USART0_A::ALTERNATE)
    }
}
#[doc = "Field `SPI0` reader - Port Multiplexer SPI0"]
pub type SPI0_R = crate::BitReader<SPI0_A>;
#[doc = "Port Multiplexer SPI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI0_A {
    #[doc = "0: Default pins"]
    DEFAULT = 0,
    #[doc = "1: Alternate pins"]
    ALTERNATE = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::DEFAULT,
            true => SPI0_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == SPI0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == SPI0_A::ALTERNATE
    }
}
#[doc = "Field `SPI0` writer - Port Multiplexer SPI0"]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, SPI0_A, O>;
impl<'a, const O: u8> SPI0_W<'a, O> {
    #[doc = "Default pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(SPI0_A::DEFAULT)
    }
    #[doc = "Alternate pins"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(SPI0_A::ALTERNATE)
    }
}
#[doc = "Field `TWI0` reader - Port Multiplexer TWI0"]
pub type TWI0_R = crate::BitReader<TWI0_A>;
#[doc = "Port Multiplexer TWI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWI0_A {
    #[doc = "0: Default pins"]
    DEFAULT = 0,
    #[doc = "1: Alternate pins"]
    ALTERNATE = 1,
}
impl From<TWI0_A> for bool {
    #[inline(always)]
    fn from(variant: TWI0_A) -> Self {
        variant as u8 != 0
    }
}
impl TWI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWI0_A {
        match self.bits {
            false => TWI0_A::DEFAULT,
            true => TWI0_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TWI0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TWI0_A::ALTERNATE
    }
}
#[doc = "Field `TWI0` writer - Port Multiplexer TWI0"]
pub type TWI0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, TWI0_A, O>;
impl<'a, const O: u8> TWI0_W<'a, O> {
    #[doc = "Default pins"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TWI0_A::DEFAULT)
    }
    #[doc = "Alternate pins"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TWI0_A::ALTERNATE)
    }
}
impl R {
    #[doc = "Bit 0 - Port Multiplexer USART0"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Port Multiplexer SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Multiplexer TWI0"]
    #[inline(always)]
    pub fn twi0(&self) -> TWI0_R {
        TWI0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer USART0"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<0> {
        USART0_W::new(self)
    }
    #[doc = "Bit 2 - Port Multiplexer SPI0"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<2> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 4 - Port Multiplexer TWI0"]
    #[inline(always)]
    #[must_use]
    pub fn twi0(&mut self) -> TWI0_W<4> {
        TWI0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexer Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
