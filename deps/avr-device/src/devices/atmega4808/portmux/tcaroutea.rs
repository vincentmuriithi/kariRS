#[doc = "Register `TCAROUTEA` reader"]
pub struct R(crate::R<TCAROUTEA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCAROUTEA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCAROUTEA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCAROUTEA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCAROUTEA` writer"]
pub struct W(crate::W<TCAROUTEA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCAROUTEA_SPEC>;
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
impl From<crate::W<TCAROUTEA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCAROUTEA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCA0` reader - Port Multiplexer TCA0"]
pub type TCA0_R = crate::FieldReader<u8, TCA0_A>;
#[doc = "Port Multiplexer TCA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCA0_A {
    #[doc = "0: TCA0 pins on PA\\[5:0\\]"]
    PORTA = 0,
    #[doc = "1: TCA0 pins on PB\\[5:0\\]"]
    PORTB = 1,
    #[doc = "2: TCA0 pins on PC\\[5:0\\]"]
    PORTC = 2,
    #[doc = "3: TCA0 pins on PD\\[5:0\\]"]
    PORTD = 3,
    #[doc = "4: TCA0 pins on PE\\[5:0\\]"]
    PORTE = 4,
    #[doc = "5: TCA0 pins on PF\\[5:0\\]"]
    PORTF = 5,
}
impl From<TCA0_A> for u8 {
    #[inline(always)]
    fn from(variant: TCA0_A) -> Self {
        variant as _
    }
}
impl TCA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCA0_A> {
        match self.bits {
            0 => Some(TCA0_A::PORTA),
            1 => Some(TCA0_A::PORTB),
            2 => Some(TCA0_A::PORTC),
            3 => Some(TCA0_A::PORTD),
            4 => Some(TCA0_A::PORTE),
            5 => Some(TCA0_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == TCA0_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == TCA0_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == TCA0_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == TCA0_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == TCA0_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == TCA0_A::PORTF
    }
}
#[doc = "Field `TCA0` writer - Port Multiplexer TCA0"]
pub type TCA0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TCAROUTEA_SPEC, u8, TCA0_A, 3, O>;
impl<'a, const O: u8> TCA0_W<'a, O> {
    #[doc = "TCA0 pins on PA\\[5:0\\]"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(TCA0_A::PORTA)
    }
    #[doc = "TCA0 pins on PB\\[5:0\\]"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(TCA0_A::PORTB)
    }
    #[doc = "TCA0 pins on PC\\[5:0\\]"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(TCA0_A::PORTC)
    }
    #[doc = "TCA0 pins on PD\\[5:0\\]"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(TCA0_A::PORTD)
    }
    #[doc = "TCA0 pins on PE\\[5:0\\]"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(TCA0_A::PORTE)
    }
    #[doc = "TCA0 pins on PF\\[5:0\\]"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(TCA0_A::PORTF)
    }
}
impl R {
    #[doc = "Bits 0:2 - Port Multiplexer TCA0"]
    #[inline(always)]
    pub fn tca0(&self) -> TCA0_R {
        TCA0_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Port Multiplexer TCA0"]
    #[inline(always)]
    #[must_use]
    pub fn tca0(&mut self) -> TCA0_W<0> {
        TCA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexer TCA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcaroutea](index.html) module"]
pub struct TCAROUTEA_SPEC;
impl crate::RegisterSpec for TCAROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcaroutea::R](R) reader structure"]
impl crate::Readable for TCAROUTEA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcaroutea::W](W) writer structure"]
impl crate::Writable for TCAROUTEA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCAROUTEA to value 0"]
impl crate::Resettable for TCAROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
