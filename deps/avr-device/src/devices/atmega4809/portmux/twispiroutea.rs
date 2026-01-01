#[doc = "Register `TWISPIROUTEA` reader"]
pub struct R(crate::R<TWISPIROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWISPIROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWISPIROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWISPIROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWISPIROUTEA` writer"]
pub struct W(crate::W<TWISPIROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWISPIROUTEA_SPEC>;
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
impl From<crate::W<TWISPIROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWISPIROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0` reader - Port Multiplexer SPI0"]
pub type SPI0_R = crate::FieldReader<u8, SPI0_A>;
#[doc = "Port Multiplexer SPI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI0_A {
    #[doc = "0: SPI0 on PA\\[7:4\\]"]
    DEFAULT = 0,
    #[doc = "1: SPI0 on PC\\[3:0\\]"]
    ALT1 = 1,
    #[doc = "2: SPI0 on PE\\[3:0\\]"]
    ALT2 = 2,
    #[doc = "3: Not connected to any pins"]
    NONE = 3,
}
impl From<SPI0_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as _
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            0 => SPI0_A::DEFAULT,
            1 => SPI0_A::ALT1,
            2 => SPI0_A::ALT2,
            3 => SPI0_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == SPI0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == SPI0_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == SPI0_A::ALT2
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SPI0_A::NONE
    }
}
#[doc = "Field `SPI0` writer - Port Multiplexer SPI0"]
pub type SPI0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, TWISPIROUTEA_SPEC, u8, SPI0_A, 2, O>;
impl<'a, const O: u8> SPI0_W<'a, O> {
    #[doc = "SPI0 on PA\\[7:4\\]"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(SPI0_A::DEFAULT)
    }
    #[doc = "SPI0 on PC\\[3:0\\]"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(SPI0_A::ALT1)
    }
    #[doc = "SPI0 on PE\\[3:0\\]"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(SPI0_A::ALT2)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SPI0_A::NONE)
    }
}
#[doc = "Field `TWI0` reader - Port Multiplexer TWI0"]
pub type TWI0_R = crate::FieldReader<u8, TWI0_A>;
#[doc = "Port Multiplexer TWI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWI0_A {
    #[doc = "0: SCL/SDA on PA\\[3:2\\], Slave mode on PC\\[3:2\\]
in dual TWI mode"]
    DEFAULT = 0,
    #[doc = "1: SCL/SDA on PA\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    ALT1 = 1,
    #[doc = "2: SCL/SDA on PC\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    ALT2 = 2,
    #[doc = "3: Not connected to any pins"]
    NONE = 3,
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
    pub fn variant(&self) -> TWI0_A {
        match self.bits {
            0 => TWI0_A::DEFAULT,
            1 => TWI0_A::ALT1,
            2 => TWI0_A::ALT2,
            3 => TWI0_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TWI0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == TWI0_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == TWI0_A::ALT2
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TWI0_A::NONE
    }
}
#[doc = "Field `TWI0` writer - Port Multiplexer TWI0"]
pub type TWI0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, TWISPIROUTEA_SPEC, u8, TWI0_A, 2, O>;
impl<'a, const O: u8> TWI0_W<'a, O> {
    #[doc = "SCL/SDA on PA\\[3:2\\], Slave mode on PC\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TWI0_A::DEFAULT)
    }
    #[doc = "SCL/SDA on PA\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(TWI0_A::ALT1)
    }
    #[doc = "SCL/SDA on PC\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(TWI0_A::ALT2)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TWI0_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port Multiplexer SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Port Multiplexer TWI0"]
    #[inline(always)]
    pub fn twi0(&self) -> TWI0_R {
        TWI0_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port Multiplexer SPI0"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<0> {
        SPI0_W::new(self)
    }
    #[doc = "Bits 4:5 - Port Multiplexer TWI0"]
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
#[doc = "Port Multiplexer TWI and SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twispiroutea](index.html) module"]
pub struct TWISPIROUTEA_SPEC;
impl crate::RegisterSpec for TWISPIROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twispiroutea::R](R) reader structure"]
impl crate::Readable for TWISPIROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twispiroutea::W](W) writer structure"]
impl crate::Writable for TWISPIROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWISPIROUTEA to value 0"]
impl crate::Resettable for TWISPIROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
